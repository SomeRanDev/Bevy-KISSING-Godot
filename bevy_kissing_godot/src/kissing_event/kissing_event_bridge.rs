use crate::{
	kissing_event::kissing_event_callbacks::{KissingEventCallbacks, TriggerCallback},
	prelude::EntityExt,
};

use std::{collections::HashMap, fmt::Display, str::FromStr};

use bevy::prelude::*;
use godot::prelude::*;

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

// -----------
// * Structs *
// -----------

/// A utility struct for bridging the Godot signal system with Bevy's events.
/// It mainly stores cached references and maps to quickly setup a Node's connection.
pub(crate) struct KissingEventBridge {
	signal_callable: Option<Callable>,
	all_kissing_event_callbacks: Vec<&'static KissingEventCallbacks>,
	string_name_to_index: HashMap<StringName, usize>,
}

impl KissingEventBridge {
	/// Constructor.
	pub(crate) fn new() -> Self {
		let all_kissing_event_callbacks = inventory::iter::<KissingEventCallbacks>()
			.collect::<Vec<&'static KissingEventCallbacks>>();

		let mut string_name_to_index = HashMap::new();
		for (index, callbacks) in all_kissing_event_callbacks.iter().enumerate() {
			let Ok(string_name) = StringName::from_str((callbacks.kissing_event_data)().name);
			string_name_to_index.insert(string_name, index);
		}

		Self {
			signal_callable: None,
			all_kissing_event_callbacks,
			string_name_to_index,
		}
	}

	/// The [`signal_callback`] must be assigned later.
	pub(crate) fn set_signal_callback(&mut self, callable: Callable) {
		self.signal_callable = Some(callable);
	}

	/// Applies the "kissing" events defined in the Godot editor on a node.
	pub(crate) fn apply_kissing_events<'a>(&self, node: &mut Gd<Node>, entity: Entity) {
		if !node.has_meta("bevy_events") {
			return;
		}

		let event_data = node.get_meta("bevy_events");
		let d = match Self::convert_event_data_variant_to_rust(event_data) {
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
			let Some(index) = self.string_name_to_index.get(&event) else {
				continue;
			};

			node.connect(
				&signal,
				&self
					.signal_callable
					.as_ref()
					.unwrap()
					.bind(&[(*index as u32).to_variant(), entity.to_godot_variant()]),
			);
		}
	}

	pub(crate) fn get_trigger_callback(&self, index: u32) -> Option<TriggerCallback> {
		self.all_kissing_event_callbacks
			.get(index as usize)
			.map(|c| c.trigger)
	}

	/// Converts the "bevy_components" metadata from a `Variant` to a Rust-digestable representation.
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
}
