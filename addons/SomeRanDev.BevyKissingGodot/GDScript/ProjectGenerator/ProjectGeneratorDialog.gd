@tool
class_name BKGProjectGeneratorDialog extends ConfirmationDialog

signal on_ok(
	rust_project_location: String,
	extension_struct_name: String,
	entry_symbol: String,
	game_crate_name: String,
	game_crate_folder_name: String,
	autoload_location: String,
	autoload_rust_name: String,
	autoload_gdscript_name: String,
	automatically_generate_gdextension: bool,
	gdextension_location: String,
	godot_crate_version: BKGCrateVersionSelector.BKGCrateVersion,
	bevy_crate_version: BKGCrateVersionSelector.BKGCrateVersion,
	bkg_crate_version: BKGCrateVersionSelector.BKGCrateVersion,
	generate_macro_package: bool,
	macro_crate_name: String,
	macro_crate_folder_name: String,
	generate_config_toml: bool,
	godot_editor_path: String,
	clang_path: String,
);

@export var rust_project_location: LineEdit;
@export var extension_struct_name: LineEdit;
@export var entry_symbol: LineEdit;
@export var game_crate_name: LineEdit;
@export var game_crate_folder_name: LineEdit;
@export var autoload_location: LineEdit;
@export var autoload_rust_name: LineEdit;
@export var autoload_gdscript_name: LineEdit;
@export var automatically_generate_gdextension: CheckBox;
@export var gdextension_location: LineEdit;
@export var gdextension_min_version_major: SpinBox;
@export var gdextension_min_version_minor: SpinBox;
@export var godot_crate_version: BKGCrateVersionSelector;
@export var bevy_crate_version: BKGCrateVersionSelector;
@export var bkg_crate_version: BKGCrateVersionSelector;
@export var generate_macro_package: CheckBox;
@export var macro_crate_name: LineEdit;
@export var macro_crate_folder_name: LineEdit;
@export var generate_config_toml: CheckBox;
@export var godot_editor_path: LineEdit;
@export var clang_path: LineEdit;
@export var error: Label;

var enable_ok_refreshing = true;

## Ready
func _ready() -> void:
	rust_project_location.text_changed.connect(refresh_ok_button);
	extension_struct_name.text_changed.connect(refresh_ok_button);
	game_crate_name.text_changed.connect(refresh_ok_button);
	game_crate_folder_name.text_changed.connect(refresh_ok_button);
	autoload_location.text_changed.connect(refresh_ok_button);
	autoload_rust_name.text_changed.connect(refresh_ok_button);
	automatically_generate_gdextension.toggled.connect(refresh_ok_button);
	gdextension_location.text_changed.connect(refresh_ok_button);
	gdextension_min_version_major.value_changed.connect(refresh_ok_button);
	gdextension_min_version_minor.value_changed.connect(refresh_ok_button);
	generate_macro_package.toggled.connect(refresh_ok_button);
	macro_crate_name.text_changed.connect(refresh_ok_button);
	macro_crate_folder_name.text_changed.connect(refresh_ok_button);
	generate_config_toml.toggled.connect(refresh_ok_button);
	godot_editor_path.text_changed.connect(refresh_ok_button);
	clang_path.text_changed.connect(refresh_ok_button);

	get_ok_button().pressed.connect(on_ok_pressed);

## This function should be called every time this window is opened.
func on_open() -> void:
	enable_ok_refreshing = false;

	rust_project_location.text = "../RustCode";
	extension_struct_name.text = "MyGameExtension";
	entry_symbol.text = "";
	game_crate_name.text = "my_game";
	game_crate_folder_name.text = "Game";
	autoload_location.text = "res://MyGameApp.gd";
	autoload_rust_name.text = "MyGameApp";
	autoload_gdscript_name.text = "MyGameAppGD";
	automatically_generate_gdextension.button_pressed = true;
	gdextension_location.text = "res://MyGame.gdextension";
	var version_info = Engine.get_version_info();
	gdextension_min_version_major.value = version_info["major"];
	gdextension_min_version_minor.value = version_info["minor"];
	generate_macro_package.button_pressed = true;
	macro_crate_name.text = "macros";
	macro_crate_folder_name.text = "Macros";
	generate_config_toml.button_pressed = false;
	godot_editor_path.text = OS.get_executable_path();
	clang_path.text = "";

	enable_ok_refreshing = true;
	refresh_ok_button();

## Called when the "ok" button is pressed.
func on_ok_pressed() -> void:
	if update_error_and_is_error_free():
		hide();
		on_ok.emit(
			rust_project_location.text,
			extension_struct_name.text,
			entry_symbol.text,
			game_crate_name.text,
			game_crate_folder_name.text,
			autoload_location.text,
			autoload_rust_name.text,
			autoload_gdscript_name.text,
			automatically_generate_gdextension.button_pressed,
			gdextension_location.text,
			gdextension_min_version_major.value,
			gdextension_min_version_minor.value,
			godot_crate_version.get_value(),
			bevy_crate_version.get_value(),
			bkg_crate_version.get_value(),
			generate_macro_package.button_pressed,
			macro_crate_name.text,
			macro_crate_folder_name.text,
			generate_config_toml.button_pressed,
			godot_editor_path.text,
			clang_path.text
		);

## Updates the error text and disables the "ok" button if there's any errors.
func refresh_ok_button(_ignored_param: Variant = null) -> void:
	if !enable_ok_refreshing: return;
	get_ok_button().disabled = !update_error_and_is_error_free();

## If an error exists, updates the text of [member error] and returns [true].
## Otherwise, returns [false].
func update_error_and_is_error_free() -> bool:
	if (
		rust_project_location.text.is_empty() ||
		extension_struct_name.text.is_empty() ||
		game_crate_name.text.is_empty() ||
		game_crate_folder_name.text.is_empty() ||
		autoload_location.text.is_empty() ||
		autoload_gdscript_name.text.is_empty() ||
		autoload_rust_name.text.is_empty() ||
		(
			automatically_generate_gdextension.button_pressed &&
			gdextension_location.text.is_empty()
		) ||
		(
			generate_macro_package.button_pressed &&
			(
				macro_crate_name.text.is_empty() ||
				macro_crate_folder_name.text.is_empty()
			)
		) ||
		(
			generate_config_toml.button_pressed &&
			(
				godot_editor_path.text.is_empty() ||
				clang_path.text.is_empty()
			)
		)
	):
		error.text = "Missing required input.";
		return false;

	var project_location = ProjectSettings.globalize_path(".");

	var project_location_absolute_path = project_location.path_join(rust_project_location.text);
	var project_location_dir_access = DirAccess.open(project_location_absolute_path);
	if !project_location_dir_access:
		error.text = "Rust project location does not exist.";
		return false;

	if !exists_in_project_with_extension(autoload_location.text, "gd"):
		error.text = "Autoload GDScript file must be in the project AND use .gd file extension.";
		return false;

	if (
		automatically_generate_gdextension.button_pressed &&
		!exists_in_project_with_extension(gdextension_location.text, "gdextension")
	):
		error.text = ".gdextension file must be in the project AND use .gdextension file extension.";
		return false;

	if (
		generate_config_toml.button_pressed &&
		!FileAccess.file_exists(godot_editor_path.text)
	):
		error.text = "Godot Editor executable does not exist.";
		return false;

	if (
		generate_config_toml.button_pressed &&
		!DirAccess.open(clang_path.text)
	):
		error.text = "Clang folder does not exist.";
		return false;

	error.text = "";
	return true;

## Checks if the file at path [param path] exists in the project and has
## the extension [param extension].
static func exists_in_project_with_extension(path: String, extension: String) -> bool:
	var project_path = ProjectSettings.globalize_path(".");
	var local_path = ProjectSettings.localize_path(ProjectSettings.globalize_path(path));
	return (
		local_path.to_lower().begins_with("res://") &&
		local_path.get_extension() == extension
	);
