use crate::{
	kissing_component::{
		kissing_component::KissingComponent, kissing_component_data::KissingComponentData,
	},
	kissing_event::{
		kissing_event_callbacks::KissingEventCallbacks, kissing_event_data::KissingEventData,
	},
};
use bevy_kissing_godot_macros::get_compilation_timestamp;

use godot::prelude::*;

// ----------
// * Traits *
// ----------

pub(crate) trait GetData {
	type Data;
	fn get_data(&self) -> Self::Data;
}

pub(crate) trait ToGodotDictionary {
	fn to_dictionary(&self) -> VarDictionary;
}

// -----------
// * Structs *
// -----------

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct KissingRegistry {
	base: Base<Node>,
}

#[godot_api]
impl KissingRegistry {
	/// Provides the "kissing" component data in a Godot-compatible format.
	///
	/// The key/value pairs of the [`VarDictionary`] correlate to the fields of [`KissingComponentData`].
	#[func]
	pub fn find_all_kissing_components() -> Array<VarDictionary> {
		Self::collect_inventory_as_godot_array::<KissingComponent, KissingComponentData>()
	}

	/// Provides the "kissing" event data in a Godot-compatible format.
	///
	/// The key/value pairs of the [`VarDictionary`] correlate to the fields of [`KissingEventData`].
	#[func]
	pub fn find_all_kissing_events() -> Array<VarDictionary> {
		Self::collect_inventory_as_godot_array::<KissingEventCallbacks, KissingEventData>()
	}

	/// Collects inventory of types that have data and returns them as a Godot array of dictionaries.
	fn collect_inventory_as_godot_array<
		T: GetData<Data = D> + inventory::Collect,
		D: ToGodotDictionary,
	>() -> Array<VarDictionary> {
		let mut result = array![];
		for item in inventory::iter::<T>() {
			let data = item.get_data();
			result.push(&data.to_dictionary());
		}
		result
	}

	/// To avoid re-running the slightly expensive [`find_all_kissing_events`], this function can be used to
	/// determine if any new Bevy components *could* be available.
	///
	/// If [`get_compilation_id`] is the same as it was last time [`find_all_kissing_components`] was called, it
	/// means the Rust lib has not been compiled since then, and a cached value from the previous call can be used.
	#[func]
	pub fn get_compilation_id() -> real {
		get_compilation_timestamp!()
	}
}
