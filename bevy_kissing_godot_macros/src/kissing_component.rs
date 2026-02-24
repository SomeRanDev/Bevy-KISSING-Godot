use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};

// -----------
// * Modules *
// -----------

mod generate_component_impl;
mod generate_component_struct;
mod generate_godot_object_struct;

// -------------
// * Functions *
// -------------

/// The implementation for `#[kissing_component]`.
pub(super) fn kissing_component_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let struct_input = parse_macro_input!(item as ItemStruct);

	let component_struct =
		generate_component_struct::generate_component_struct(struct_input.clone());

	let component_struct_impl =
		generate_component_impl::generate_component_impl(struct_input.clone());

	let object_struct =
		match generate_godot_object_struct::generate_godot_object_struct(struct_input) {
			Ok(object_struct) => object_struct,
			Err(err) => return err.into_compile_error().into(),
		};

	let result = quote! {
		#component_struct
		#component_struct_impl
		#object_struct
	};

	result.into()
}
