use quote::quote;
use syn::ItemStruct;

/// Given a copy of the input `kissing_component` struct, returns the token-stream
/// for the Bevy component that replaces it.
pub(super) fn generate_component_struct(original_struct: ItemStruct) -> proc_macro2::TokenStream {
	let mut result = original_struct;

	// Update field types for Bevy component.
	for f in &mut result.fields {
		f.attrs.clear();

		match &f.ty {
			syn::Type::Tuple(tuple_type) => {
				if let Some(ty) = tuple_type.elems.iter().nth(1) {
					f.ty = ty.clone();
				}
			}
			_ => (),
		}
	}

	quote! {
		#result
	}
}
