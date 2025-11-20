// ----------
// * Traits *
// ----------

/// A trait that is required for a component's property's type to be compatible
/// in a "kissing" component.
pub trait KissingComponentField {
	fn parse_from_editor_input(input: &String) -> Self;
}

// ----------
// * String *
// ----------

impl KissingComponentField for String {
	fn parse_from_editor_input(input: &String) -> Self {
		input.clone()
	}
}

// -----------
// * Numbers *
// -----------

macro_rules! impl_number_types {
	($($ty:ty),*) => {
		$(
			impl KissingComponentField for $ty {
				fn parse_from_editor_input(input: &String) -> Self {
					input.parse::<$ty>().unwrap_or_default()
				}
			}
		)*
	};
}

impl_number_types!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);
