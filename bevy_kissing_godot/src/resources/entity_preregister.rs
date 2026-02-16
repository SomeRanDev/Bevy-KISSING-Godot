use std::collections::BTreeMap;

use bevy::prelude::*;
use godot::prelude::*;

#[derive(Default)]
pub struct EntityPreregister {
	instance_id_to_entity: BTreeMap<InstanceId, Entity>,
}

impl EntityPreregister {
	/// Use this to generate a Bevy entity for a Godot Node prior to it being
	/// properly registered with Bevy💋Godot. This should ONLY be used when
	/// creating a Node at runtime.
	pub fn get_entity_for_node<T: Inherits<Node>>(
		&mut self,
		commands: &mut Commands,
		node: &Gd<T>,
	) -> Entity {
		let instance_id = node.instance_id();
		let existing_entity = self.instance_id_to_entity.get(&instance_id);
		if let Some(existing_entity) = existing_entity {
			*existing_entity
		} else {
			let entity = commands.spawn_empty().id();
			self.instance_id_to_entity.insert(instance_id, entity);
			entity
		}
	}

	pub(crate) fn take_entity_if_exists<T: Inherits<Node>>(
		&mut self,
		node: &Gd<T>,
	) -> Option<Entity> {
		let instance_id = node.instance_id();
		self.instance_id_to_entity.remove(&instance_id)
	}
}
