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

/// Used to mark the entry function for a Bevy💋Godot app.
///
/// This attribute requires one argument for the name of the Bevy app node that needs
/// to be added as an autoload in the editor:
/// ```rust
/// #[kiss_bevy(MyAppNodeName)]
/// fn main(app: &mut bevy::prelude::App) {
///		// Do stuff with `app`...
/// }
/// ```
///
/// The second and third arguments are optional. If provided, they are both expected to
/// be paths to macros that take two expression arguments. The "second" is a macro that
/// wraps the "process" expression of the generated Bevy app node. The "third" is the
/// same, but it's for the "physics_process" expression.
///
/// These macros should take two arguments:
/// 	* The first is the original expression
/// 	* The second is the `self` expression
///
/// ```rust
/// macro_rules panic_catcher {
/// 	($process: expr, $self: expr) => {
///			let result = std::panic::catch_unwind(|| {
/// 			$process
/// 		});
/// 		if result.is_err() {
/// 			println!("Panic happened!");
///
/// 			// Check bevy_kissing_godot::kissing_app for all `self.app` functions.
/// 			$self.app.clear_app();
/// 		}
/// 	}
/// }
///
/// #[kiss_bevy(MyAppNodeName, panic_catcher, panic_catcher)]
/// fn main(app: &mut bevy::prelude::App) {
///		// Do stuff with `app`...
/// }
/// ```
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
