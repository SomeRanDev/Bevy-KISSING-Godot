use crate::resources::all_nodes::AllNodes;

use bevy::prelude::*;
use godot::prelude::*;

#[derive(Component, Debug)]
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

	pub fn try_get_as<T: Inherits<Node>>(&self, all_nodes: &AllNodes) -> Option<Gd<T>> {
		all_nodes
			.try_get(self.id)
			.and_then(|n| n.try_cast::<T>().ok())
	}
}
