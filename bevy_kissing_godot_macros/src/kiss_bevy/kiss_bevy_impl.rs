use crate::kiss_bevy::node_identifier_argument::NodeIdentifierArgument;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

pub(crate) fn kiss_bevy_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
	let input_arg = syn::parse_macro_input!(attr as NodeIdentifierArgument);
	let input_fn = parse_macro_input!(item as ItemFn);
	let input_fn_name = input_fn.sig.ident.clone();

	let node_identifier = input_arg.ident;

	let result = quote! {
		#[derive(godot::prelude::GodotClass)]
		#[class(init, base = Node)]
		pub struct #node_identifier {
			base: Base<godot::prelude::Node>,
			app: bevy_kissing_godot::kissing_app::KissingApp,
		}

		#[godot::prelude::godot_api]
		impl #node_identifier {
			fn on_node_added(&mut self, node_added: Gd<Node>) {
				self.app.on_node_added(node_added);
			}

			fn on_node_removed(&mut self, node_removed: Gd<Node>) {
				self.app.on_node_removed(node_removed);
			}
		}

		#[godot::prelude::godot_api]
		impl godot::prelude::INode for #node_identifier {
			fn ready(&mut self) {
				self.app.pre_ready();

				#input_fn_name(self.app.get_app_mut());

				let Some(tree) = self.base().get_tree() else {
					godot_warn!("Could not get SceneTree while setting up Bevy App.");
					return;
				};
				tree.signals().node_added().connect_other(self, Self::on_node_added);
				tree.signals().node_removed().connect_other(self, Self::on_node_removed);
				self.app.post_ready(tree);
			}

			fn process(&mut self, delta: f64) {
				self.app.process(delta);
			}

			fn physics_process(&mut self, delta: f64) {
				self.app.physics_process(delta);
			}
		}

		#input_fn
	};

	result.into()
}
