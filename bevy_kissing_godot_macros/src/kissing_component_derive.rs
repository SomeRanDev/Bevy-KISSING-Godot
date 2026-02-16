use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{Attribute, Expr, ItemStruct, Lit, Meta, MetaNameValue, parse_macro_input};

use crate::utils::NodeOrResource;
use crate::utils::generate_godot_object_name_for_kissing_component_data;
use crate::utils::is_field_export;
use crate::utils::is_node_or_resource_id;

/// Returns a `String` that's a combination of all `doc` attributes in the list.
fn get_doc_comment_from_attrs(attrs: &Vec<Attribute>) -> String {
	attrs
		.iter()
		.filter_map(|attr| {
			// Only keep attributes that are `doc = "..."`
			if let Meta::NameValue(MetaNameValue { path, value, .. }) = &attr.meta {
				if path.is_ident("doc") {
					if let Expr::Lit(lit) = &value {
						if let Lit::Str(s) = &lit.lit {
							return Some(s.value());
						}
					}
				}
			}
			None
		})
		.collect::<Vec<String>>()
		.join("\n")
}

/// The implementation for `#[derive(KissingComponent)]`.
pub(super) fn kissing_component_derive_impl(input: TokenStream) -> TokenStream {
	let struct_input = parse_macro_input!(input as ItemStruct);
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
			let (godot_type, id_type, tracker) = match data.kind {
				NodeOrResource::Node => (
					quote! { godot::prelude::NodePath },
					quote! { bevy_kissing_godot::prelude::GodotNodeId },
					quote! { all_nodes },
				),
				NodeOrResource::Resource => (
					quote! { godot::prelude::Gd<godot::prelude::Resource> },
					quote! { bevy_kissing_godot::prelude::GodotResourceId },
					quote! { all_resources },
				),
			};
			if data.is_array {
				let convert = match data.kind {
					NodeOrResource::Node => Some(quote! {
						node
							.get_node_or_null(&node_path)
							.map(|node_path_node|
								#tracker.get_or_register_id_from_node(&node_path_node)
							)
					}),
					NodeOrResource::Resource => Some(quote! {
						Some(#tracker.get_or_register_id_from_node(&node_path))
					}),
				};
				quote! {
					#ident: fields
						.get(stringify!(#ident))
						.and_then(|node_path| {
							node_path
								.try_to::<godot::prelude::Array<#godot_type>>()
								.ok()
								.map(|node_paths|
									node_paths
										.iter_shared()
										.map(|node_path| #convert)
										.filter(|maybe_node_path| maybe_node_path.is_some())
										.map(|node_path| node_path.unwrap())
										.collect::<Vec<#id_type>>()
								)
						})
						.unwrap_or_default()
				}
			} else {
				let convert = match data.kind {
					NodeOrResource::Node => {
						Some(quote! { .and_then(|node_path| node.get_node_or_null(&node_path)) })
					}
					NodeOrResource::Resource => None,
				};
				quote! {
					#ident: fields
						.get(stringify!(#ident))
						.and_then(|node_path| {
							node_path
								.try_to::<#godot_type>()
								.ok()
								#convert
								.map(|node_path_node| #tracker.get_or_register_id_from_node(&node_path_node))
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

	// Add additional static fields and add [kissing_component_data] function to inventory.
	let result = quote! {
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
			pub fn from_editor_fields(
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
				node: &godot::prelude::Gd<godot::prelude::Node>,
				world: &mut bevy::prelude::World,
				entity: &bevy::prelude::Entity,
				fields: std::collections::BTreeMap<String, godot::prelude::Variant>,
			) -> bool {
				let mut system_state: bevy::ecs::system::SystemState<(
					NonSendMut<bevy_kissing_godot::prelude::AllNodes>,
					NonSendMut<bevy_kissing_godot::prelude::AllResources>,
				)> = bevy::ecs::system::SystemState::new(world);
				let (all_nodes, all_resources) = system_state.get_mut(world);
				let c = Self::from_editor_fields(node, &mut all_nodes.into_inner(), &mut all_resources.into_inner(), fields);
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
	};

	result.into()
}
