use crate::{kissing_event::kissing_event_data::KissingEventData, kissing_registry::GetData};

use bevy::prelude::*;
use godot::prelude::*;
use std::{collections::HashMap, sync::LazyLock};

// -------------------------
// * Top-Level Macro Calls *
// -------------------------

inventory::collect!(KissingEvent);

// -------------------------
// * Top-Level Static Vars *
// -------------------------

/// A `HashMap` that, given a name of a `KissingComponent`-derived `Component`,
/// returns its `add_component_from_editor_fields`.
pub static EVENT_NAME_TO_SLOT_FUNC: LazyLock<HashMap<StringName, &UntypedSlotCallback>> =
	LazyLock::new(|| {
		let mut event_name_to_func = HashMap::<StringName, &UntypedSlotCallback>::new();
		for kissing_event in inventory::iter::<KissingEvent>() {
			let data = kissing_event.get_data();
			let name = StringName::from(data.name);
			event_name_to_func.insert(name, &kissing_event.untyped_slot_callback);
		}
		event_name_to_func
	});

// ----------------
// * Type Aliases *
// ----------------

type UntypedSlotCallback = fn(entity: Entity, args: &[&Variant]) -> ();

// -----------
// * Structs *
// -----------

/// Used by inventory to store references to static functions for editor events.
pub struct KissingEvent {
	kissing_event_data: fn() -> KissingEventData,
	commands_callback: fn(&mut Commands) -> (),
	untyped_slot_callback: UntypedSlotCallback,
}

impl KissingEvent {
	pub const fn new(
		kissing_event_data: fn() -> KissingEventData,
		commands_callback: fn(&mut Commands) -> (),
		untyped_slot_callback: UntypedSlotCallback,
	) -> Self {
		Self {
			kissing_event_data,
			commands_callback,
			untyped_slot_callback,
		}
	}

	pub fn run_commands_callback(&self, commands: &mut Commands) {
		(self.commands_callback)(commands);
	}
}

impl GetData for KissingEvent {
	type Data = KissingEventData;
	fn get_data(&self) -> KissingEventData {
		(self.kissing_event_data)()
	}
}
