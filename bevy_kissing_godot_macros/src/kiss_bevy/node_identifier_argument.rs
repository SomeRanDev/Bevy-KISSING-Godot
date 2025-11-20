use syn::{Error, Ident, Result, parse::Parse, parse::ParseStream};

pub(super) struct NodeIdentifierArgument {
	pub(super) ident: Ident,
}

const NODE_IDENTIFIER_ARGUMENT_ERROR: &'static str =
	"Expected a single identifier for the Bevy App node.";

impl Parse for NodeIdentifierArgument {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.is_empty() {
			return Err(Error::new(input.span(), NODE_IDENTIFIER_ARGUMENT_ERROR));
		}

		let ident: Ident = input.parse()?;

		if !input.is_empty() {
			let extra = input.parse::<proc_macro2::TokenTree>()?;
			return Err(Error::new_spanned(extra, NODE_IDENTIFIER_ARGUMENT_ERROR));
		}

		Ok(Self { ident })
	}
}
