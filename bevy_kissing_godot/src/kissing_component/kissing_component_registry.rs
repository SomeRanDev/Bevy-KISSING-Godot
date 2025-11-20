use crate::kissing_component::kissing_component::{COMPONENT_NAME_TO_FUNC, KissingComponent};
use crate::kissing_component::kissing_component_data::KissingComponentData;

use bevy_kissing_godot_macros::get_compilation_timestamp;

use std::collections::BTreeMap;

use bevy::prelude::*;
use godot::prelude::*;

// -----------
// * Structs *
// -----------

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct KissingComponentRegistry {
	base: Base<Node>,
}

impl KissingComponentRegistry {
	/// Applies the "kissing" components defined in the Godot editor on a node.
	pub fn apply_kissing_components<'a>(node: &Gd<Node>, world: &mut World, entity: Entity) {
		if !node.has_meta("bevy_components") {
			return;
		}

		let component_data = node.get_meta("bevy_components");
		let d = Self::convert_component_data_variant_to_rust(component_data);
		for entry in d {
			let Some(func) = COMPONENT_NAME_TO_FUNC.get(&entry.0) else {
				continue;
			};
			func(world, &entity, entry.1);
		}
	}

	/// Converts the "bevy_components" metadata from a `Node`, to a
	/// Rust-digestable representation.
	fn convert_component_data_variant_to_rust(
		variant: Variant,
	) -> Vec<(StringName, BTreeMap<String, String>)> {
		let mut result = vec![];

		let Ok(component_data) = variant.try_to::<Array<Variant>>() else {
			return vec![];
		};
		for i in 0..component_data.len() {
			let Some(component) = component_data
				.get(i)
				.and_then(|d| d.try_to::<Dictionary>().ok())
			else {
				continue;
			};
			let (Some(name), Some(data)) = (
				component
					.get("name")
					.and_then(|n| n.try_to::<StringName>().ok()),
				component
					.get("data")
					.and_then(|n| n.try_to::<Dictionary>().ok()),
			) else {
				continue;
			};

			result.push((name, Self::convert_dictionary_to_string_string_map(data)));
		}
		result
	}

	/// Given a Godot `Dictionary` that has only strings for both keys and values,
	/// converts it to a `BTreeMap`.
	fn convert_dictionary_to_string_string_map(dictionary: Dictionary) -> BTreeMap<String, String> {
		let mut values = BTreeMap::<String, String>::new();
		let keys = dictionary.keys_array();
		for j in 0..keys.len() {
			let Some(key) = keys.get(j) else { continue };
			let Ok(key_string_name) = key.try_to::<String>() else {
				continue;
			};
			let Some(value) = dictionary.get(key) else {
				continue;
			};
			values.insert(key_string_name.to_string(), value.to::<String>());
		}
		values
	}
}

#[godot_api]
impl KissingComponentRegistry {
	/// Provides the "kissing" component data in a Godot-compatible format.
	///
	/// The key/value pairs of the [`Dictionary`] correlate to the fields of [`KissingComponentData`].
	#[func]
	pub fn find_all_kissing_components() -> Array<Dictionary> {
		let all_component_data = inventory::iter::<KissingComponent>()
			.map(|e| e.get_data())
			.collect::<Vec<KissingComponentData>>();
		let mut result = array![];
		for component_data in all_component_data {
			let fields_gd = component_data
				.fields
				.iter()
				.map(|s| s.to_dictionary())
				.collect::<Array<Dictionary>>();

			result.push(&vdict!(
				"name": component_data.name.to_variant(),
				"docs": component_data.docs.to_variant(),
				"fields": fields_gd,
			));
		}
		result
	}

	/// To avoid re-running the slightly expensive [`find_all_kissing_components`], this function can be used to
	/// determine if any new Bevy components *could* be available.
	///
	/// If [`get_compilation_id`] is the same as it was last time [`find_all_kissing_components`] was called, it
	/// means the Rust lib has not been compiled since then, and a cached value from the previous call can be used.
	#[func]
	fn get_compilation_id() -> real {
		get_compilation_timestamp!()
	}
}
