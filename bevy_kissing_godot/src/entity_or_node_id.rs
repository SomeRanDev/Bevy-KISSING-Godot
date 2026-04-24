use crate::prelude::GodotNodeId;

use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EntityOrNodeId {
	Entity(Entity),
	NodeId(GodotNodeId),
}

impl From<Entity> for EntityOrNodeId {
	fn from(entity: Entity) -> Self {
		Self::Entity(entity)
	}
}

impl From<GodotNodeId> for EntityOrNodeId {
	fn from(godot_node_id: GodotNodeId) -> Self {
		Self::NodeId(godot_node_id)
	}
}

impl std::fmt::Display for EntityOrNodeId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Entity(entity) => write!(f, "EntityOrNodeId(Entity({}))", entity),
			Self::NodeId(godot_node_id) => write!(f, "EntityOrNodeId({})", godot_node_id),
		}
	}
}
