use super::utils::get_node;
use crate::{
	entity_or_node_id::EntityOrNodeId,
	prelude::{AllNodes, GodotNodeId},
};

use bevy::prelude::*;
use godot::prelude::*;

#[derive(Event)]
pub struct SetScale3D {
	entity_or_node_id: EntityOrNodeId,
	scale: Vector3,
}

impl SetScale3D {
	pub fn new(entity_or_node_id: EntityOrNodeId, scale: Vector3) -> Self {
		Self {
			entity_or_node_id,
			scale,
		}
	}
}

pub(crate) fn on_set_scale_3d(
	event: On<SetScale3D>,
	nodes: Query<&mut GodotNodeId>,
	all_nodes: NonSend<AllNodes>,
) -> bevy::prelude::Result<()> {
	let mut node_3d = get_node::<Node3D>(event.entity_or_node_id, nodes, &all_nodes)?;
	node_3d.set_scale(event.scale);
	Ok(())
}
