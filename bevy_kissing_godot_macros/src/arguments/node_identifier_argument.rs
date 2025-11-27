use syn::{
	Error, Ident, Path, Result, Token,
	parse::{Parse, ParseStream},
	punctuated::Punctuated,
};

pub(crate) struct NodeIdentifierArgument {
	pub(crate) app_identifier: Ident,
	pub(crate) process_wrapper_macro: Option<Path>,
	pub(crate) physics_process_wrapper_macro: Option<Path>,
}

impl Parse for NodeIdentifierArgument {
	fn parse(input: ParseStream) -> Result<Self> {
		let paths: Punctuated<Path, Token![,]> = Punctuated::parse_terminated(input)?;
		let paths = paths.into_iter().collect::<Vec<Path>>();

		let app_identifier = if let Some(first) = paths.first() {
			if let Some(ident) = first.get_ident() {
				ident
			} else {
				return Err(Error::new(
					input.span(),
					"The Bevy App node name must be a single identifier.",
				));
			}
		} else {
			return Err(Error::new(
				input.span(),
				"At least a single argument for the Bevy App node name is expected.",
			));
		};

		if paths.len() > 3 {
			return Err(Error::new(
				input.span(),
				"Only 3 arguments expected: (bevy_app_name: Ident, process_wrapper_macro_path: Path, physics_process_wrapper_macro_path: Path)",
			));
		}

		Ok(Self {
			app_identifier: app_identifier.clone(),
			process_wrapper_macro: paths.get(1).cloned(),
			physics_process_wrapper_macro: paths.get(2).cloned(),
		})
	}
}
