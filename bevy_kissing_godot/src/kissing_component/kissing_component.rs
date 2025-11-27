use crate::kissing_component::kissing_component_data::KissingComponentData;

use std::{
	collections::{BTreeMap, HashMap},
	sync::LazyLock,
};

use bevy::prelude::*;
use godot::prelude::*;

// -------------------------
// * Top-Level Macro Calls *
// -------------------------

inventory::collect!(KissingComponent);

// -------------------------
// * Top-Level Static Vars *
// -------------------------

/// A `HashMap` that, given a name of a `KissingComponent`-derived `Component`,
/// returns its `add_component_from_editor_fields`.
pub static COMPONENT_NAME_TO_FUNC: LazyLock<
	HashMap<StringName, &AddComponentFromEditorFieldsCallback>,
> = LazyLock::new(|| {
	let mut component_name_to_func =
		HashMap::<StringName, &AddComponentFromEditorFieldsCallback>::new();
	for kissing_component in inventory::iter::<KissingComponent>() {
		let data = kissing_component.get_data();
		let name = StringName::from(data.name);
		component_name_to_func.insert(
			name,
			kissing_component.get_add_component_from_editor_fields(),
		);
	}
	component_name_to_func
});

// ----------------
// * Type Aliases *
// ----------------

/// A reference to a component's static function [add_component_from_editor_fields].
type AddComponentFromEditorFieldsCallback = fn(
	node: &Gd<Node>,
	world: &mut World,
	entity: &Entity,
	fields: BTreeMap<String, Variant>,
) -> bool;

// ----------------
// * Structs *
// ----------------

/// Used by inventory to store references to static functions for editor components.
pub struct KissingComponent {
	kissing_component_data: fn() -> KissingComponentData,
	add_component_from_editor_fields: AddComponentFromEditorFieldsCallback,
}

impl KissingComponent {
	pub const fn new(
		kissing_component_data: fn() -> KissingComponentData,
		add_component_from_editor_fields: AddComponentFromEditorFieldsCallback,
	) -> Self {
		Self {
			kissing_component_data,
			add_component_from_editor_fields,
		}
	}

	pub fn get_data(&self) -> KissingComponentData {
		(self.kissing_component_data)()
	}

	pub fn get_add_component_from_editor_fields(&self) -> &AddComponentFromEditorFieldsCallback {
		&self.add_component_from_editor_fields
	}
}
