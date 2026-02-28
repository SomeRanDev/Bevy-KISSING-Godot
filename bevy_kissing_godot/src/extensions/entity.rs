use bevy::prelude::Entity;
use godot::{
	meta::ToGodot,
	prelude::{Variant, Vector2i},
};

pub trait EntityExt {
	fn to_godot_variant(&self) -> Variant;
}

impl EntityExt for Entity {
	fn to_godot_variant(&self) -> Variant {
		unsafe { std::mem::transmute::<u64, Vector2i>(self.to_bits()) }.to_variant()
	}
}
