use proc_macro::TokenStream;
use quote::quote;
use syn::{TypePath, parse_macro_input};

pub(super) fn kiss_node_impl(input: TokenStream) -> TokenStream {
	let input_path_arg = parse_macro_input!(input as TypePath);
	let ident = input_path_arg
		.path
		.segments
		.last()
		.expect("Type path is empty.");

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
