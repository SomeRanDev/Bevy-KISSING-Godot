use crate::kissing_component::KissingComponentArguments;
use crate::utils::{
	NodeOrResource, generate_godot_object_name_for_kissing_component_data,
	get_doc_comment_from_attrs, is_field_export, is_node_or_resource_id,
};

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};
use syn::ItemStruct;

/// Generates the component's impl providing functions and metadata to allow
/// for a Bevy component to be accessible from the Godot editor.
pub(super) fn generate_component_impl(
	struct_input: ItemStruct,
	args: KissingComponentArguments,
) -> TokenStream2 {
	let ident = struct_input.ident;

	let component_docs = get_doc_comment_from_attrs(&struct_input.attrs);
	let component_docs = component_docs.trim();

	// Field names
	let field_names = struct_input.fields.iter().filter_map(|f| {
		let Some(n) = f.ident.as_ref().map(|i| i.to_string()) else {
			return None;
		};
		let n = n.as_str();

		let ty = &f.ty;
		let type_string = ty.to_token_stream().to_string();
		let type_string = type_string.as_str();

		let docs = get_doc_comment_from_attrs(&f.attrs);

		let docs = if docs.is_empty() {
			quote!(None)
		} else {
			quote!(Some(#docs))
		};

		Some(
			quote!(bevy_kissing_godot::kissing_component::kissing_component_data::KissingComponentFieldData {
				name: #n,
				type_string: #type_string,
				description: #docs,
			}),
		)
	});

	// Generate field assignments used in generated [from_editor_fields].
	let field_inputs = struct_input.fields.iter().map(|f| {
		let Some(ident) = &f.ident else {
			return quote! {
				#ident: Default::default()
			};
		};
		let ty = &f.ty;
		if let Some(data) = is_node_or_resource_id(ty) {
			let (identifier, godot_type, id_type, tracker) = match data.kind {
				NodeOrResource::Node => (
					format_ident!("node_path"),
					quote! { godot::prelude::NodePath },
					quote! { bevy_kissing_godot::prelude::GodotNodeId },
					quote! { all_nodes },
				),
				NodeOrResource::Resource => (
					format_ident!("resource"),
					quote! { godot::prelude::Gd<godot::prelude::Resource> },
					quote! { bevy_kissing_godot::prelude::GodotResourceId },
					quote! { all_resources },
				),
			};
			if data.is_array {
				let convert = match data.kind {
					NodeOrResource::Node => Some(quote! {
						node
							.get_node_or_null(&#identifier)
							.map(|node_path_node|
								#tracker.get_or_register_id_from_gd_object(&node_path_node)
							)
					}),
					NodeOrResource::Resource => Some(quote! {
						Some(#tracker.get_or_register_id_from_gd_object(&#identifier))
					}),
				};
				let identifier_plural = format_ident!("{}s", identifier);
				let identifier_maybe = format_ident!("maybe_{}", identifier);
				quote! {
					#ident: fields
						.get(stringify!(#ident))
						.and_then(|#identifier| {
							#identifier
								.try_to::<godot::prelude::Array<#godot_type>>()
								.ok()
								.map(|#identifier_plural|
									#identifier_plural
										.iter_shared()
										.map(|#identifier| #convert)
										.filter(|#identifier_maybe| #identifier_maybe.is_some())
										.map(|#identifier| #identifier.unwrap())
										.collect::<Vec<#id_type>>()
								)
						})
						.unwrap_or_default()
				}
			} else {
				let convert = match data.kind {
					NodeOrResource::Node => Some(
						quote! { .and_then(|#identifier| node.get_node_or_null(&#identifier)) },
					),
					NodeOrResource::Resource => None,
				};
				quote! {
					#ident: fields
						.get(stringify!(#ident))
						.and_then(|#identifier| {
							#identifier
								.try_to::<#godot_type>()
								.ok()
								#convert
								.map(|#identifier| #tracker.get_or_register_id_from_gd_object(&#identifier))
						})
				}
			}
		} else if is_field_export(&f) {
			let get_error_string = format!("could not get field of name {}", ident.to_string());
			let to_error_string = format!(
				"could not type field of name {} as {:?}",
				ident.to_string(),
				ty
			);
			quote! {
				#ident: fields
					.get(stringify!(#ident))
					.expect(#get_error_string)
					.try_to::<#ty>()
					.expect(#to_error_string)
			}
		} else {
			quote! {
				#ident: Default::default()
			}
		}
	});

	// Get the name of the Godot class used by the editor to obtain the component data.
	let data_class_name = generate_godot_object_name_for_kissing_component_data(&ident);

	// Get tokens for what happens after the construction of the component `c`.
	let on_construct = if let Some(on_construct) = args.on_construct {
		quote! { #on_construct(&mut c); }
	} else {
		quote! {}
	};

	// Get tokens for what happens upon adding the component `c` to a node.
	let on_added_to_node = if let Some(on_added_to_node) = args.on_added_to_node {
		quote! { #on_added_to_node(node, &mut c, entity, world); }
	} else {
		quote! {}
	};

	// Add additional static fields and add [kissing_component_data] function to inventory.
	quote! {
		impl #ident {
			/// Returns the component's data to be used to generate the Godot editor UI.
			fn kissing_component_data() -> bevy_kissing_godot::kissing_component::kissing_component_data::KissingComponentData {
				bevy_kissing_godot::kissing_component::kissing_component_data::KissingComponentData {
					name: stringify!(#ident),
					data_class_name: #data_class_name,
					docs: #component_docs,
					fields: vec!(#(#field_names),*),
				}
			}

			/// Generates the component given a map of strings provided from the Godot editor UI.
			fn from_editor_fields(
				node: &godot::prelude::Gd<godot::prelude::Node>,
				all_nodes: &mut bevy_kissing_godot::prelude::AllNodes,
				all_resources: &mut bevy_kissing_godot::prelude::AllResources,
				fields: std::collections::BTreeMap<String, godot::prelude::Variant>
			) -> Self {
				Self {
					#(#field_inputs),*
				}
			}

			/// Adds the component to [entity] given its fields as a map from the Godot editor UI.
			pub fn add_component_from_editor_fields(
				node: &mut godot::prelude::Gd<godot::prelude::Node>,
				world: &mut bevy::prelude::World,
				entity: &bevy::prelude::Entity,
				fields: std::collections::BTreeMap<String, godot::prelude::Variant>,
			) -> bool {
				let mut system_state: bevy::ecs::system::SystemState<(
					NonSendMut<bevy_kissing_godot::prelude::AllNodes>,
					NonSendMut<bevy_kissing_godot::prelude::AllResources>,
				)> = bevy::ecs::system::SystemState::new(world);
				let (all_nodes, all_resources) = system_state.get_mut(world);
				let mut c = Self::from_editor_fields(node, &mut all_nodes.into_inner(), &mut all_resources.into_inner(), fields);
				#on_construct
				#on_added_to_node
				let Ok(mut e) = world.get_entity_mut(*entity) else { return false };
				e.insert(c);
				true
			}
		}

		bevy_kissing_godot::prelude::bevy_kissing_godot_inventory::submit! {
			bevy_kissing_godot::kissing_component::kissing_component::KissingComponent::new(
				#ident::kissing_component_data,
				#ident::add_component_from_editor_fields,
			)
		}
	}
}
