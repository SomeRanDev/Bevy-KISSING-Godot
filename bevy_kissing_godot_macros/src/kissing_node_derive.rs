use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};

pub(super) fn kissing_node_derive_impl(input: TokenStream) -> TokenStream {
	let struct_input = parse_macro_input!(input as ItemStruct);
	let ident = struct_input.ident;

	// Add additional static fields and add [kissing_component_data] function to inventory.
	let result = quote! {
		bevy_kissing_godot::prelude::bevy_kissing_godot_inventory::submit! {
			bevy_kissing_godot::kissing_node::kissing_node::KissingNode::new(
				stringify!(#ident),
				|world| bevy_kissing_godot::kissing_node::kissing_node::KissingNode::create_entity_with_godot_node_class_components::<#ident>(world),
			)
		}
	};

	result.into()
}
