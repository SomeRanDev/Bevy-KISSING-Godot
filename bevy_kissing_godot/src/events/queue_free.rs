use super::utils::get_node;
use crate::{
	entity_or_node_id::EntityOrNodeId,
	prelude::{AllNodes, GodotNodeId},
};

use bevy::prelude::*;
use godot::prelude::*;

#[derive(Event)]
pub struct QueueFree {
	entity_or_node_id: EntityOrNodeId,
}

impl QueueFree {
	pub fn new(entity_or_node_id: EntityOrNodeId) -> Self {
		Self { entity_or_node_id }
	}
}

pub(crate) fn on_queue_free(
	event: On<QueueFree>,
	nodes: Query<&mut GodotNodeId>,
	all_nodes: NonSend<AllNodes>,
) -> bevy::prelude::Result<()> {
	let mut node = get_node::<Node>(event.entity_or_node_id, nodes, &all_nodes)?;
	node.queue_free();
	Ok(())
}
