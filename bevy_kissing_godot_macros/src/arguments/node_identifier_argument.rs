use syn::{
	Error, Expr, Ident, MetaNameValue, Path, Result, Token,
	parse::{Parse, ParseStream},
	punctuated::Punctuated,
	spanned::Spanned,
};

// ---

const NAME_ARGUMENT_NAME: &str = "node_name";
const PROCESS_WRAP_ARGUMENT_NAME: &str = "process_wrapper";
const PHYSICS_PROCESS_WRAP_ARGUMENT_NAME: &str = "physics_process_wrapper";

// ---

pub(crate) struct NodeIdentifierArgument {
	pub(crate) node_name: Ident,
	pub(crate) process_wrapper_macro: Option<Path>,
	pub(crate) physics_process_wrapper_macro: Option<Path>,
}

// ---

impl Parse for NodeIdentifierArgument {
	fn parse(input: ParseStream) -> Result<Self> {
		let entries: Punctuated<MetaNameValue, Token![,]> = Punctuated::parse_terminated(input)?;
		let entries_span = entries.span();
		let entries = entries.into_iter().collect::<Vec<MetaNameValue>>();

		let mut node_name: Option<Ident> = None;
		let mut process_wrapper_macro: Option<Path> = None;
		let mut physics_process_wrapper_macro: Option<Path> = None;
		for entry in entries {
			// node_name
			if entry.path.is_ident(NAME_ARGUMENT_NAME) {
				match &entry.value {
					Expr::Path(expr_path) => {
						if expr_path.path.segments.len() == 1 {
							node_name = expr_path.path.segments.first().map(|ps| ps.ident.clone());
						}
					}
					_ => (),
				}

				if node_name.is_none() {
					return Err(Error::new_spanned(
						entry,
						format!("{} must be assigned an identifier", NAME_ARGUMENT_NAME),
					));
				}
			}

			// process_wrapper
			if entry.path.is_ident(PROCESS_WRAP_ARGUMENT_NAME) {
				match &entry.value {
					Expr::Path(expr_path) => {
						process_wrapper_macro = Some(expr_path.path.clone());
					}
					_ => (),
				}

				if node_name.is_none() {
					return Err(Error::new_spanned(
						entry,
						format!(
							"{} must be assigned an path to a macro",
							PROCESS_WRAP_ARGUMENT_NAME
						),
					));
				}
			}

			// physics_process_wrapper
			if entry.path.is_ident(PHYSICS_PROCESS_WRAP_ARGUMENT_NAME) {
				match &entry.value {
					Expr::Path(expr_path) => {
						physics_process_wrapper_macro = Some(expr_path.path.clone());
					}
					_ => (),
				}

				if node_name.is_none() {
					return Err(Error::new_spanned(
						entry,
						format!(
							"{} must be assigned an path to a macro",
							PHYSICS_PROCESS_WRAP_ARGUMENT_NAME
						),
					));
				}
			}
		}

		let Some(node_name) = node_name else {
			return Err(Error::new(
				entries_span,
				format!("{} must be assigned an identifier", NAME_ARGUMENT_NAME),
			));
		};

		Ok(Self {
			node_name,
			process_wrapper_macro,
			physics_process_wrapper_macro,
		})
	}
}
