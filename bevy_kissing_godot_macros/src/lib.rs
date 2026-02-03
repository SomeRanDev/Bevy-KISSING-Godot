use proc_macro::TokenStream;

// -----------
// * Modules *
// -----------

mod arguments;
mod get_compilation_timestamp;
mod kiss_bevy;
mod kiss_node;
mod kissing_component;
mod kissing_component_derive;
mod kissing_node_derive;
mod plugin_and_kissing_component;
mod utils;

// -------------
// * Functions *
// -------------

/// Used to expose a Bevy component to the Godot editor.
///
/// This should be used alongside `#[derive(bevy::prelude::Component, bevy_kissing_godot::prelude::KissingComponent)]`.
/// ```rust
/// #[derive(Component, KissingComponent)]
/// #[kissing_component]
/// struct Health {
/// 	maximum: i32,
/// }
/// ```
///
/// To enable a field to be modified in the Godot editor, the `#[export]` attribute can be used.
/// Optionally, a default-value can be provided with `initial_value = VALUE`.
/// ```rust
/// #[derive(Component, KissingComponent)]
/// #[kissing_component]
/// struct Health {
/// 	#[export(initial_value = 100)]
/// 	maximum: i32,
/// }
/// ```
///
/// `#[export_node]` can be used to allow for a `NodePath` input.
///
/// To allow only certain classes to be selected in the editor, Godot classes may be listed
/// as the arguments to the attribute:
/// ```rust
/// #[export_node(Camera3D, CollisionShape3D, Path3D)]
/// ```
///
/// A field with `#[export_node]` MUST be an `Option<bevy_kissing_godot::prelude::GodotNodeId>`.
/// `GodotNodeId` can be converted an actual `Gd<T>` node through `NonSend<AllNodes>` at runtime.
/// ```rust
/// #[derive(Component, KissingComponent)]
/// #[kissing_component]
/// struct Health {
/// 	#[export(initial_value = 100)]
/// 	maximum: i32,
///
/// 	#[export_node(Label)]
/// 	label: Option<GodotNodeId>,
/// }
///
/// fn on_update(
///		healths: Query<&Health>,
///		all_nodes: NonSend<AllNodes>,
/// ) {
/// 	for health in gooblers.iter() {
/// 		let mut health_label: Gd<Label> = health.label.get_as::<Label>(&all_nodes);
/// 	}
/// }
/// ```
#[proc_macro_attribute]
pub fn kissing_component(attr: TokenStream, item: TokenStream) -> TokenStream {
	kissing_component::kissing_component_impl(attr, item)
}

/// A utility used to track whether the Rust binary has been recompiled in Godot.
/// See `kissing_component::kissing_component_registry::KissingComponentRegistry::get_compilation_id`.
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

/// This should be added to all custom Godot types added in Rust so they will appear in the hierarchy.
/// NOTE: Is there any difference between this and `#[derive(KissingNode)]`??? Why did I add both???
#[proc_macro]
pub fn kiss_node(input: TokenStream) -> TokenStream {
	kiss_node::kiss_node_impl(input)
}

/// Generates functions and metadata to allow for a Bevy component to be accessible from the Godot editor.
/// This should be used with `#[kissing_component]`.
#[proc_macro_derive(KissingComponent)]
pub fn kissing_component_derive(input: TokenStream) -> TokenStream {
	kissing_component_derive::kissing_component_derive_impl(input)
}

/// This should be added to all custom Godot types added in Rust so they will appear in the hierarchy.
/// NOTE: Is there any difference between this and `#[kiss_node]`??? Why did I add both???
#[proc_macro_derive(KissingNode)]
pub fn kissing_node_derive(input: TokenStream) -> TokenStream {
	kissing_node_derive::kissing_node_derive_impl(input)
}

/// Given a function that takes an `app: &mut bevy::prelude::App`, this generates an empty
/// Bevy component AND a `bevy::prelude::Plugin` that runs the function as its `build` function.
///
/// This code...
/// ```rust
/// #[plugin_and_kissing_component(Cool)]
/// pub(crate) fn cool_plugin(app: &mut App) {
/// 	app.add_systems(Startup, init_cool_stuff);
/// }
/// ```
///
/// ...gets converted to this.
/// ```rust
/// #[kissing_component]
/// #[derive(Component, KissingComponent)]
/// struct CoolComponent;
///
/// pub(crate) struct CoolPlugin;
/// impl Plugin for CoolPlugin {
/// 	fn build(&self, app: &mut App) {
/// 		app.add_systems(Startup, init_cool_stuff);
/// 	}
/// }
/// ```
#[proc_macro_attribute]
pub fn plugin_and_kissing_component(attr: TokenStream, item: TokenStream) -> TokenStream {
	plugin_and_kissing_component::plugin_and_kissing_component_impl(attr, item)
}
