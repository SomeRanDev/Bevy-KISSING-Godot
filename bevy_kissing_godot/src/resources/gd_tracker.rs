use crate::{
	components::gd_tracker_id::{GdTrackerId, GodotResourceId},
	prelude::GodotNodeId,
};

use std::collections::BTreeMap;

use bevy::prelude::*;
use godot::prelude::*;

// ------------
// * AllNodes *
// ------------

pub type AllNodes = GdTracker<Node, GodotNodeId>;

// ----------------
// * AllResources *
// ----------------

pub type AllResources = GdTracker<godot::prelude::Resource, GodotResourceId>;

// -------------
// * GdTracker *
// -------------

/// Stores a list of all nodes live in the `SceneTree`.
/// This resource is required to access `Gd<T>` instances from a `GodotNodeId` or `GodotResourceId`.
///
/// It must be passed to a Bevy function with `bevy::prelude::NonSend` as it directly stores
/// `Gd<T>` objects that cannot be sent across threads.
pub struct GdTracker<T: GodotClass, TrackerType: GdTrackerId> {
	nodes: Vec<Option<Gd<T>>>,
	empty_indexes: Vec<usize>,
	instance_id_to_tracker_id: BTreeMap<InstanceId, usize>,
	_spooky: std::marker::PhantomData<TrackerType>,
}

impl<T: GodotClass, TrackerType: GdTrackerId> Default for GdTracker<T, TrackerType> {
	fn default() -> Self {
		Self {
			nodes: vec![],
			empty_indexes: vec![],
			instance_id_to_tracker_id: BTreeMap::default(),
			_spooky: Default::default(),
		}
	}
}

impl<T: GodotClass, TrackerType: GdTrackerId> GdTracker<T, TrackerType> {
	/// Provides a `GdTrackerId` given a `T`.
	/// `pub` since used in "user code" generated via macro code.
	pub fn get_or_register_id_from_node(&mut self, node: &Gd<T>) -> TrackerType {
		if let Some(node_id) = self.get_id_from_instance_id(&node.instance_id()) {
			node_id
		} else {
			self.register(node.clone())
		}
	}

	/// Provides a `GdTrackerId` given an `InstanceId`.
	pub fn get_id_from_instance_id(&self, instance_id: &InstanceId) -> Option<TrackerType> {
		self.instance_id_to_tracker_id
			.get(&instance_id)
			.map(|id| TrackerType::new(*id))
	}

	pub(crate) fn register(&mut self, node: Gd<T>) -> TrackerType {
		let instance_id = node.instance_id();
		let id = self.register_impl(node);
		self.instance_id_to_tracker_id.insert(instance_id, id);
		TrackerType::new(id)
	}

	fn register_impl(&mut self, node: Gd<T>) -> usize {
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

	pub(crate) fn get(&self, index: usize) -> Gd<T> {
		match self.nodes.get(index) {
			Some(n) => n.clone().unwrap(),
			None => panic!("Could not get node from AllNodes."),
		}
	}

	pub(crate) fn try_get(&self, index: usize) -> Option<Gd<T>> {
		self.nodes.get(index).and_then(|a| a.clone())
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
