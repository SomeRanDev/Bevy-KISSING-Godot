use std::collections::BTreeMap;

use bevy::prelude::*;
use godot::prelude::*;

use crate::prelude::GodotNodeId;

// ---------------
// * AllNodes *
// ---------------

/// Stores a list of all nodes live in the `SceneTree`.
/// This resource is required to access `Gd<Node>` instances from a `GodotNodeId`.
///
/// It must be passed to a Bevy function with `bevy::prelude::NonSend` as it directly stores
/// `Gd<T>` objects that cannot be sent across threads.
#[derive(Default)]
pub struct AllNodes {
	nodes: Vec<Option<Gd<Node>>>,
	empty_indexes: Vec<usize>,
	instance_id_to_tracker_id: BTreeMap<InstanceId, usize>,
}

impl AllNodes {
	/// Provides a `GodotNodeId` given a `Node`.
	/// `pub` since used in user code in via generated macro code.
	pub fn get_id_from_node(&self, node: &Gd<Node>) -> Option<GodotNodeId> {
		let instance_id = node.instance_id();
		self.instance_id_to_tracker_id
			.get(&instance_id)
			.map(|id| GodotNodeId::new(*id))
	}

	pub(crate) fn register(&mut self, node: Gd<Node>) -> usize {
		let instance_id = node.instance_id();
		let id = self.register_impl(node);
		self.instance_id_to_tracker_id.insert(instance_id, id);
		id
	}

	fn register_impl(&mut self, node: Gd<Node>) -> usize {
		// Reuse hole in `nodes` if it exists.
		if let Some(new_index) = self.empty_indexes.pop() {
			// Checks if there is a valid element of value `None` at `new_index`.
			if let Some(None) = self.nodes.get(new_index) {
				self.nodes[new_index] = node.into();
				return new_index;
			}
		}

		// If no free indexes, just add to the end.
		self.nodes.push(node.into());
		self.nodes.len() - 1
	}

	pub(crate) fn get(&self, index: usize) -> Gd<Node> {
		match self.nodes.get(index) {
			Some(n) => n.clone().unwrap(),
			None => panic!("Could not get node from AllNodes."),
		}
	}

	pub(crate) fn remove(&mut self, instance_id: &InstanceId) {
		let Some(id) = self.instance_id_to_tracker_id.remove(instance_id) else {
			return;
		};
		let Some(node) = self.nodes.get_mut(id) else {
			return;
		};
		let _ = node.take();
		self.empty_indexes.push(id);
	}
}
