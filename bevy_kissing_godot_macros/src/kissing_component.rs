use std::sync::LazyLock;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Attribute, Field, Ident, ItemStruct, Meta, NestedMeta, Path, Type, parse_macro_input};

use crate::utils::generate_godot_object_name_for_kissing_component_data;

// --------------------
// * Reusable Globals *
// --------------------

thread_local! {
	/// Stores a reusable reference to a `syn::Type` that represents `godot::prelude::NodePath`.
	static NODE_PATH_TYPE: LazyLock<Type> =
		LazyLock::new(|| syn::parse2::<Type>(quote!(godot::prelude::NodePath)).unwrap());

	/// Stores a reusable reference to a `syn::Attribute` that represents `#[export]`.
	static EXPORT_ATTRIBUTE: LazyLock<Attribute> = LazyLock::new(|| {
		let path: Path = syn::parse2(quote!(export)).unwrap();
		Attribute {
			pound_token: Default::default(),
			style: syn::AttrStyle::Outer,
			bracket_token: Default::default(),
			path,
			tokens: quote!(),
		}
	});

	/// Stores a reusable reference to a `syn::Attribute` that represents `#[var]`.
	static VAR_ATTRIBUTE: LazyLock<Attribute> = LazyLock::new(|| {
		let path: Path = syn::parse_str("var").unwrap();

		Attribute {
			pound_token: Default::default(),
			style: syn::AttrStyle::Outer,
			bracket_token: Default::default(),
			path,
			tokens: quote::quote!(),
		}
	});
}

// -------------
// * Functions *
// -------------

/// The implementation for `#[kissing_component]`.
pub(super) fn kissing_component_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let struct_input = parse_macro_input!(item as ItemStruct);

	let component_struct = generate_component_struct(struct_input.clone());
	let object_struct = generate_godot_object_struct(struct_input);

	let result = quote! {
		#component_struct
		#object_struct
	};

	result.into()
}

/// Given a copy of the input `kissing_component` struct, returns the token-stream
/// for the Bevy component that replaces it.
fn generate_component_struct(original_struct: ItemStruct) -> proc_macro2::TokenStream {
	let mut result = original_struct;

	// Update field types for Bevy component.
	for f in &mut result.fields {
		f.attrs.clear();

		match &f.ty {
			syn::Type::Tuple(tuple_type) => {
				if let Some(ty) = tuple_type.elems.iter().nth(1) {
					f.ty = ty.clone();
				}
			}
			_ => (),
		}
	}

	quote! {
		#result
	}
}

/// If a `#[godot_node(A, B, etc...)]` attribute exists on `field`, it is removed
/// and a `Vec` of its arguments is returned.
///
/// Returns `Some(vec![])` if the attribute has no arguments (`#[godot_node]`).
///
/// Returns `None` if there are no `#[godot_node]` attributes.
fn take_godot_node_attribute_if_exists(field: &mut Field) -> Option<Vec<Ident>> {
	let mut i = 0;
	let mut attr = None;
	for a in &field.attrs {
		if a.path.is_ident("export_node") {
			attr = field.attrs.remove(i).into();
			break;
		}
		i += 1;
	}
	let attr = attr?;
	match attr.parse_meta() {
		Ok(Meta::Path(_)) => Some(vec![]),
		Ok(Meta::List(meta_list)) => {
			let mut result = Vec::new();
			for nested in meta_list.nested {
				if let NestedMeta::Meta(Meta::Path(path)) = nested {
					if let Some(ident) = path.get_ident() {
						result.push(ident.clone());
					}
				}
			}
			Some(result)
		}
		_ => None,
	}
}

/// Given a copy of the input `kissing_component` struct, returns the token-stream
/// for the Godot object that replaces it.
fn generate_godot_object_struct(original_struct: ItemStruct) -> proc_macro2::TokenStream {
	let mut result = original_struct;

	// Remove attributes, they should be applied on the Component.
	result.attrs.clear();

	// Update field types for Godot Object.
	for f in &mut result.fields {
		if let Some(allowed_nodes) = take_godot_node_attribute_if_exists(f) {
			NODE_PATH_TYPE.with(|node_path_type| {
				f.ty = (*node_path_type).clone();
			});

			EXPORT_ATTRIBUTE.with(|export_attribute| {
				f.attrs.push((*export_attribute).clone());
			});

			if !allowed_nodes.is_empty() {
				let allow_types_string = allowed_nodes
					.iter()
					.map(|n| n.to_string())
					.collect::<Vec<String>>()
					.join(", ");

				VAR_ATTRIBUTE.with(|var_attribute| {
					let mut var_attribute = (*var_attribute).clone();
					var_attribute.tokens = quote::quote! {(hint = NODE_PATH_VALID_TYPES, hint_string = #allow_types_string)};
					f.attrs.push(var_attribute);
				});
			}
		}
	}

	// Append "Object" to struct identifier.
	result.ident = Ident::new(
		&generate_godot_object_name_for_kissing_component_data(&result.ident),
		Span::call_site(),
	);

	let ident = result.ident;
	let fields_without_brackets = match result.fields {
		syn::Fields::Named(named) => Some(named.named),
		_ => None,
	};
	quote! {
		#[derive(godot::prelude::GodotClass)]
		#[class(init, base = Object)]
		struct #ident {
			base: godot::prelude::Base<godot::prelude::Object>,
			#fields_without_brackets
		}
	}
}
