use super::{error::Error, utils::get_parent_and_child};
use crate::{
	entity_or_node_id::EntityOrNodeId,
	prelude::{AllNodes, GodotNodeId},
};

use bevy::prelude::*;
use godot::prelude::*;

/// Runs `remove_child` on the Godot nodes that correlate to the provided Bevy entities.
#[derive(Event)]
pub struct RemoveChild {
	entity: EntityOrNodeId,
	child: EntityOrNodeId,
}

impl RemoveChild {
	pub fn new(parent: EntityOrNodeId, child: EntityOrNodeId) -> Self {
		Self {
			entity: parent,
			child,
		}
	}
}

pub(crate) fn on_remove_child(
	event: On<RemoveChild>,
	nodes: Query<&mut GodotNodeId>,
	all_nodes: NonSend<AllNodes>,
) -> bevy::prelude::Result<()> {
	if event.entity == event.child {
		return Err(Error::CannotRemoveChildFromItself.into());
	}
	let (mut parent, child) =
		get_parent_and_child::<Node, Node>(event.entity, event.child, nodes, &all_nodes)?;
	parent.remove_child(&child);
	Ok(())
}
