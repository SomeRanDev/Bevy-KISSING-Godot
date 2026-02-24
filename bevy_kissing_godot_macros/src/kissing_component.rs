use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
	ItemStruct, MetaNameValue, parse::Parser, parse_macro_input, punctuated::Punctuated,
	spanned::Spanned, token::Comma,
};

// -----------
// * Modules *
// -----------

mod generate_component_impl;
mod generate_component_struct;
mod generate_godot_object_struct;

// -----------
// * Structs *
// -----------

/// A representation of the arguments passed to `#[kissing_component]`.
struct KissingComponentArguments {
	on_construct: Option<proc_macro2::TokenStream>,
}

impl KissingComponentArguments {
	fn from_attr_token_stream(attr: TokenStream) -> syn::Result<Self> {
		let args: Punctuated<MetaNameValue, Comma> = match Parser::parse(
			Punctuated::<MetaNameValue, syn::Token![,]>::parse_terminated,
			attr,
		) {
			Ok(data) => data,
			Err(err) => return Err(err),
		};

		let mut result = Self { on_construct: None };
		for arg in args {
			if arg.path.is_ident("on_construct") {
				result.on_construct = Some(arg.value.into_token_stream());
			} else {
				return Err(syn::Error::new(
					arg.span(),
					"unknown argument for #[kissing_component]",
				));
			}
		}

		Ok(result)
	}
}

// -------------
// * Functions *
// -------------

/// The implementation for `#[kissing_component]`.
pub(super) fn kissing_component_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
	let struct_input = parse_macro_input!(item as ItemStruct);

	let component_struct =
		generate_component_struct::generate_component_struct(struct_input.clone());

	let args = match KissingComponentArguments::from_attr_token_stream(attr) {
		Ok(args) => args,
		Err(err) => return err.into_compile_error().into(),
	};
	let component_struct_impl =
		generate_component_impl::generate_component_impl(struct_input.clone(), args);

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
