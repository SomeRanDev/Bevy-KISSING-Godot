use std::fmt::Display;

use bevy::prelude::*;
use godot::prelude::*;

use crate::kissing_event::kissing_event::EVENT_NAME_TO_SLOT_FUNC;

// ---------
// * Enums *
// ---------

enum ConvertEventDataVariantToRustError {
	NotArray,
	EntryNotDictionary,
	EntryLacksSignal,
	EntryLacksEvent,
}

impl Display for ConvertEventDataVariantToRustError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			Self::NotArray => "not an array",
			Self::EntryNotDictionary => "entry not a dictionary",
			Self::EntryLacksSignal => "entry dictionary lacks \"signal\"",
			Self::EntryLacksEvent => "entry dictionary lacks \"event\"",
		})
	}
}

// -------------
// * Functions *
// -------------

/// Applies the "kissing" events defined in the Godot editor on a node.
pub fn apply_kissing_events<'a>(node: &mut Gd<Node>, entity: Entity) {
	if !node.has_meta("bevy_events") {
		return;
	}

	let event_data = node.get_meta("bevy_events");
	let d = match convert_event_data_variant_to_rust(event_data) {
		Ok(d) => d,
		Err(e) => {
			godot_error!(
				"Bevy Event connection metadata for {} is malformed (reason: {}).",
				node,
				e
			);
			return;
		}
	};

	for (signal, event) in d {
		let Some(callback) = EVENT_NAME_TO_SLOT_FUNC.get(&event) else {
			continue;
		};
		let entity_clone = entity.clone();
		node.connect(
			&signal,
			&Callable::from_sync_fn("", move |args| {
				callback(entity_clone, args);
			}),
		);
	}
}

/// Converts the "bevy_components" metadata from a `Node`, to a
/// Rust-digestable representation.
fn convert_event_data_variant_to_rust(
	variant: Variant,
) -> Result<Vec<(StringName, StringName)>, ConvertEventDataVariantToRustError> {
	let mut result = vec![];

	let Ok(event_data) = variant.try_to::<Array<Variant>>() else {
		return Err(ConvertEventDataVariantToRustError::NotArray);
	};
	for i in 0..event_data.len() {
		let Some(event) = event_data
			.get(i)
			.and_then(|d| d.try_to::<VarDictionary>().ok())
		else {
			return Err(ConvertEventDataVariantToRustError::EntryNotDictionary);
		};

		let Some(signal_name) = event
			.get("signal")
			.and_then(|v| v.try_to_relaxed::<StringName>().ok())
		else {
			return Err(ConvertEventDataVariantToRustError::EntryLacksSignal);
		};

		let Some(event_name) = event
			.get("event")
			.and_then(|v| v.try_to_relaxed::<StringName>().ok())
		else {
			return Err(ConvertEventDataVariantToRustError::EntryLacksEvent);
		};

		result.push((signal_name, event_name));
	}

	Ok(result)
}
