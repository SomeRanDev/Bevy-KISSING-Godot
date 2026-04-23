use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{Attribute, Field, Ident, ItemStruct, Meta, Path, spanned::Spanned};

use crate::utils::{
	NodeOrResource, generate_godot_object_name_for_kissing_component_data, is_field_export,
};

// ---------
// * Enums *
// ---------

enum FieldAttribute {
	Export {
		export_attribute: syn::Attribute,
		initial_value: Option<proc_macro2::TokenStream>,
	},
	ExportNodeOrResource {
		data: ExportNodeOrResource,
	},
	ExportString {
		initial_value: Option<proc_macro2::TokenStream>,
	},
}

// -----------
// * Structs *
// -----------

struct ExportNodeOrResource {
	types: Vec<Ident>,
	kind: NodeOrResource,
}

// -------------
// * Functions *
// -------------

/// Given a copy of the input `kissing_component` struct, returns the token-stream
/// for the Godot object that replaces it.
pub(super) fn generate_godot_object_struct(
	original_struct: ItemStruct,
) -> Result<proc_macro2::TokenStream, syn::Error> {
	let mut result = original_struct;

	// Remove attributes, they should be applied on the Component.
	result.attrs.clear();

	// Filter out all fields that aren't `#[export]`, `#[export_node]`, or `#[export_resource]`.
	let mut fields = result
		.fields
		.iter()
		.filter(|f| is_field_export(f))
		.map(|f| f.clone())
		.collect::<Vec<syn::Field>>();

	// Update field types for Godot Object.
	for f in &mut fields {
		let Some(export_data) = take_any_export_attribute_if_exists(f) else {
			continue;
		};

		let (field_attribute, new_attributes) = export_data?;

		f.attrs = new_attributes;

		match field_attribute {
			FieldAttribute::Export {
				export_attribute,
				initial_value,
			} => {
				// Transfer `#[export]` attribute.
				f.attrs.push(export_attribute);

				// Add `#[init(val = TOKENS)]` for `#[initial_value = TOKENS]`.
				if let Some(initial_value) = initial_value {
					f.attrs
						.push(syn::parse_quote! { #[init(val = #initial_value)] });
				}
			}

			// Add `#[var(hint = NODE_PATH_VALID_TYPES, hint_string = "A,B,C")]` for `#[export_nodes(A, B, C)]`.
			// Or add `#[var(hint = RESOURCE_TYPE, hint_string = "A,B,C")]` for `#[export_resources(A, B, C)]`.
			FieldAttribute::ExportNodeOrResource { data } => {
				f.attrs.push(syn::parse_quote! { #[export] });

				// Set the exported type to NodePath for nodes and Gd<Resource> for resources.
				let original_type_is_vec = is_vec(&f.ty);
				f.ty = match data.kind {
					NodeOrResource::Node => syn::parse_quote! { godot::prelude::NodePath },
					NodeOrResource::Resource => {
						syn::parse_quote! { godot::prelude::Gd<godot::prelude::Resource> }
					}
				};

				// If the original type is a Vec, wrap our new type with Array.
				// Otherwise, if it's a resource, wrap with Option.
				{
					let t = &f.ty;
					if original_type_is_vec {
						f.ty = syn::parse_quote! { godot::prelude::Array<#t> };
					} else if data.kind == NodeOrResource::Resource {
						f.ty = syn::parse_quote! { Option<#t> };
					}
				}

				// If filtering for specific types, get them as a hint string here.
				let allow_types_string = if !data.types.is_empty() {
					data.types
						.iter()
						.map(|n| n.to_string())
						.collect::<Vec<String>>()
						.join(", ")
				} else {
					match data.kind {
						NodeOrResource::Node => "Node",
						NodeOrResource::Resource => "Resource",
					}
					.to_string()
				};

				// Generate the #[var] attribute.
				f.attrs.push(match data.kind {
					NodeOrResource::Node => {
						syn::parse_quote! { #[var(hint = NODE_PATH_VALID_TYPES, hint_string = #allow_types_string)] }
					}
					NodeOrResource::Resource => {
						syn::parse_quote! { #[var(hint = RESOURCE_TYPE, hint_string = #allow_types_string)] }
					}
				});
			}

			FieldAttribute::ExportString { initial_value } => {
				f.attrs.push(syn::parse_quote! { #[export] });

				f.ty = syn::parse_quote! { godot::prelude::GString };

				if let Some(initial_value) = initial_value {
					f.attrs
						.push(syn::parse_quote! { #[init(val = #initial_value)] });
				}
			}
		}
	}

	// Append "Object" to struct identifier.
	result.ident = Ident::new(
		&generate_godot_object_name_for_kissing_component_data(&result.ident),
		Span::call_site(),
	);

	let ident = result.ident;
	let new_declaration = quote! {
		#[derive(godot::prelude::GodotClass)]
		#[class(init, base = Object)]
		struct #ident {
			base: godot::prelude::Base<godot::prelude::Object>,
			#(#fields,)*
		}
	};

	Ok(new_declaration)
}

/// If a `#[export]` or `#[export_node/resource]` attribute exists on `field`, it is
/// removed and a `FieldAttribute` containing its data is returned.
///
/// Returns `Ok(None)` if there are no `#[export]` or `#[export_node/resource]` attributes.
///
/// Returns `Err` if there IS a supported attribute, but it's malformed.
fn take_any_export_attribute_if_exists(
	field: &Field,
) -> Option<Result<(FieldAttribute, Vec<Attribute>), syn::Error>> {
	let export_attribute_result = take_export_attribute_if_exists(field);
	let export_node_or_resource = take_export_node_or_resource_attribute_if_exists(field);
	if export_attribute_result.is_some() && export_node_or_resource.is_some() {
		return Some(Err(syn::Error::new(
			field.span(),
			"A field cannot have both #[export] and #[export_node/resource] on it",
		)));
	}

	if let Some(export_attribute_result) = export_attribute_result {
		return Some(export_attribute_result.map(|export_attribute_result| {
			let TakeExportAttributeIfExistsResult {
				export_attribute,
				initial_value,
				new_attributes,
			} = export_attribute_result;
			(
				if export_attribute.path().is_ident("export_string") {
					FieldAttribute::ExportString { initial_value }
				} else {
					FieldAttribute::Export {
						export_attribute,
						initial_value,
					}
				},
				new_attributes,
			)
		}));
	}

	if let Some(export_node_or_resource) = export_node_or_resource {
		return Some(export_node_or_resource.map(|export_node_or_resource| {
			let TakeExportNodeOrResourceAttributeIfExistsResult {
				export_node_or_resource,
				new_attributes,
			} = export_node_or_resource;
			(
				FieldAttribute::ExportNodeOrResource {
					data: export_node_or_resource,
				},
				new_attributes,
			)
		}));
	}

	None
}

/// The result from `take_export_attribute_if_exists`.
struct TakeExportAttributeIfExistsResult {
	export_attribute: syn::Attribute,
	initial_value: Option<proc_macro2::TokenStream>,
	new_attributes: Vec<Attribute>,
}

/// If a `#[export]` or `#[export_string]` attribute exists on `field`, it is removed
/// and returned verbatim.
///
/// Returns `None` if there are no `#[export]` attributes.
fn take_export_attribute_if_exists(
	field: &Field,
) -> Option<Result<TakeExportAttributeIfExistsResult, syn::Error>> {
	let mut i = 0;
	let mut export_attribute = None;
	let mut initial_value_attr = None;
	let mut attrs = field.attrs.clone();
	while i < attrs.len() {
		let attr = &attrs[i];
		if export_attribute.is_none() && attr.path().is_ident("export")
			|| attr.path().is_ident("export_string")
		{
			export_attribute = Some(attrs.remove(i));
			continue; // do not increment i
		} else if initial_value_attr.is_none() && attr.path().is_ident("initial_value") {
			initial_value_attr = Some(attrs.remove(i));
			continue; // do not increment i
		} else if export_attribute.is_none() {
			// Remove all attributes prior to #[export]
			attrs.remove(i);
			continue;
		}
		i += 1;
	}

	// If no `#[export]` attribute, return `None` entirely.
	let Some(export_attribute) = export_attribute else {
		return None;
	};

	// Extract TOKEN_STREAM from `#[initial_value = TOKEN_STREAM]` or `#[initial_value(TOKEN_STREAM)]`.
	let mut initial_value_token_stream = None;
	if let Some(initial_value_attr) = initial_value_attr {
		match initial_value_attr.meta {
			Meta::Path(_) => {
				initial_value_token_stream = Some(quote!(Default::default()));
			}
			Meta::List(meta_list) => {
				initial_value_token_stream = Some(meta_list.tokens);
			}
			Meta::NameValue(meta_name_value) => {
				initial_value_token_stream = Some(meta_name_value.value.into_token_stream());
			}
		}
	}

	Some(Ok(TakeExportAttributeIfExistsResult {
		export_attribute,
		initial_value: initial_value_token_stream,
		new_attributes: attrs,
	}))
}

/// The result from `take_export_node_or_resource_attribute_if_exists`.
struct TakeExportNodeOrResourceAttributeIfExistsResult {
	export_node_or_resource: ExportNodeOrResource,
	new_attributes: Vec<Attribute>,
}

/// If an `#[export_node(A, B, etc...)]` or `#[export_resource(A, B, etc...)]` attribute
/// exists on `field`, it is removed and a `Vec` of its arguments is returned.
///
/// Returns `Some(vec![])` if the attribute has no arguments (`#[export_node]`).
///
/// Returns `None` if there are no `#[export_node]` attributes.
fn take_export_node_or_resource_attribute_if_exists(
	field: &Field,
) -> Option<Result<TakeExportNodeOrResourceAttributeIfExistsResult, syn::Error>> {
	let mut i = 0;
	let mut export_attribute = None;
	let mut attrs = field.attrs.clone();
	while i < attrs.len() {
		let attr = &attrs[i];
		if attr.path().is_ident("export_node") || attr.path().is_ident("export_resource") {
			export_attribute = Some(attrs.remove(i));
			break;
		} else if export_attribute.is_none() {
			// Remove all attributes prior to #[export_node/resource]
			attrs.remove(i);
			continue;
		}
		i += 1;
	}

	let export_attribute = export_attribute?;

	let valid_types = if !matches!(export_attribute.meta, Meta::List(_)) {
		Ok(vec![])
	} else {
		export_attribute.parse_args_with(
			|input: syn::parse::ParseStream| -> syn::Result<Vec<Ident>> {
				let mut result = vec![];
				while !input.is_empty() {
					let path: Path = input.parse()?;
					let Some(ident) = path.get_ident() else {
						return Err(syn::Error::new(path.span(), "Must be an identifier"));
					};
					result.push(ident.clone());
					if input.peek(syn::Token![,]) {
						input.parse::<syn::Token![,]>()?;
					}
				}
				Ok(result)
			},
		)
	};

	let valid_types = match valid_types {
		Ok(valid_types) => valid_types,
		Err(e) => return Some(Err(e)),
	};

	Some(Ok(TakeExportNodeOrResourceAttributeIfExistsResult {
		export_node_or_resource: ExportNodeOrResource {
			types: valid_types,
			kind: match export_attribute.path().get_ident() {
				Some(v) if v == "export_node" => NodeOrResource::Node,
				_ => NodeOrResource::Resource,
			},
		},
		new_attributes: attrs,
	}))
}

fn is_vec(ty: &syn::Type) -> bool {
	let syn::Type::Path(syn::TypePath { path, .. }) = ty else {
		return false;
	};

	let Some(segment) = path.segments.last() else {
		return false;
	};

	if segment.ident != "Vec" {
		return false;
	}

	let syn::PathArguments::AngleBracketed(_) = segment.arguments else {
		return false;
	};

	true
}
