use super::{error::Error, utils::get_node};
use crate::{
	entity_or_node_id::EntityOrNodeId,
	prelude::{AllNodes, GodotNodeId},
};

use bevy::prelude::*;
use godot::prelude::*;

/// To use this event, with any type that isn't [`godot::classes::Node`] it must be registered
/// with the Bevy app using [`bevy_kissing_godot::plugins::node_events::RunCodeOnNode::register`].
///
/// ```rust
/// use godot::classes::MeshInstance3D;
///
/// struct RegisterMeshInstance3D;
///
/// impl Plugin for RegisterMeshInstance3D {
///     fn build(app: &mut App) {
///         RunCodeOnNode::<MeshInstance3D>::register(app);
///     }
/// }
/// ```
///
/// From there, a function pointer can be passed and executed using [`bevy::prelude::Commands`]:
/// ```rust
/// use bevy::prelude::*;
///
/// fn on_something(
///     _event: On<Something>,
///     mut commands: Commands,
///     mesh: Single<Entity, With<GodotNode<MeshInstance3D>>>
/// ) {
///     let entity = mesh.into_inner();
///     commands.trigger(RunCodeOnNode::new(
///         entity,
///         |mi3d: Gd<MeshInstance3D>| {
///             mi3d.create_trimesh_collision();
///         }
///     ));
/// }
/// ```
#[derive(Event)]
pub struct RunCodeOnNode<T: GodotClass + Inherits<Node>> {
	entity_or_node_id: EntityOrNodeId,
	callback: fn(Gd<T>) -> (),
}

impl<T: GodotClass + Inherits<Node>> RunCodeOnNode<T> {
	pub fn new(entity_or_node_id: EntityOrNodeId, callback: fn(Gd<T>) -> ()) -> Self {
		Self {
			entity_or_node_id,
			callback,
		}
	}

	pub fn register(app: &mut App) {
		app.add_observer(on_run_code_on_node::<T>);
	}
}

fn on_run_code_on_node<T: GodotClass + Inherits<Node>>(
	event: On<RunCodeOnNode<T>>,
	nodes: Query<&mut GodotNodeId>,
	all_nodes: NonSend<AllNodes>,
) -> bevy::prelude::Result<()> {
	let node = get_node::<Node>(event.entity_or_node_id, nodes, &all_nodes)?;
	if let Ok(typed_node) = node.try_cast::<T>() {
		(event.callback)(typed_node);
		Ok(())
	} else {
		Err(Error::NodeWrongType(event.entity_or_node_id).into())
	}
}

pub(crate) fn on_run_code_on_untyped_node(
	event: On<RunCodeOnNode<Node>>,
	nodes: Query<&mut GodotNodeId>,
	all_nodes: NonSend<AllNodes>,
) -> bevy::prelude::Result<()> {
	let node = get_node::<Node>(event.entity_or_node_id, nodes, &all_nodes)?;
	(event.callback)(node);
	Ok(())
}
