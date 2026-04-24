use super::error::Error;
use crate::{
	entity_or_node_id::EntityOrNodeId,
	prelude::{AllNodes, GodotNodeId},
};

use bevy::prelude::*;
use godot::prelude::*;

pub(super) fn get_node<T: Inherits<Node>>(
	entity_or_node_id: EntityOrNodeId,
	mut nodes: Query<&mut GodotNodeId>,
	all_nodes: &AllNodes,
) -> Result<Gd<T>, Error> {
	let id = match entity_or_node_id {
		EntityOrNodeId::Entity(entity) => nodes.get_mut(entity).map(|id| *id).ok(),
		EntityOrNodeId::NodeId(godot_node_id) => Some(godot_node_id),
	};

	let Some(parent) = id.and_then(|id| id.try_get_as::<T>(&all_nodes)) else {
		return Err(Error::NodeDoesntExist(entity_or_node_id));
	};

	Ok(parent)
}

pub(super) fn get_parent_and_child<Parent: Inherits<Node>, Child: Inherits<Node>>(
	parent_entity_or_node_id: EntityOrNodeId,
	child_entity_or_node_id: EntityOrNodeId,
	mut nodes: Query<&mut GodotNodeId>,
	all_nodes: &AllNodes,
) -> Result<(Gd<Parent>, Gd<Child>), Error> {
	let parent_id = match parent_entity_or_node_id {
		EntityOrNodeId::Entity(entity) => nodes.get_mut(entity).map(|id| *id).ok(),
		EntityOrNodeId::NodeId(godot_node_id) => Some(godot_node_id),
	};
	let Some(parent) = parent_id.and_then(|id| id.try_get_as::<Parent>(&all_nodes)) else {
		return Err(Error::ParentNodeDoesntExist(parent_entity_or_node_id));
	};

	let child_id = match child_entity_or_node_id {
		EntityOrNodeId::Entity(entity) => nodes.get_mut(entity).map(|id| *id).ok(),
		EntityOrNodeId::NodeId(godot_node_id) => Some(godot_node_id),
	};
	let Some(child) = child_id.and_then(|id| id.try_get_as::<Child>(&all_nodes)) else {
		return Err(Error::ChildNodeDoesntExist(child_entity_or_node_id));
	};

	Ok((parent, child))
}
