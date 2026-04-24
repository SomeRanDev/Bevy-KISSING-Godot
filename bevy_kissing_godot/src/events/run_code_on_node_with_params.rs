use super::{error::Error, utils::get_node};
use crate::{
	entity_or_node_id::EntityOrNodeId,
	prelude::{AllNodes, GodotNodeId},
};

use bevy::prelude::*;
use godot::prelude::*;

/// To use this event, it must be registered with the Bevy app using both the node type (`T`)
/// and the parameters type (`Params`): [`bevy_kissing_godot::plugins::node_events::RunCodeOnNodeWithParams::register`].
///
/// ```rust
/// use godot::classes::MeshInstance3D;
///
/// struct RegisterMeshInstance3DWithI32;
///
/// impl Plugin for RegisterMeshInstance3DWithI32 {
///     fn build(app: &mut App) {
///         RunCodeOnNodeWithParams::<MeshInstance3D, (bool, bool)>::register(app);
///     }
/// }
/// ```
///
/// From there, a function pointer that also takes the `Params` argument can be passed and
/// executed using [`bevy::prelude::Commands`]:
///
/// ```rust
/// use bevy::prelude::*;
///
/// fn on_something(
///     _event: On<Something>,
///     mut commands: Commands,
///     mesh: Single<Entity, With<GodotNode<MeshInstance3D>>>
/// ) {
///     let entity = mesh.into_inner();
///     commands.trigger(
///         RunCodeOnNodeWithParams::new(
///             entity,
///             |mi3d: Gd<MeshInstance3D>, params: (bool, bool)| {
///                 mi3d.create_convex_collision_ex()
///                     .clean(params.0)
///                     .simplify(params.1)
///                     .done();
///             },
///             (true, false)
///         )
///     );
/// }
/// ```
#[derive(Event)]
pub struct RunCodeOnNodeWithParams<T: GodotClass + Inherits<Node>, Params: Send + Sync + 'static> {
	entity_or_node_id: EntityOrNodeId,
	callback: fn(Gd<T>, &Params) -> (),
	params: Params,
}

impl<T: GodotClass + Inherits<Node>, Params: Send + Sync + 'static>
	RunCodeOnNodeWithParams<T, Params>
{
	pub fn new(
		entity_or_node_id: EntityOrNodeId,
		callback: fn(Gd<T>, &Params) -> (),
		params: Params,
	) -> Self {
		Self {
			entity_or_node_id,
			callback,
			params,
		}
	}

	pub fn register(app: &mut App) {
		app.add_observer(on_run_code_on_node_with_params::<T, Params>);
	}
}

fn on_run_code_on_node_with_params<
	T: GodotClass + Inherits<Node>,
	Params: Send + Sync + 'static,
>(
	event: On<RunCodeOnNodeWithParams<T, Params>>,
	nodes: Query<&mut GodotNodeId>,
	all_nodes: NonSend<AllNodes>,
) -> bevy::prelude::Result<()> {
	let node = get_node::<Node>(event.entity_or_node_id, nodes, &all_nodes)?;
	if let Ok(typed_node) = node.try_cast::<T>() {
		(event.callback)(typed_node, &event.params);
		Ok(())
	} else {
		Err(Error::NodeWrongType(event.entity_or_node_id).into())
	}
}
