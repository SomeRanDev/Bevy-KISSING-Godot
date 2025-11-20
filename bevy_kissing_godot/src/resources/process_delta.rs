use std::ops::Deref;

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ProcessDelta(pub(crate) f64);

impl Deref for ProcessDelta {
	type Target = f64;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
