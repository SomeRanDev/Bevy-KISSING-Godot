use crate::arguments::node_identifier_argument::NodeIdentifierArgument;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

pub(crate) fn kiss_bevy_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
	let input_arg = syn::parse_macro_input!(attr as NodeIdentifierArgument);
	let input_fn = parse_macro_input!(item as ItemFn);
	let input_fn_name = input_fn.sig.ident.clone();

	let node_identifier = input_arg.node_name;

	let process_call = quote!(self.app.process(delta));
	let process_call = input_arg
		.process_wrapper_macro
		.map(|m| quote!(#m!(#process_call, self)))
		.unwrap_or(process_call);

	let physics_process_call = quote!(self.app.physics_process(delta));
	let physics_process_call = input_arg
		.physics_process_wrapper_macro
		.map(|m| quote!(#m!(#physics_process_call, self)))
		.unwrap_or(physics_process_call);

	let input_event_function = if cfg!(feature = "input") {
		Some(quote! {
			fn input(&mut self, event: godot::obj::Gd<godot::classes::InputEvent>) {
				self.app.input(event);
			}
		})
	} else {
		None
	};

	let result = quote! {
		#[derive(godot::prelude::GodotClass)]
		#[class(init, base = Node)]
		pub struct #node_identifier {
			base: godot::obj::Base<godot::prelude::Node>,
			app: bevy_kissing_godot::kissing_app::KissingApp,
		}

		#[godot::prelude::godot_api]
		impl #node_identifier {
		}

		#[godot::prelude::godot_api]
		impl godot::prelude::INode for #node_identifier {
			fn ready(&mut self) {
				use godot::obj::WithBaseField;

				self.app.pre_ready();

				#input_fn_name(self.app.get_app_mut());

				let tree = self.base().get_tree();
				self.app.post_ready(self.to_gd().upcast(), tree);
			}

			fn process(&mut self, delta: f64) {
				#process_call;
			}

			fn physics_process(&mut self, delta: f64) {
				#physics_process_call;
			}

			#input_event_function
		}

		#input_fn
	};

	result.into()
}
