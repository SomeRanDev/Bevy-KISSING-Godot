use proc_macro::TokenStream;

// -----------
// * Modules *
// -----------

mod arguments;
mod get_compilation_timestamp;
mod kiss_bevy;
mod kiss_node;
mod kissing_component_derive;
mod kissing_node_derive;
mod plugin_and_kissing_component;

// -------------
// * Functions *
// -------------

#[proc_macro]
pub fn get_compilation_timestamp(input: TokenStream) -> TokenStream {
	get_compilation_timestamp::get_compilation_timestamp_impl(input)
}

#[proc_macro_attribute]
pub fn kiss_bevy(attr: TokenStream, item: TokenStream) -> TokenStream {
	kiss_bevy::kiss_bevy_impl(attr, item)
}

#[proc_macro]
pub fn kiss_node(input: TokenStream) -> TokenStream {
	kiss_node::kiss_node_impl(input)
}

#[proc_macro_derive(KissingComponent)]
pub fn kissing_component_derive(input: TokenStream) -> TokenStream {
	kissing_component_derive::kissing_component_derive_impl(input)
}

#[proc_macro_derive(KissingNode)]
pub fn kissing_node_derive(input: TokenStream) -> TokenStream {
	kissing_node_derive::kissing_node_derive_impl(input)
}

#[proc_macro_attribute]
pub fn plugin_and_kissing_component(attr: TokenStream, item: TokenStream) -> TokenStream {
	plugin_and_kissing_component::plugin_and_kissing_component_impl(attr, item)
}
