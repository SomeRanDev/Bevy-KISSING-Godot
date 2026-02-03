use godot::prelude::*;

/// A structure containing the data for a "kissing" component.
pub struct KissingComponentData {
	pub name: &'static str,
	pub data_class_name: &'static str,
	pub docs: &'static str,
	pub fields: Vec<KissingComponentFieldData>,
}

impl KissingComponentData {
	pub fn to_dictionary(&self) -> VarDictionary {
		vdict! {
			"name": self.name.to_variant(),
			"data_class_name": self.data_class_name.to_variant(),
			"docs": self.docs.to_variant(),
			"fields": self.fields
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
			"name": self.name,
			"type_string": self.type_string,
			"description": self.description.unwrap_or_default(),
		}
	}
}
