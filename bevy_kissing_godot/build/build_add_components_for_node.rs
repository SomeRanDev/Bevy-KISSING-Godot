use std::collections::BTreeMap;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::process::Command;

use serde_json::Map as JsonMap;
use serde_json::Value as JsonValue;
use serde_json::from_str as json_from_str;

// ----------
// * Consts *
// ----------

const OUTPUT_FILE_NAME: &str = "add_components_for_node.rs";

// -------------
// * Functions *
// -------------

/// Generates `OUT_DIR/add_components_for_node.rs`.
pub(crate) fn build_add_components_for_node() {
	let out_dir = env::var_os("OUT_DIR").unwrap();
	let extension_api_path = Path::new(&out_dir).join("extension_api.json");

	// ---
	// Set up rerun conditions

	println!("cargo:rerun-if-env-changed=GODOT4_BIN");
	println!("cargo:rerun-if-env-changed=GODOT_PATH");

	// ---
	// Get Godot path and ensure extension_api.json exists

	let godot_path = get_godot_executable();
	generate_extension_api_json(&godot_path, &out_dir);

	// ---
	// Generate add_components_for_node.rs

	let extension_api_json_content =
		fs::read_to_string(&extension_api_path).expect("Could not read extension_api.json.");
	let extension_api_json =
		json_from_str::<JsonMap<String, JsonValue>>(&extension_api_json_content)
			.expect("Could not parse extension_api.json.");

	let inherit_map = generate_inherit_map(&extension_api_json);
	let (class_name_list, class_inheritance_list_map) = generate_class_data(inherit_map);
	let class_name_hashes = generate_class_name_hashes(&class_name_list, godot_path, &out_dir);
	let entries = generate_cases(
		class_name_list,
		class_name_hashes,
		class_inheritance_list_map,
	);
	generate_file(out_dir, entries);
}

/// Get the user-defined path to the user's Godot executable.
fn get_godot_executable() -> String {
	if let Ok(path) = env::var("GODOT4_BIN") {
		path
	} else if let Ok(path) = env::var("GODOT_PATH") {
		path
	} else {
		println!(
			"cargo:warning=Neither GODOT4_BIN nor GODOT_PATH environment variables are set, so attempting to execute Godot using \"godot\" command. If this fails, please provide a GODOT4_BIN environment variable (this can be done using a \".cargo/config.toml\" file)."
		);
		"godot".to_string()
	}
}

/// Generates OUT_DIR/extension_api.json.
fn generate_extension_api_json(godot_path: &str, out_dir: &OsString) {
	Command::new(godot_path)
		.arg("--headless")
		.arg("--dump-extension-api")
		.arg("--quit")
		.current_dir(out_dir)
		.output()
		.expect("Could not execute Godot to generate API dump.");
}

/// Generates a map of GDScript class names to their super-class names.
/// If the GDScript class does not have a super-class, its value is `None`.
fn generate_inherit_map<'a>(
	extension_api_json: &'a JsonMap<String, JsonValue>,
) -> BTreeMap<&'a str, Option<&'a str>> {
	let classes = extension_api_json
		.get("classes")
		.and_then(|c| c.as_array())
		.expect("Could not find \"classes\" as Array entry in extension_api.json.");

	let mut inherit_map = BTreeMap::<&str, Option<&str>>::new();
	for cls in classes {
		match cls {
			JsonValue::Object(map) => {
				let name: &str = map
					.get("name")
					.and_then(|n| n.as_str())
					.expect("Could not get classes[_].name");
				let inherits = map.get("inherits").and_then(|n| n.as_str());
				inherit_map.insert(name, inherits);
			}
			_ => (),
		}
	}
	inherit_map
}

/// Generates both:
/// * An ordered list of GDScript class names.
/// * A map whose keys are GDScript class names and values are their full inheritance lists.
///
/// Types that are filtered by GDExt are filtered in this function as well.
fn generate_class_data<'a>(
	inherit_map: BTreeMap<&'a str, Option<&'a str>>,
) -> (Vec<&'a str>, BTreeMap<&'a str, Vec<&'a str>>) {
	let mut class_name_list: Vec<&str> = vec![];
	let mut class_inheritance_list_map = BTreeMap::<&str, Vec<&str>>::new();
	for (name, inherits) in &inherit_map {
		if crate::build_utils::godot_codegen::special_cases::is_godot_type_deleted(name) {
			continue;
		}

		let mut inherit_list: Vec<&str> = vec![];
		let mut super_class = inherits.clone();
		while let Some(certain_super_class) = super_class.as_ref() {
			inherit_list.push(certain_super_class);
			super_class = inherit_map.get(certain_super_class).cloned().flatten();
		}
		class_name_list.push(name);
		class_inheritance_list_map.insert(name, inherit_list);
	}
	(class_name_list, class_inheritance_list_map)
}

/// Generates `StringName` hashes for the GDScript class names by executing a
/// GDScript script on the target Godot executable.
fn generate_class_name_hashes(
	class_name_list: &Vec<&str>,
	godot_path: String,
	out_dir: &OsString,
) -> Vec<u64> {
	let gdscript_script_path = Path::new(out_dir).join("generate_class_name_hashes.gd");
	let gdscript_code = format!(
		"extends SceneTree

func _init():
	var ids = [\"💋\"];
{}
	print(\",\".join(ids));
	quit();",
		class_name_list
			.iter()
			.map(|n| format!("\tids.push_back(StringName(\"{}\").hash());", n))
			.collect::<Vec<String>>()
			.join("\n")
	);

	fs::write(&gdscript_script_path, gdscript_code)
		.expect("Could not write to generate_class_name_hashes.gd");

	let output = Command::new(&godot_path)
		.arg("--quit")
		.arg("--headless")
		.arg("--script")
		.arg(gdscript_script_path.as_os_str())
		.current_dir(&out_dir)
		.output()
		.expect("Could not execute Godot to generate API dump.");

	let output = String::from_utf8(output.stdout)
		.expect("Could not get utf8 String from stdout of running generate_class_name_hashes.gd");

	let output = output.trim();

	const PREFIX: &str = "💋,";
	let pos = output
		.find(PREFIX)
		.expect("💋 could not be found in the output of generate_class_name_hashes.gd");
	let output = &output[(pos + PREFIX.len())..];

	output
		.split(",")
		.map(|s| {
			if let Ok(num) = s.parse::<u64>() {
				num
			} else {
				panic!(
					"Could not parse u64 ({}) from StringName hash from output of generate_class_name_hashes.gd",
					s
				);
			}
		})
		.collect::<Vec<u64>>()
}

/// Generates a `String` list of code for the `match` branches.
fn generate_cases(
	class_name_list: Vec<&str>,
	class_name_hashes: Vec<u64>,
	class_inheritance_list_map: BTreeMap<&str, Vec<&str>>,
) -> Vec<String> {
	let mut entries: Vec<String> = vec![];
	for i in 0..class_name_list.len() {
		let name = class_name_list[i];
		let Some(inherits) = class_inheritance_list_map.get(name) else {
			continue;
		};
		let component_construct_code = if inherits.is_empty() {
			format!("crate::prelude::GodotNode::<godot::classes::{}>::default()", name)
		} else {
			format!(
				"({})",
				vec![name]
					.into_iter()
					.chain(inherits.iter().copied())
					.map(|cls| format!(
						"crate::prelude::GodotNode::<godot::classes::{}>::default()",
						crate::build_utils::godot_codegen::conv::to_pascal_case(cls)
					))
					.collect::<Vec<String>>()
					.join(", ")
			)
		};
		entries.push(format!(
			"\n\t\t{} => {{ world.spawn({}).into() }}",
			class_name_hashes[i], &component_construct_code,
		));
	}
	entries
}

/// Generates and writes the Rust code for `OUT_DIR/add_components_for_node.rs`.
fn generate_file(out_dir: OsString, entries: Vec<String>) {
	let dest_path = Path::new(&out_dir).join(OUTPUT_FILE_NAME);
	fs::write(
		&dest_path,
		format!(
			"fn add_components_for_node<'a>(world: &'a mut bevy::prelude::World, node: &godot::prelude::Gd<godot::prelude::Node>) -> Option<bevy::prelude::EntityWorldMut<'a>> {{
	match godot::prelude::StringName::from(&node.get_class()).hash_u32() {{{}
		_ => {{
			let custom_entity = crate::kissing_node::kissing_node::KissingNode::add_godot_editor_components_for_kissing_node(world, node);
			if custom_entity.is_none() {{
				godot_warn!(\"Could not set up Node marker components for `get_class` returning \\\"{{}}\\\"\", node.get_class());
			}}
			custom_entity
		}},
	}}
}}",
			entries.join("")
		),
	)
	.expect("Could not write to add_components_for_node.rs");
}
