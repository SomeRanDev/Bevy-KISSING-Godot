use crate::components::gd_tracker_id::GodotNodeId;
use crate::components::godot_node::GodotNode;
use crate::resources::gd_tracker::AllNodes;

use bevy::prelude::*;
use godot::prelude::*;

// ---------
// * Types *
// ---------

/// An alias for a `Query` for a Godot Node with a component.
pub type QueryGodotNodeWith<'world, 'state, 'id, T> =
	Query<'world, 'state, &'id GodotNodeId, With<T>>;

/// An alias for a `Query` for a Godot Node type.
pub type QueryGodotNode<'world, 'state, 'id, T> =
	QueryGodotNodeWith<'world, 'state, 'id, GodotNode<T>>;

/// An alias for a `Single` query for a Godot Node with a component.
pub type SingleGodotNodeWith<'world, 'state, 'id, T> =
	Single<'world, 'state, &'id GodotNodeId, With<T>>;

/// An alias for a `Single` query for a Godot Node type.
pub type SingleGodotNode<'world, 'state, 'id, T> =
	SingleGodotNodeWith<'world, 'state, 'id, GodotNode<T>>;

// ----------
// * Traits *
// ----------

pub trait GodotNodeQueryUtils<T: GodotClass> {
	fn get(&self, all_nodes: &AllNodes) -> Gd<T>;
}

impl<'world, 'state, 'id, T: GodotClass + Inherits<Node>> GodotNodeQueryUtils<T>
	for SingleGodotNode<'world, 'state, 'id, T>
{
	fn get(&self, all_nodes: &AllNodes) -> Gd<T> {
		self.get_as::<T>(all_nodes)
	}
}
