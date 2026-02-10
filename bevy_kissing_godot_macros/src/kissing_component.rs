use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{Field, Ident, ItemStruct, Path, parse_macro_input, spanned::Spanned};

use crate::utils::{generate_godot_object_name_for_kissing_component_data, is_field_export};

// ---------
// * Enums *
// ---------

enum FieldAttribute {
	Export { data: ExportData },
	ExportNode { types: Vec<Ident> },
}

// -----------
// * Structs *
// -----------

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

/// If a `#[export]` or `#[export_node]` attribute exists on `field`, it is removed and
/// a `FieldAttribute` containing its data is returned.
///
/// Returns `Ok(None)` if there are no `#[export]` or `#[export_node]` attributes.
///
/// Returns `Err` if there IS a supported attribute, but it's malformed.
fn take_any_export_attribute_if_exists(
	field: &mut Field,
) -> Option<Result<FieldAttribute, syn::Error>> {
	let export_data = take_export_attribute_if_exists(field);
	let export_node_data = take_export_node_attribute_if_exists(field);
	if export_data.is_some() && export_node_data.is_some() {
		return Some(Err(syn::Error::new(
			field.span(),
			"A field cannot have both #[export] and #[export_node] on it",
		)));
	}

	if let Some(export_data) = export_data {
		return Some(export_data.map(|data| FieldAttribute::Export { data }));
	}

	if let Some(export_node_data) = export_node_data {
		return Some(export_node_data.map(|types| FieldAttribute::ExportNode { types }));
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

/// If a `#[export_node(A, B, etc...)]` attribute exists on `field`, it is removed
/// and a `Vec` of its arguments is returned.
///
/// Returns `Some(vec![])` if the attribute has no arguments (`#[export_node]`).
///
/// Returns `None` if there are no `#[export_node]` attributes.
fn take_export_node_attribute_if_exists(
	field: &mut Field,
) -> Option<Result<Vec<Ident>, syn::Error>> {
	let mut i = 0;
	let mut attr = None;
	for a in &field.attrs {
		if a.path().is_ident("export_node") {
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

	if valid_types.is_empty() {
		return Some(Ok(vec![]));
	}

	Some(Ok(valid_types))
}

/// Given a copy of the input `kissing_component` struct, returns the token-stream
/// for the Godot object that replaces it.
fn generate_godot_object_struct(
	original_struct: ItemStruct,
) -> Result<proc_macro2::TokenStream, syn::Error> {
	let mut result = original_struct;

	// Remove attributes, they should be applied on the Component.
	result.attrs.clear();

	// Filter out all fields that aren't `#[export]` or `#[export_node]`.
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
			FieldAttribute::ExportNode { types } => {
				f.ty = syn::parse_quote! { godot::prelude::NodePath };

				if !types.is_empty() {
					let allow_types_string = types
						.iter()
						.map(|n| n.to_string())
						.collect::<Vec<String>>()
						.join(", ");

					f.attrs.push(
						syn::parse_quote! { #[var(hint = NODE_PATH_VALID_TYPES, hint_string = #allow_types_string)] },
					);
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
	let a = quote! {
		#[derive(godot::prelude::GodotClass)]
		#[class(init, base = Object)]
		struct #ident {
			base: godot::prelude::Base<godot::prelude::Object>,
			#(#fields,)*
		}
	};

	Ok(a)
}
