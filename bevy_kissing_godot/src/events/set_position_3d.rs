use super::utils::get_node;
use crate::{
	entity_or_node_id::EntityOrNodeId,
	prelude::{AllNodes, GodotNodeId},
};

use bevy::prelude::*;
use godot::prelude::*;

#[derive(Event)]
pub struct SetPosition3D {
	entity_or_node_id: EntityOrNodeId,
	position: Vector3,
}

impl SetPosition3D {
	pub fn new(entity_or_node_id: EntityOrNodeId, position: Vector3) -> Self {
		Self {
			entity_or_node_id,
			position,
		}
	}
}

pub(crate) fn on_set_position_3d(
	event: On<SetPosition3D>,
	nodes: Query<&mut GodotNodeId>,
	all_nodes: NonSend<AllNodes>,
) -> bevy::prelude::Result<()> {
	let mut node_3d = get_node::<Node3D>(event.entity_or_node_id, nodes, &all_nodes)?;
	node_3d.set_position(event.position);
	Ok(())
}
