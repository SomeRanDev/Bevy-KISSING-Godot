use crate::arguments::node_identifier_argument::NodeIdentifierArgument;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Ident, ItemFn, parse_macro_input};

pub(crate) fn plugin_and_kissing_component_impl(
	attr: TokenStream,
	item: TokenStream,
) -> TokenStream {
	let input_arg = syn::parse_macro_input!(attr as NodeIdentifierArgument);
	let input_fn = parse_macro_input!(item as ItemFn);
	let input_fn_block = input_fn.block;
	let input_fn_vis = input_fn.vis;

	let identifier_string = input_arg.get_ident().to_string();
	let component_identifier = Ident::new(
		&format!("{}Component", identifier_string),
		Span::call_site(),
	);
	let plugin_identifier = Ident::new(&format!("{}Plugin", identifier_string), Span::call_site());

	let result = quote! {
		#[derive(bevy::prelude::Component, bevy_kissing_godot::prelude::KissingComponent)]
		struct #component_identifier;

		#input_fn_vis struct #plugin_identifier;
		impl bevy::prelude::Plugin for #plugin_identifier {
			fn build(&self, app: &mut App) {
				#input_fn_block
			}
		}
	};
	result.into()
}
