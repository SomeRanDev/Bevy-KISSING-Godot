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
	add_components: fn(world: &mut World, entity: Entity) -> (),
}

impl KissingNode {
	pub const fn new(name: &'static str, add_components: fn(world: &mut World, entity: Entity) -> ()) -> Self {
		Self {
			name,
			add_components,
		}
	}

	pub fn add_godot_editor_components_for_kissing_node(
		world: &mut World,
		entity: Entity,
		node: &Gd<Node>,
	) -> bool {
		static CUSTOM_NODES_MAPPING: LazyLock<BTreeMap<u32, fn(world: &mut World, entity: Entity) -> ()>> =
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
			.map(|f| f(world, entity))
			.is_some()
	}

	pub fn create_entity_with_godot_node_class_components<T: GodotClass>(
		world: &mut World,
		entity: Entity,
	) -> () {
		let mut e = world.entity_mut(entity);
		crate::components::godot_node::GodotNode::<T>::add_components_from_type(&mut e);
	}
}
