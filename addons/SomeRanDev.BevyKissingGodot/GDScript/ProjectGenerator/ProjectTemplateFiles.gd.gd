@tool
class_name BKGProjectTemplateFiles extends Object

## Represents a file to be generated for the Rust project.
class ProjectFile extends RefCounted:
	var file_path: String;
	var file_content: String;
	var file_condition: String;

	func _init(path: String):
		self.file_path = path;

	func content(content: String) -> ProjectFile:
		self.file_content = content;
		return self;

	func condition(condition: String) -> ProjectFile:
		self.file_condition = condition;
		return self;

## All the files that may be generated for the Rust project.
static var files: Array[ProjectFile] = [
	ProjectFile
		.new("Cargo.toml")
		.content("[workspace]
resolver = \"3\"
members = [\"{GAME_CRATE_FOLDER_NAME}\"]"),

	ProjectFile
		.new(".gitignore")
		.content("/target"),

	ProjectFile
		.new(".cargo/config.toml")
		.condition("GENERATE_CONFIG_TOML")
		.content("[env]
GODOT4_BIN = \"{GODOT_EDITOR_PATH}\"
LIBCLANG_PATH = \"{LIBCLANG_PATH}\""),

	ProjectFile
		.new("{GAME_CRATE_FOLDER_NAME}/Cargo.toml")
		.content("[package]
name = \"{GAME_CRATE_NAME}\"
version = \"1.0.0\"
edition = \"2024\"

[lib]
crate-type = [\"cdylib\"]

[features]
default = [
	# \"debug\",
	# \"double-precision\",
	# \"multi-threaded\"
]
debug = [\"bevy/debug\"]
double-precision = [\"godot/double-precision\"]
multi-threaded = [\"bevy/multi_threaded\", \"godot/experimental-threads\"]

[dependencies]{MACRO_DEPENDENCY}
godot = { {GODOT_CRATE_VERSION_KIND} = \"{GODOT_CRATE_VERSION}\", features=[\"api-custom\"] }
bevy = { {BEVY_CRATE_VERSION_KIND} = \"{BEVY_CRATE_VERSION}\", default-features = false }
bevy_kissing_godot = { {BKG_CRATE_VERSION_KIND} = \"{BKG_CRATE_VERSION}\" }

[build-dependencies]
gdext-gen = { version = \"0.1.1\" }"),

	ProjectFile
		.new("{GAME_CRATE_FOLDER_NAME}/src/prelude.rs")
		.content("pub(crate) use bevy::prelude::*;
pub(crate) use bevy_kissing_godot::prelude::*;
pub(crate) use godot::prelude::*;"),

	ProjectFile
		.new("{GAME_CRATE_FOLDER_NAME}/src/lib.rs")
		.content("mod app;
mod extension_library;
mod prelude;"),

	ProjectFile
		.new("{GAME_CRATE_FOLDER_NAME}/src/app.rs")
		.content("use bevy::prelude::*;
use bevy_kissing_godot::prelude::*;

#[kiss_bevy({AUTOLOAD_RUST_NAME})]
fn main(app: &mut App) {
	// Do something with app
}"),

	ProjectFile
		.new("{GAME_CRATE_FOLDER_NAME}/src/extension_library.rs")
		.content("use godot::prelude::*;

struct {EXTENSION_STRUCT_NAME};

#[gdextension{GDEXTENSION_ATTRIBUTE_PARAMETERS}]
unsafe impl ExtensionLibrary for {EXTENSION_STRUCT_NAME} {
	fn on_stage_init(_level: InitStage) {}
	fn on_stage_deinit(_level: InitStage) {}
}"),

	ProjectFile
		.new("{GAME_CRATE_FOLDER_NAME}/build.rs")
		.condition("USE_GDEXT_GEN")
		.content("use gdext_gen::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
	generate_gdextension_file(
		BaseDirectory::ProjectFolder.into(),
		Some(\"{TARGET_DIR}\".into()),
		Some(\"{GDEXTENSION_LOCATION}\".into()),
		true,
		Some(Configuration::new(
			{ENTRY_SYMBOL_KIND},
			Some(({GDEXTENSION_MIN_VERSION_MAJOR}, {GDEXTENSION_MIN_VERSION_MINOR})),
			None,
			true,
			false,
		)),
		Some(WindowsABI::MSVC),
		None,
	)?;

	Ok(())
}"),

	ProjectFile
		.new("{MACRO_CRATE_FOLDER_NAME}/Cargo.toml")
		.condition("GENERATE_MACRO_PACKAGE")
		.content("[package]
name = \"{MACRO_CRATE_NAME}\"
version = \"1.0.0\"
edition = \"2024\"

[lib]
proc-macro = true"),

	ProjectFile
		.new("{MACRO_CRATE_FOLDER_NAME}/src/lib.rs")
		.condition("GENERATE_MACRO_PACKAGE")
		.content(""),
];
