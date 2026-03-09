use crate::kissing_registry::ToGodotDictionary;

use godot::prelude::*;

/// A structure containing the data for a "kissing" event.
pub struct KissingEventData {
	pub name: &'static str,
	pub argument_count: usize,
	pub docs: &'static str,
}

impl ToGodotDictionary for KissingEventData {
	fn to_dictionary(&self) -> VarDictionary {
		vdict! {
			"name" => self.name,
			"argument_count" => self.argument_count as u32,
			"docs" => self.docs,
		}
	}
}
