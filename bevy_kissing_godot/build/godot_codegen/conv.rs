/*
 * Copied from:
 * https://github.com/godot-rust/gdext/blob/master/godot-codegen/src/conv/name_conversions.rs
 *
 * These functions are pulled from gdext since they are not accessible normally.
 * I've made an Issue to expose `to_pascal_case`, so maybe I can get rid of this eventually.
 * https://github.com/godot-rust/gdext/issues/1409
 *
 * There may also be a better way to do this?
 */

/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

fn to_snake_special_case(class_name: &str) -> Option<&'static str> {
	match class_name {
		// Classes
		"JSONRPC" => Some("json_rpc"),
		"OpenXRAPIExtension" => Some("open_xr_api_extension"),
		"OpenXRIPBinding" => Some("open_xr_ip_binding"),

		// Enums
		// "SDFGIYScale" => Some("sdfgi_y_scale"),
		_ => None,
	}
}

/// Used for `PascalCase` identifiers: classes and enums.
pub fn to_pascal_case(ty_name: &str) -> String {
	use heck::ToPascalCase;

	assert!(
		is_valid_ident(ty_name),
		"invalid identifier for PascalCase conversion: {ty_name}"
	);

	// Special cases: reuse snake_case impl to ensure at least consistency between those 2.
	if let Some(snake_special) = to_snake_special_case(ty_name) {
		return snake_special.to_pascal_case();
	}

	ty_name
		.to_pascal_case()
		.replace("GdExtension", "GDExtension")
		.replace("GdNative", "GDNative")
		.replace("GdScript", "GDScript")
		.replace("Vsync", "VSync")
		.replace("Sdfgiy", "SdfgiY")
}

/// Check if input is a valid identifier; i.e. no special characters except '_' and not starting with a digit.
fn is_valid_ident(s: &str) -> bool {
	!starts_with_invalid_char(s) && s.chars().all(|c| c == '_' || c.is_ascii_alphanumeric())
}

fn starts_with_invalid_char(s: &str) -> bool {
	s.starts_with(|c: char| c.is_ascii_digit())
}
