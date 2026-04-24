use super::utils::get_node;
use crate::{
	entity_or_node_id::EntityOrNodeId,
	prelude::{AllNodes, GodotNodeId},
};

use bevy::prelude::*;
use godot::prelude::*;

/// To use this event, its generic type must be registered with the Bevy app using
/// [`bevy_kissing_godot::plugins::node_events::SetGodotProperty::register`].
///
/// ```rust
/// struct RegisterI32;
///
/// impl Plugin for RegisterI32 {
///     fn build(app: &mut App) {
///         SetGodotProperty::<i32>::register(app);
///     }
/// }
/// ```
#[derive(Event)]
pub struct SetGodotProperty<T> {
	entity_or_node_id: EntityOrNodeId,
	property_name: String,
	value: T,
}

impl<T: ToGodot + Send + Sync + 'static> SetGodotProperty<T> {
	pub fn new(entity_or_node_id: EntityOrNodeId, property_name: String, value: T) -> Self {
		Self {
			entity_or_node_id,
			property_name,
			value,
		}
	}

	pub fn register(app: &mut App) {
		app.add_observer(on_set_godot_property::<T>);
	}
}

pub(crate) fn on_set_godot_property<T: ToGodot + Send + Sync + 'static>(
	event: On<SetGodotProperty<T>>,
	nodes: Query<&mut GodotNodeId>,
	all_nodes: NonSend<AllNodes>,
) -> bevy::prelude::Result<()> {
	let mut node = get_node::<Node>(event.entity_or_node_id, nodes, &all_nodes)?;
	node.set(&event.property_name, &event.value.to_variant());
	Ok(())
}
