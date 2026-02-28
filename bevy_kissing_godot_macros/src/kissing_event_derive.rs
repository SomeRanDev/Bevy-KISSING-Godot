use crate::utils::get_doc_comment_from_attrs;
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Field, Fields, Ident, ItemStruct, LitInt, Type, parse_macro_input, spanned::Spanned};

// -----------
// * Structs *
// -----------

struct KissingEventExpressions {
	argument_count: usize,
	untyped_constructor_expr: proc_macro2::TokenStream,
	typed_constructor_expr: proc_macro2::TokenStream,
	typed_slot_args: Vec<proc_macro2::TokenStream>,
}

struct KissingEventField {
	kind: FieldKind,
	ident: Ident,
	ty: Type,
}

// ---------
// * Enums *
// ---------

enum FieldKind {
	EventTarget,
	GodotSignalArg(usize),
}

// -------------
// * Functions *
// -------------

pub(super) fn kissing_event_derive_impl(input: TokenStream) -> TokenStream {
	let item_struct = parse_macro_input!(input as ItemStruct);

	let kissing_event_expressions = match parse_fields(item_struct.fields) {
		Ok(expr) => expr,
		Err(e) => return e.into_compile_error().into(),
	};

	let docs = get_doc_comment_from_attrs(&item_struct.attrs);

	let ident = item_struct.ident;
	let name = format_ident!("{}_QUEUE", ident.to_string().to_case(Case::UpperSnake));

	let argument_count = kissing_event_expressions.argument_count;
	let untyped_constructor_expr = kissing_event_expressions.untyped_constructor_expr;
	let typed_constructor_expr = kissing_event_expressions.typed_constructor_expr;
	let typed_slot_args = kissing_event_expressions.typed_slot_args;

	quote! {
		static #name: std::sync::LazyLock<
			bevy_kissing_godot::prelude::bevy_kissing_godot_concurrent_queue::ConcurrentQueue<#ident>,
		> = std::sync::LazyLock::new(|| bevy_kissing_godot::prelude::bevy_kissing_godot_concurrent_queue::ConcurrentQueue::unbounded());

		impl #ident {
			pub fn untyped_slot(entity: bevy::prelude::Entity, args: &[&godot::prelude::Variant]) {
				match #name.push(#untyped_constructor_expr) {
					Ok(()) => (),
					Err(e) => godot::prelude::godot_error!("Failed to queue slot for {} {}", stringify!(#ident), e),
				}
			}

			pub fn typed_slot(#(#typed_slot_args),*) {
				match #name.push(#typed_constructor_expr) {
					Ok(()) => (),
					Err(e) => godot::prelude::godot_error!("Failed to queue slot for {} {}", stringify!(#ident), e),
				}
			}

			pub fn execute_queue(commands: &mut bevy::prelude::Commands) {
				while !#name.is_empty() {
					match #name.pop() {
						Ok(event) => commands.trigger(event),
						Err(e) => godot::prelude::godot_error!("Failed to execute slot for {} {}", stringify!(#ident), e),
					}
				}
			}
		}

		bevy_kissing_godot::prelude::bevy_kissing_godot_inventory::submit! {
			bevy_kissing_godot::kissing_event::kissing_event::KissingEvent::new(
				|| bevy_kissing_godot::kissing_event::kissing_event_data::KissingEventData {
					name: stringify!(#ident),
					argument_count: #argument_count,
					docs: #docs,
				},
				#ident::execute_queue,
				#ident::untyped_slot,
			)
		}
	}
	.into()
}

fn generate_identifier_from_number(number: i32) -> Ident {
	Ident::new(&format!("_{}", number), proc_macro2::Span::call_site())
}

fn parse_fields(fields: Fields) -> syn::Result<KissingEventExpressions> {
	let mut kissing_fields = vec![];
	match &fields {
		Fields::Named(fields_named) => {
			let mut index = 0;
			for field in &fields_named.named {
				kissing_fields.push(KissingEventField {
					kind: parse_field(&field)?,
					ident: field
						.ident
						.clone()
						.unwrap_or_else(|| generate_identifier_from_number(index)),
					ty: field.ty.clone(),
				});
				index += 1;
			}
		}
		Fields::Unnamed(fields_unnamed) => {
			let mut index = 0;
			for field in &fields_unnamed.unnamed {
				kissing_fields.push(KissingEventField {
					kind: parse_field(&field)?,
					ident: field
						.ident
						.clone()
						.unwrap_or_else(|| generate_identifier_from_number(index)),
					ty: field.ty.clone(),
				});
				index += 1;
			}
		}
		Fields::Unit => (),
	};

	Ok(KissingEventExpressions {
		argument_count: kissing_fields
			.iter()
			.filter(|f| !matches!(f.kind, FieldKind::EventTarget))
			.count(),

		untyped_constructor_expr: match fields {
			Fields::Named(_) => {
				let args = kissing_fields
					.iter()
					.map(|f| {
						let ident = f.ident.clone();
						match f.kind {
							FieldKind::EventTarget => quote!(#ident: entity),
							FieldKind::GodotSignalArg(index) => quote!(#ident: args[#index].to()),
						}
					})
					.collect::<Vec<proc_macro2::TokenStream>>();
				quote!(Self { #(#args),* })
			}
			Fields::Unnamed(_) => {
				let args = kissing_fields
					.iter()
					.map(|f| match f.kind {
						FieldKind::EventTarget => quote!(entity),
						FieldKind::GodotSignalArg(index) => quote!(args[#index].to()),
					})
					.collect::<Vec<proc_macro2::TokenStream>>();
				quote!(Self(#(#args),*))
			}
			Fields::Unit => {
				quote!(Self)
			}
		},

		typed_constructor_expr: {
			let args = kissing_fields
				.iter()
				.map(|f| {
					let ident = f.ident.clone();
					match f.kind {
						FieldKind::EventTarget => quote!(entity),
						FieldKind::GodotSignalArg(_) => quote!(#ident),
					}
				})
				.collect::<Vec<proc_macro2::TokenStream>>();
			match fields {
				Fields::Named(_) => quote!(Self { #(#args),* }),
				Fields::Unnamed(_) => quote!(Self(#(#args),*)),
				Fields::Unit => quote!(Self),
			}
		},

		typed_slot_args: {
			kissing_fields
				.iter()
				.map(|f| {
					let ident = f.ident.clone();
					let ty = f.ty.clone();
					match f.kind {
						FieldKind::EventTarget => quote!(entity: bevy::prelude::Entity),
						FieldKind::GodotSignalArg(_) => quote!(#ident: #ty),
					}
				})
				.collect::<Vec<proc_macro2::TokenStream>>()
		},
	})
}

fn parse_field(field: &Field) -> syn::Result<FieldKind> {
	for attr in &field.attrs {
		if attr.path().is_ident("event_target") {
			return Ok(FieldKind::EventTarget);
		} else if attr.path().is_ident("godot_signal_arg") {
			let index: LitInt = attr.parse_args()?;
			let index = index.base10_parse::<usize>()?;
			return Ok(FieldKind::GodotSignalArg(index));
		}
	}
	Err(syn::Error::new(
		field.span(),
		"#[event_target] or #[godot_signal_arg(index: u32)] required for all KissingEvent fields",
	))
}
