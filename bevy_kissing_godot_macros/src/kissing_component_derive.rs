use proc_macro::TokenStream;
use quote::ToTokens;
use quote::quote;
use syn::{Attribute, ItemStruct, Lit, Meta, MetaNameValue, parse_macro_input};

use crate::utils::generate_godot_object_name_for_kissing_component_data;
use crate::utils::is_option_godot_node_id;

/// Returns a `String` that's a combination of all `doc` attributes in the list.
fn get_doc_comment_from_attrs(attrs: &Vec<Attribute>) -> String {
	attrs
		.iter()
		.filter_map(|attr| {
			// Only keep attributes that are `doc = "..."`
			if let Ok(Meta::NameValue(MetaNameValue { path, lit, .. })) = attr.parse_meta() {
				if path.is_ident("doc") {
					if let Lit::Str(s) = &lit {
						return Some(s.value());
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
		let ident = &f.ident;
		let ty = &f.ty;
		if is_option_godot_node_id(ty) {
			quote! {
				// Get `Option<bevy_kissing_godot::prelude::GodotNodeId>` from `godot::prelude::NodePath`
				#ident: fields
					.get(stringify!(#ident))
					.and_then(|node_path| {
						node_path
							.try_to::<godot::prelude::NodePath>()
							.ok()
							.and_then(|node_path| node.get_node_or_null(&node_path))
							.map(|node_path_node| all_nodes.get_id_from_node(&node_path_node))
					})
					.unwrap()
			}
		} else {
			quote! {
				#ident: fields
					.get(stringify!(#ident))
					.map(|v| v.to::<#ty>())
					.unwrap()
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
				all_nodes: &bevy_kissing_godot::prelude::AllNodes,
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
				let Some(all_nodes) = world.get_non_send_resource::<AllNodes>() else {
					return false;
				};
				let c = Self::from_editor_fields(node, all_nodes, fields);
				drop(all_nodes);

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
