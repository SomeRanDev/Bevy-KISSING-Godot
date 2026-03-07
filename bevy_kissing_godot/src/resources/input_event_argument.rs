use std::ops::Deref;

use godot::{classes::InputEvent, obj::Gd};

pub struct InputEventArgument(pub(crate) Option<Gd<InputEvent>>);

impl InputEventArgument {
	pub fn take(&mut self) -> Option<Gd<InputEvent>> {
		self.0.take()
	}
}

impl Deref for InputEventArgument {
	type Target = Option<Gd<InputEvent>>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
