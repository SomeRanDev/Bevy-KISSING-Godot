use godot::prelude::*;

use crate::kissing_registry::ToGodotDictionary;

/// A structure containing the data for a "kissing" component.
pub struct KissingComponentData {
	pub name: &'static str,
	pub data_class_name: &'static str,
	pub docs: &'static str,
	pub fields: Vec<KissingComponentFieldData>,
}

impl ToGodotDictionary for KissingComponentData {
	fn to_dictionary(&self) -> VarDictionary {
		vdict! {
			"name" => self.name,
			"data_class_name" => self.data_class_name,
			"docs" => self.docs,
			"fields" => &self.fields
				.iter()
				.map(|s| s.to_dictionary())
				.collect::<Array<VarDictionary>>(),
		}
	}
}

/// A structure containing the data for a "kissing" component's fields.
pub struct KissingComponentFieldData {
	pub name: &'static str,
	pub type_string: &'static str,
	pub description: Option<&'static str>,
}

impl KissingComponentFieldData {
	pub fn to_dictionary(&self) -> VarDictionary {
		vdict! {
			"name" => self.name,
			"type_string" => self.type_string,
			"description" => self.description.unwrap_or_default(),
		}
	}
}
