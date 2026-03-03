use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, ItemStruct, TypePath, parse_macro_input};

// -----------
// * Modules *
// -----------

mod generate_component_impl;
mod generate_godot_object_struct;

// -----------
// * Structs *
// -----------

/// A representation of the arguments passed to `#[kissing_component]`.
#[derive(Default)]
struct KissingComponentArguments {
	on_construct: Option<TypePath>,
	on_added_to_node: Option<TypePath>,
}

impl KissingComponentArguments {
	fn from_attr_token_stream(attr: &Attribute) -> syn::Result<Self> {
		let mut result = Self {
			on_construct: None,
			on_added_to_node: None,
		};

		attr.parse_nested_meta(|meta| {
			if meta.path.is_ident("on_construct") {
				result.on_construct = Some(meta.value()?.parse::<TypePath>()?);
			} else if meta.path.is_ident("on_added_to_node") {
				result.on_added_to_node = Some(meta.value()?.parse::<TypePath>()?);
			} else {
				return Err(meta.error("unknown argument for #[kissing_component]"));
			}
			Ok(())
		})?;

		Ok(result)
	}
}

// -------------
// * Functions *
// -------------

/// The implementation for `#[derive(KissingComponent)]`.
pub(super) fn kissing_component_derive_impl(input: TokenStream) -> TokenStream {
	let struct_input = parse_macro_input!(input as ItemStruct);

	// Find `#[kissing_component]` if it exists
	let mut arguments_attribute = None;
	for attr in &struct_input.attrs {
		if attr.path().is_ident("kissing_component") {
			arguments_attribute = Some(attr);
			break;
		}
	}

	// Parse arguments from `#[kissing_component]`
	let args = if let Some(arguments_attribute) = arguments_attribute {
		match KissingComponentArguments::from_attr_token_stream(arguments_attribute) {
			Ok(args) => args,
			Err(err) => return err.into_compile_error().into(),
		}
	} else {
		KissingComponentArguments::default()
	};

	// Generate `impl` for struct this derive is on
	let component_struct_impl =
		generate_component_impl::generate_component_impl(struct_input.clone(), args);

	// Generate Godot object struct for this component
	let object_struct =
		match generate_godot_object_struct::generate_godot_object_struct(struct_input) {
			Ok(object_struct) => object_struct,
			Err(err) => return err.into_compile_error().into(),
		};

	// Final output
	let result = quote! {
		#component_struct_impl
		#object_struct
	};

	result.into()
}
