use bevy::prelude::*;
use godot::prelude::*;

use crate::resources::all_nodes::AllNodes;

#[derive(Component)]
pub struct GodotNodeId {
	id: usize,
}

impl GodotNodeId {
	pub fn new(id: usize) -> Self {
		Self { id }
	}

	pub fn get_as<T: Inherits<Node>>(&self, all_nodes: &AllNodes) -> Gd<T> {
		all_nodes.get(self.id).try_cast::<T>().unwrap()
	}
}
