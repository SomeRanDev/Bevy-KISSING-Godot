use std::{collections::BTreeMap, sync::LazyLock};

use bevy::prelude::*;
use godot::prelude::*;

// -------------------------
// * Top-Level Macro Calls *
// -------------------------

inventory::collect!(KissingNode);

// -----------
// * Structs *
// -----------

/// Used by inventory to store references to user-made Godot nodes.
pub struct KissingNode {
	name: &'static str,
	add_components: fn(world: &mut World) -> Entity,
}

impl KissingNode {
	pub const fn new(name: &'static str, add_components: fn(&mut World) -> Entity) -> Self {
		Self {
			name,
			add_components,
		}
	}

	pub fn add_components_for_kissing_node<'a>(
		world: &'a mut World,
		node: &Gd<Node>,
	) -> Option<EntityWorldMut<'a>> {
		static CUSTOM_NODES_MAPPING: LazyLock<BTreeMap<u32, fn(world: &mut World) -> Entity>> =
			LazyLock::new(|| {
				let mut result = BTreeMap::new();
				for kissing_node in inventory::iter::<KissingNode>() {
					let hash = StringName::from(kissing_node.name).hash_u32();
					result.insert(hash, kissing_node.add_components);
				}
				result
			});

		let id = StringName::from(&node.get_class()).hash_u32();
		CUSTOM_NODES_MAPPING
			.get(&id)
			.map(|f| f(world))
			.and_then(|e| world.get_entity_mut(e).ok())
	}

	pub fn aaa<T: GodotClass>(world: &mut World) -> Entity {
		let mut e = world.spawn_empty();
		crate::components::godot_node::GodotNode::<T>::add_components_from_type(&mut e);
		e.id()
	}
}
