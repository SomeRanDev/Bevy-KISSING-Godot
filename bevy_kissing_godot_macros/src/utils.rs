use syn::{GenericArgument, Ident, PathArguments, Type};

/// Given an identifier to a "kissing" component struct, return the name of the Godot Object class
/// used in the editor to generate the component data UI.
pub(crate) fn generate_godot_object_name_for_kissing_component_data(
	original_ident: &Ident,
) -> String {
	format!("{}_KissingDataObject", original_ident)
}

/// Checks if the type is `Option<GodotNodeId>`.
///
/// This implementation is flawed as it only checks the final identifier for both `Option` and
/// `GodotNodeId`, so `something::Option<whatever::GodotNodeId>` will return `true`.
pub(crate) fn is_option_godot_node_id(ty: &Type) -> bool {
	let Type::Path(type_path) = ty else {
		return false;
	};
	let Some(segment) = type_path.path.segments.last() else {
		return false;
	};
	if segment.ident != "Option" {
		return false;
	}
	let PathArguments::AngleBracketed(option_type_args) = &segment.arguments else {
		return false;
	};
	if option_type_args.args.len() != 1 {
		return false;
	}
	let GenericArgument::Type(inner_ty) = &option_type_args.args[0] else {
		return false;
	};
	let Type::Path(inner_path) = inner_ty else {
		return false;
	};
	let Some(path_segment) = inner_path.path.segments.last() else {
		return false;
	};
	path_segment.ident == "GodotNodeId"
}
