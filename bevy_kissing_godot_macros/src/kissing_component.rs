use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{Field, Ident, ItemStruct, Path, parse_macro_input, spanned::Spanned};

use crate::utils::{
	NodeOrResource, generate_godot_object_name_for_kissing_component_data, is_field_export,
};

// ---------
// * Enums *
// ---------

enum FieldAttribute {
	Export { data: ExportData },
	ExportNodeOrResource { data: ExportNodeOrResource },
}

// -----------
// * Structs *
// -----------

struct ExportNodeOrResource {
	types: Vec<Ident>,
	kind: NodeOrResource,
}

// ---

struct ExportData {
	initial_value: Option<proc_macro2::TokenStream>,
}

impl Default for ExportData {
	fn default() -> Self {
		Self {
			initial_value: None,
		}
	}
}

// -------------
// * Functions *
// -------------

/// The implementation for `#[kissing_component]`.
pub(super) fn kissing_component_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let struct_input = parse_macro_input!(item as ItemStruct);

	let component_struct = generate_component_struct(struct_input.clone());
	let object_struct = match generate_godot_object_struct(struct_input) {
		Ok(object_struct) => object_struct,
		Err(err) => return err.into_compile_error().into(),
	};

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

/// If a `#[export]` or `#[export_node/resource]` attribute exists on `field`, it is
/// removed and a `FieldAttribute` containing its data is returned.
///
/// Returns `Ok(None)` if there are no `#[export]` or `#[export_node/resource]` attributes.
///
/// Returns `Err` if there IS a supported attribute, but it's malformed.
fn take_any_export_attribute_if_exists(
	field: &mut Field,
) -> Option<Result<FieldAttribute, syn::Error>> {
	let export_data = take_export_attribute_if_exists(field);
	let export_node_or_resource = take_export_node_or_resource_attribute_if_exists(field);
	if export_data.is_some() && export_node_or_resource.is_some() {
		return Some(Err(syn::Error::new(
			field.span(),
			"A field cannot have both #[export] and #[export_node/resource] on it",
		)));
	}

	if let Some(export_data) = export_data {
		return Some(export_data.map(|data| FieldAttribute::Export { data }));
	}

	if let Some(export_node_or_resource) = export_node_or_resource {
		return Some(
			export_node_or_resource.map(|data| FieldAttribute::ExportNodeOrResource { data }),
		);
	}

	None
}

/// If a `#[export(initial_value = X)]` attribute exists on `field`, it is removed
/// and an `ExportData` containing its data is returned.
///
/// Returns `Ok(None)` if there are no `#[export]` attributes.
///
/// Returns `Err` if there IS an `#export` attribute, but it's malformed.
fn take_export_attribute_if_exists(field: &mut Field) -> Option<Result<ExportData, syn::Error>> {
	let mut i = 0;
	let mut attr = None;
	for a in &field.attrs {
		if a.path().is_ident("export") {
			attr = field.attrs.remove(i).into();
			break;
		}
		i += 1;
	}
	let Some(attr) = attr else {
		return None;
	};
	let named_arguments = attr.parse_args_with(
		|input: syn::parse::ParseStream| -> syn::Result<Vec<(Path, proc_macro2::TokenStream)>> {
			let mut result = vec![];
			while !input.is_empty() {
				let path: Path = input.parse()?;
				input.parse::<syn::Token![=]>()?;
				let tokens: proc_macro2::TokenStream = input.parse()?;
				result.push((path, tokens));
				if input.peek(syn::Token![,]) {
					input.parse::<syn::Token![,]>()?;
				}
			}
			Ok(result)
		},
	);

	let named_arguments = match named_arguments {
		Ok(named_arguments) => named_arguments,
		Err(e) => return Some(Err(e)),
	};

	if named_arguments.is_empty() {
		return Some(Ok(Default::default()));
	}

	let mut result = ExportData {
		initial_value: None,
	};
	for a in named_arguments {
		if a.0.is_ident("initial_value") {
			result.initial_value = a.1.into();
		} else {
			let first_span = a.0.span();
			return Some(Err(syn::Error::new(
				first_span.join(a.1.span()).unwrap_or(first_span),
				format!(
					"Unsupported entry name \"{}\" on #[kissing_component]'s #[export]",
					a.0.to_token_stream().to_string()
				),
			)));
		}
	}

	Some(Ok(result))
}

/// If an `#[export_node(A, B, etc...)]` or `#[export_resource(A, B, etc...)]` attribute
/// exists on `field`, it is removed and a `Vec` of its arguments is returned.
///
/// Returns `Some(vec![])` if the attribute has no arguments (`#[export_node]`).
///
/// Returns `None` if there are no `#[export_node]` attributes.
fn take_export_node_or_resource_attribute_if_exists(
	field: &mut Field,
) -> Option<Result<ExportNodeOrResource, syn::Error>> {
	let mut i = 0;
	let mut attr = None;
	for a in &field.attrs {
		if a.path().is_ident("export_node") || a.path().is_ident("export_resource") {
			attr = field.attrs.remove(i).into();
			break;
		}
		i += 1;
	}

	let Some(attr) = attr else {
		return None;
	};

	let valid_types = attr.parse_args_with(
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
	);

	let valid_types = match valid_types {
		Ok(valid_types) => valid_types,
		Err(e) => return Some(Err(e)),
	};

	Some(Ok(ExportNodeOrResource {
		types: valid_types,
		kind: match attr.path().get_ident() {
			Some(v) if v == "export_node" => NodeOrResource::Node,
			_ => NodeOrResource::Resource,
		},
	}))
}

/// Given a copy of the input `kissing_component` struct, returns the token-stream
/// for the Godot object that replaces it.
fn generate_godot_object_struct(
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

		let export_data = match export_data {
			Ok(data) => data,
			Err(err) => return Err(err),
		};

		f.attrs.push(syn::parse_quote! { #[export] });

		match export_data {
			// Add `#[init(val = TOKENS)]` for `#[export(initial_value = TOKENS)]`.
			FieldAttribute::Export { data } => {
				if let Some(initial_value) = data.initial_value {
					f.attrs
						.push(syn::parse_quote! { #[init(val = #initial_value)] });
				}
			}

			// Add `#[var(hint = NODE_PATH_VALID_TYPES, hint_string = "A,B,C")]` for `#[export_nodes(A, B, C)]`.
			// Or add `#[var(hint = RESOURCE_TYPE, hint_string = "A,B,C")]` for `#[export_resources(A, B, C)]`.
			FieldAttribute::ExportNodeOrResource { data } => {
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
					data
						.types
						.iter()
						.map(|n| n.to_string())
						.collect::<Vec<String>>()
						.join(", ")
				} else {
					match data.kind {
						NodeOrResource::Node => "Node",
						NodeOrResource::Resource => "Resource",
					}.to_string()
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
