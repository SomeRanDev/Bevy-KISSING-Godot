use crate::{kissing_event::kissing_event_data::KissingEventData, kissing_registry::GetData};

use bevy::{ecs::world::CommandQueue, prelude::*};
use godot::prelude::*;
//use std::{collections::HashMap, sync::LazyLock};

// -------------------------
// * Top-Level Macro Calls *
// -------------------------

inventory::collect!(KissingEventCallbacks);

// -------------------------
// * Top-Level Static Vars *
// -------------------------

// /// A `HashMap` that, given a name of a `KissingEvent`-derived `Event`/`EntityEvent`,
// /// returns its `untyped_slot_callback`.
// pub static EVENT_NAME_TO_SLOT_FUNC: LazyLock<HashMap<StringName, &UntypedSlotCallback>> =
// 	LazyLock::new(|| {
// 		let mut event_name_to_func = HashMap::<StringName, &UntypedSlotCallback>::new();
// 		for kissing_event in inventory::iter::<KissingEventCallbacks>() {
// 			let data = kissing_event.get_data();
// 			let name = StringName::from(data.name);
// 			event_name_to_func.insert(name, &kissing_event.untyped_slot_callback);
// 		}
// 		event_name_to_func
// 	});

// ----------------
// * Type Aliases *
// ----------------

//pub(crate) type UntypedSlotCallback = fn(entity: Entity, args: &[&Variant]) -> ();
pub(crate) type TriggerCallback =
	fn(commands: &mut CommandQueue, entity: Entity, args: &[&Variant]) -> ();

// -----------
// * Structs *
// -----------

/// Used by inventory to store references to static functions for editor events.
pub struct KissingEventCallbacks {
	pub(crate) kissing_event_data: fn() -> KissingEventData,
	// commands_callback: fn(&mut Commands) -> (),
	// untyped_slot_callback: UntypedSlotCallback,
	pub(crate) trigger: TriggerCallback,
}

impl KissingEventCallbacks {
	pub const fn new(
		kissing_event_data: fn() -> KissingEventData,
		// commands_callback: fn(&mut Commands) -> (),
		// untyped_slot_callback: UntypedSlotCallback,
		trigger: TriggerCallback,
	) -> Self {
		Self {
			kissing_event_data,
			// commands_callback,
			// untyped_slot_callback,
			trigger,
		}
	}

	// pub fn run_commands_callback(&self, commands: &mut Commands) {
	// 	(self.commands_callback)(commands);
	// }
}

impl GetData for KissingEventCallbacks {
	type Data = KissingEventData;
	fn get_data(&self) -> KissingEventData {
		(self.kissing_event_data)()
	}
}
