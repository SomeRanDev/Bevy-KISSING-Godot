use bevy::prelude::*;
use godot::prelude::*;

#[derive(Component)]
pub struct GodotNodeId {
	instance_id: InstanceId,
}

impl GodotNodeId {
	pub fn new<T: Inherits<Node>>(node: &Gd<T>) -> Self {
		Self {
			instance_id: node.instance_id(),
		}
	}

	pub fn get_as<T: Inherits<Node>>(&self) -> Gd<T> {
		Gd::try_from_instance_id(self.instance_id).unwrap()
	}
}
