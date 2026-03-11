use crate::utils::get_doc_comment_from_attrs;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
	Field, Fields, Ident, ItemStruct, LitInt, Path, Type, parse_macro_input, spanned::Spanned,
};

// -----------
// * Structs *
// -----------

struct KissingEventExpressions {
	argument_count: usize,
	requires_entity: bool,
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
	GodotSignalArg(GodotSignalArgData),
	DirectValue(proc_macro2::TokenStream),
}

// ----------------------
// * GodotSignalArgData *
// ----------------------

struct GodotSignalArgData {
	index: LitInt,
	gd_handle: bool,
	from_variant: Option<Path>,
}

impl GodotSignalArgData {
	fn generate_variant_converstion_expr(
		&self,
		variant_array_expr: proc_macro2::TokenStream,
	) -> proc_macro2::TokenStream {
		let index = &self.index;
		let variant_expr = quote!(#variant_array_expr[#index]);
		if self.gd_handle {
			quote!(bevy_kissing_godot::resources::gd_handle::GdHandle::from_variant(#variant_expr))
		} else if let Some(from_variant) = &self.from_variant {
			quote!(#from_variant(#variant_expr))
		} else {
			quote!(#variant_expr.to())
		}
	}
}

impl syn::parse::Parse for GodotSignalArgData {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		use syn::{Error, Expr, ExprLit, Lit, Meta, Token, punctuated::Punctuated};

		fn invalid_argument<T: quote::ToTokens>(tokens: T) -> Error {
			Error::new_spanned(
				tokens,
				"argument must be `index = <INT>`, `gd_handle`, or `from_variant = <FUNCTION_PATH>`",
			)
		}

		let args = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;

		let mut index: Option<LitInt> = None;
		let mut gd_handle: bool = false;
		let mut from_variant: Option<Path> = None;

		for meta in args {
			match meta {
				Meta::Path(path) => {
					if path.is_ident("gd_handle") {
						gd_handle = true;
					} else {
						return Err(invalid_argument(path));
					}
				}
				Meta::NameValue(meta) => {
					let ident = meta
						.path
						.get_ident()
						.ok_or_else(|| invalid_argument(&meta.path))?
						.to_string();

					match ident.as_str() {
						"index" => match meta.value {
							Expr::Lit(ExprLit {
								lit: Lit::Int(lit), ..
							}) => {
								index = Some(lit);
							}
							other => {
								return Err(invalid_argument(other));
							}
						},
						"from_variant" => {
							if let Expr::Path(expr_path) = meta.value {
								from_variant = Some(expr_path.path);
							} else {
								return Err(invalid_argument(meta.value));
							}
						}
						_ => {
							return Err(invalid_argument(meta.path));
						}
					}
				}
				_ => {
					return Err(invalid_argument(meta));
				}
			}
		}

		if gd_handle && from_variant.is_some() {
			return Err(Error::new(
				input.span(),
				"cannot provide both `gd_handle` and `from_variant`",
			));
		}

		let index = index
			.ok_or_else(|| Error::new(input.span(), "missing required argument `index = <INT>`"))?;

		Ok(Self {
			index,
			gd_handle,
			from_variant,
		})
	}
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

	let KissingEventExpressions {
		argument_count,
		requires_entity,
		untyped_constructor_expr,
		typed_constructor_expr,
		typed_slot_args,
	} = kissing_event_expressions;

	let untyped_slot_entity_argument = if requires_entity {
		Some(quote!(entity: bevy::prelude::Entity,))
	} else {
		None
	};

	quote! {
		impl bevy::prelude::Command for #ident {
			fn apply(self, world: &mut bevy::prelude::World) {
				world.trigger(self);
			}
		}

		impl #ident {
			pub fn trigger(commands: &mut bevy::ecs::world::CommandQueue, entity: bevy::prelude::Entity, args: &[&godot::prelude::Variant]) {
				commands.push(#untyped_constructor_expr);
			}

			pub fn untyped_slot(scene_tree: &mut godot::prelude::SceneTree, #untyped_slot_entity_argument args: &[&godot::prelude::Variant]) {
				use bevy_kissing_godot::extensions::scene_tree::SceneTreeExt;
				scene_tree.push_to_command_queue(#untyped_constructor_expr);
			}

			pub fn typed_slot(scene_tree: &mut godot::prelude::SceneTree, #(#typed_slot_args),*) {
				use bevy_kissing_godot::extensions::scene_tree::SceneTreeExt;
				scene_tree.push_to_command_queue(#typed_constructor_expr);
			}
		}

		bevy_kissing_godot::prelude::bevy_kissing_godot_inventory::submit! {
			bevy_kissing_godot::kissing_event::kissing_event_callbacks::KissingEventCallbacks::new(
				|| bevy_kissing_godot::kissing_event::kissing_event_data::KissingEventData {
					name: stringify!(#ident),
					argument_count: #argument_count,
					docs: #docs,
				},
				#ident::trigger,
			)
		}
	}.into()
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

		requires_entity: kissing_fields
			.iter()
			.any(|f| matches!(f.kind, FieldKind::EventTarget)),

		untyped_constructor_expr: match fields {
			Fields::Named(_) => {
				let args = kissing_fields
					.iter()
					.map(|f| {
						let ident = f.ident.clone();
						match &f.kind {
							FieldKind::EventTarget => quote!(#ident: entity),
							FieldKind::GodotSignalArg(data) => {
								let expr = data.generate_variant_converstion_expr(quote!(args));
								quote!(#ident: #expr)
							}
							FieldKind::DirectValue(expression) => quote!(#ident: #expression),
						}
					})
					.collect::<Vec<proc_macro2::TokenStream>>();
				quote!(Self { #(#args),* })
			}
			Fields::Unnamed(_) => {
				let args = kissing_fields
					.iter()
					.map(|f| match &f.kind {
						FieldKind::EventTarget => quote!(entity),
						FieldKind::GodotSignalArg(data) => {
							data.generate_variant_converstion_expr(quote!(args))
						}
						FieldKind::DirectValue(expression) => expression.clone(),
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
					match &f.kind {
						FieldKind::EventTarget => quote!(entity),
						FieldKind::GodotSignalArg(_) => quote!(#ident),
						FieldKind::DirectValue(expression) => quote!(#ident: #expression),
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
						FieldKind::EventTarget => Some(quote!(entity: bevy::prelude::Entity)),
						FieldKind::GodotSignalArg(_) => Some(quote!(#ident: #ty)),
						FieldKind::DirectValue(_) => None,
					}
				})
				.filter_map(|v| v)
				.collect::<Vec<proc_macro2::TokenStream>>()
		},
	})
}

fn parse_field(field: &Field) -> syn::Result<FieldKind> {
	use syn::{Error, Meta};

	for attr in &field.attrs {
		if attr.path().is_ident("event_target") {
			return Ok(FieldKind::EventTarget);
		} else if attr.path().is_ident("godot_signal_arg") {
			let data: GodotSignalArgData = attr.parse_args()?;
			return Ok(FieldKind::GodotSignalArg(data));
		} else if attr.path().is_ident("godot_signal_value") {
			return match &attr.meta {
				Meta::List(meta_list) => Ok(FieldKind::DirectValue(meta_list.tokens.clone())),
				_ => Err(Error::new_spanned(
					attr,
					"expected name-value syntax `#[godot_signal_value(<EXPRESSION>)]`",
				)),
			};
		}
	}

	Err(Error::new(
		field.span(),
		"`#[event_target]`, `#[godot_signal_arg(index = <INT>)]`, or `#[godot_signal_value(<EXPRESSION>)]` required for all KissingEvent fields",
	))
}
