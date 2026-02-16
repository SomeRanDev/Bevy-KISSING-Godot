use bevy::prelude::*;
use godot::prelude::*;

pub trait GdTrackerId {
	fn new(id: usize) -> Self;
}

macro_rules! define_tracker_id {
	($name: ident, $type: ty, $tracker: ty) => {
		#[derive(Component, Default, Debug, Clone)]
		pub struct $name {
			id: usize,
		}

		impl $name {
			pub fn get_as<T: Inherits<$type>>(&self, all_nodes: &$tracker) -> Gd<T> {
				all_nodes.get(self.id).try_cast::<T>().unwrap()
			}

			pub fn try_get_as<T: Inherits<$type>>(&self, all_nodes: &$tracker) -> Option<Gd<T>> {
				all_nodes
					.try_get(self.id)
					.and_then(|n| n.try_cast::<T>().ok())
			}
		}

		impl GdTrackerId for $name {
			fn new(id: usize) -> Self {
				Self { id }
			}
		}
	};
}

define_tracker_id!(
	GodotNodeId,
	godot::prelude::Node,
	crate::resources::gd_tracker::AllNodes
);
define_tracker_id!(
	GodotResourceId,
	godot::prelude::Resource,
	crate::resources::gd_tracker::AllResources
);
