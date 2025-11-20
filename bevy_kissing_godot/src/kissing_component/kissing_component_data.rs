use godot::prelude::*;

/// A structure containing the data for a "kissing" component.
pub struct KissingComponentData {
	pub name: &'static str,
	pub docs: &'static str,
	pub fields: Vec<KissingComponentFieldData>,
}

pub struct KissingComponentFieldData {
	pub name: &'static str,
	pub type_string: &'static str,
	pub description: Option<&'static str>,
	pub default_value: Option<String>,
}

impl KissingComponentFieldData {
	pub fn to_dictionary(&self) -> Dictionary {
		vdict! {
			"name": self.name,
			"type_string": self.type_string,
			"description": self.description.unwrap_or_default(),
			"default_value": self.default_value.clone().unwrap_or_default(),
		}
	}
}
