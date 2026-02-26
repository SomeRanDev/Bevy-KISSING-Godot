use crate::{
	components::gd_tracker_id::{GdTrackerId, GodotResourceId},
	prelude::GodotNodeId,
};

use std::collections::BTreeMap;

use bevy::prelude::*;
use godot::prelude::*;

// -------------------------
// * GdTrackerErrorStrings *
// -------------------------

pub trait GdTrackerErrorStrings {
	const COULD_NOT_GET_OBJECT: &'static str;
}

// ------------
// * AllNodes *
// ------------

/// Stores a list of all nodes live in the `SceneTree`.
/// This resource is required to access `Gd<T>` instances from a `GodotNodeId`.
///
/// It must be passed to a Bevy function with `bevy::prelude::NonSend` as it directly stores
/// `Gd<T>` objects that cannot be sent across threads.
pub type AllNodes = GdTracker<Node, GodotNodeId, AllNodesErrorStrings>;

pub struct AllNodesErrorStrings;
impl GdTrackerErrorStrings for AllNodesErrorStrings {
	const COULD_NOT_GET_OBJECT: &'static str = "could not get node from AllNodes";
}

// ----------------
// * AllResources *
// ----------------

/// Stores a list of all resources that have been loaded.
/// This resource is required to access `Gd<T>` instances from a `GodotResourceId`.
///
/// It must be passed to a Bevy function with `bevy::prelude::NonSend` as it directly stores
/// `Gd<T>` objects that cannot be sent across threads.
pub type AllResources =
	GdTracker<godot::prelude::Resource, GodotResourceId, AllResourcesErrorStrings>;

pub struct AllResourcesErrorStrings;
impl GdTrackerErrorStrings for AllResourcesErrorStrings {
	const COULD_NOT_GET_OBJECT: &'static str = "could not get resource from AllResources";
}

// -------------
// * GdTracker *
// -------------

/// The underlying implementation for [`AllNodes`] and [`AllResources`].
pub struct GdTracker<T: GodotClass, TrackerType: GdTrackerId, ErrorStrings: GdTrackerErrorStrings> {
	gd_objects: Vec<Option<Gd<T>>>,
	empty_indexes: Vec<usize>,
	instance_id_to_tracker_id: BTreeMap<InstanceId, usize>,
	_spooky: (
		std::marker::PhantomData<TrackerType>,
		std::marker::PhantomData<ErrorStrings>,
	),
}

impl<T: GodotClass, TrackerType: GdTrackerId, ErrorStrings: GdTrackerErrorStrings> Default
	for GdTracker<T, TrackerType, ErrorStrings>
{
	fn default() -> Self {
		Self {
			gd_objects: vec![],
			empty_indexes: vec![],
			instance_id_to_tracker_id: BTreeMap::default(),
			_spooky: Default::default(),
		}
	}
}

impl<T: GodotClass, TrackerType: GdTrackerId, ErrorStrings: GdTrackerErrorStrings>
	GdTracker<T, TrackerType, ErrorStrings>
{
	/// Provides a `GdTrackerId` given a `T`.
	/// `pub` since used in "user code" generated via macro code.
	pub fn get_or_register_id_from_gd_object(&mut self, gd_object: &Gd<T>) -> TrackerType {
		if let Some(gd_id) = self.get_id_from_instance_id(&gd_object.instance_id()) {
			gd_id
		} else {
			self.register(gd_object.clone())
		}
	}

	/// Provides a `GdTrackerId` given an `InstanceId`.
	pub fn get_id_from_instance_id(&self, instance_id: &InstanceId) -> Option<TrackerType> {
		self.instance_id_to_tracker_id
			.get(&instance_id)
			.map(|id| TrackerType::new(*id))
	}

	pub(crate) fn register(&mut self, gd_object: Gd<T>) -> TrackerType {
		let instance_id = gd_object.instance_id();
		let id = self.register_impl(gd_object);
		self.instance_id_to_tracker_id.insert(instance_id, id);
		TrackerType::new(id)
	}

	fn register_impl(&mut self, gd_object: Gd<T>) -> usize {
		// Reuse hole in `gd_objects` if it exists.
		if let Some(new_index) = self.empty_indexes.pop() {
			// Checks if there is a valid element of value `None` at `new_index`.
			if let Some(None) = self.gd_objects.get(new_index) {
				self.gd_objects[new_index] = gd_object.into();
				return new_index;
			}
		}

		// If no free indexes, just add to the end.
		self.gd_objects.push(gd_object.into());
		self.gd_objects.len() - 1
	}

	pub(crate) fn get(&self, index: usize) -> Gd<T> {
		match self.gd_objects.get(index) {
			Some(n) => n.clone().unwrap(),
			None => panic!("{}", ErrorStrings::COULD_NOT_GET_OBJECT),
		}
	}

	pub(crate) fn try_get(&self, index: usize) -> Option<Gd<T>> {
		self.gd_objects.get(index).and_then(|a| a.clone())
	}

	pub(crate) fn remove(&mut self, instance_id: &InstanceId) {
		let Some(id) = self.instance_id_to_tracker_id.remove(instance_id) else {
			return;
		};
		let Some(gd_object) = self.gd_objects.get_mut(id) else {
			return;
		};
		let _ = gd_object.take();
		self.empty_indexes.push(id);
	}
}
