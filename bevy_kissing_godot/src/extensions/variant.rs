use std::fmt::Display;

use bevy::prelude::Entity;
use godot::prelude::{Variant, Vector2i};

// ---

#[derive(Debug)]
pub enum ToBevyEntityError {
	VariantIsNotVector2i,
}

impl Display for ToBevyEntityError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("only Vector2i Variants can be converted to Bevy entities")
	}
}

impl std::error::Error for ToBevyEntityError {}

// ---

pub trait VariantExt {
	fn to_bevy_entity(&self) -> Result<Entity, ToBevyEntityError>;
}

impl VariantExt for Variant {
	fn to_bevy_entity(&self) -> Result<Entity, ToBevyEntityError> {
		let Ok(vector2i) = self.try_to::<Vector2i>() else {
			return Err(ToBevyEntityError::VariantIsNotVector2i);
		};
		let bits = unsafe { std::mem::transmute::<Vector2i, u64>(vector2i) };
		Ok(Entity::from_bits(bits))
	}
}
