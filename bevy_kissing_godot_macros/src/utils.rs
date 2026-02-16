use syn::{Field, GenericArgument, Ident, PathArguments, Type};

#[derive(PartialEq)]
pub(crate) enum NodeOrResource {
	Node,
	Resource,
}

pub(crate) struct IDTypeInfo {
	pub(crate) kind: NodeOrResource,
	pub(crate) is_array: bool,
}

/// Given an identifier to a "kissing" component struct, return the name of the Godot Object class
/// used in the editor to generate the component data UI.
pub(crate) fn generate_godot_object_name_for_kissing_component_data(
	original_ident: &Ident,
) -> String {
	format!("{}_KissingDataObject", original_ident)
}

pub(crate) fn is_field_export(field: &Field) -> bool {
	field.attrs.iter().any(|a| match a.path().get_ident() {
		Some(ident) => matches!(
			ident.to_string().as_str(),
			"export" | "export_node" | "export_resource"
		),
		_ => false,
	})
}

/// Checks if the type is `Option<X>` or `Vec<X>` with `X` either being `GodotNodeId` or `GodotResourceId`.
///
/// This implementation is flawed as it only checks the final identifier for both types,
/// so `something::Option<whatever::GodotNodeId>` will return `true`.
pub(crate) fn is_node_or_resource_id(ty: &Type) -> Option<IDTypeInfo> {
	let Type::Path(type_path) = ty else {
		return None;
	};
	let Some(segment) = type_path.path.segments.last() else {
		return None;
	};

	let is_array = segment.ident == "Vec";
	if !is_array && segment.ident != "Option" {
		return None;
	}

	let PathArguments::AngleBracketed(option_type_args) = &segment.arguments else {
		return None;
	};
	if option_type_args.args.len() != 1 {
		return None;
	}
	let GenericArgument::Type(inner_ty) = &option_type_args.args[0] else {
		return None;
	};
	let Type::Path(inner_path) = inner_ty else {
		return None;
	};
	let Some(path_segment) = inner_path.path.segments.last() else {
		return None;
	};

	let kind = if path_segment.ident == "GodotNodeId" {
		NodeOrResource::Node
	} else if path_segment.ident == "GodotResourceId" {
		NodeOrResource::Resource
	} else {
		return None;
	};

	Some(IDTypeInfo { kind, is_array })
}
