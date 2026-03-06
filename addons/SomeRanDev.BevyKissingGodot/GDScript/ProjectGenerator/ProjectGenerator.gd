@tool
class_name BKGProjectGenerator extends Node

# ---

const PROJECT_GENERATOR_DIALOG = preload("res://addons/SomeRanDev.BevyKissingGodot/Scenes/ProjectGenerator/ProjectGeneratorDialog.tscn");

# ---

var plugin: BKGEditorPlugin;
var dialog: BKGProjectGeneratorDialog;

# ---

## Opens the project generation dialog.
func open_generate_project_dialog(plugin: BKGEditorPlugin) -> void:
	self.plugin = plugin;

	ensure_dialog_exists();
	dialog.popup_centered();
	dialog.on_open();

## If [param dialog] does not exist, creates it.
func ensure_dialog_exists() -> void:
	if dialog != null:
		return;

	dialog = PROJECT_GENERATOR_DIALOG.instantiate();
	dialog.on_ok.connect(generate_project);
	get_window().add_child(dialog);
	dialog.hide();

func generate_project(
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
	gdextension_min_version_major: int,
	gdextension_min_version_minor: int,
	godot_crate_version: BKGCrateVersionSelector.BKGCrateVersion,
	bevy_crate_version: BKGCrateVersionSelector.BKGCrateVersion,
	bkg_crate_version: BKGCrateVersionSelector.BKGCrateVersion,
	generate_macro_package: bool,
	macro_crate_name: String,
	macro_crate_folder_name: String,
	generate_config_toml: bool,
	godot_editor_path: String,
	clang_path: String,
) -> void:
	print_rich("[font_size=22][b]Generating Rust Project for Bevy💋Godot[/b][/font_size]");

	var project_global_path = ProjectSettings.globalize_path("res://");

	var FORMAT_VARS = {
		"RUST_PROJECT_LOCATION": rust_project_location,
		"EXTENSION_STRUCT_NAME": extension_struct_name,
		"ENTRY_SYMBOL_KIND": ("EntrySymbol::Custom(\"" + entry_symbol + "\".to_string())") if !entry_symbol.is_empty() else "EntrySymbol::GodotRustDefault",
		"GDEXTENSION_ATTRIBUTE_PARAMETERS": "(entry_symbol = " + entry_symbol + ")" if !entry_symbol.is_empty() else "",
		"GAME_CRATE_NAME": game_crate_name,
		"GAME_CRATE_FOLDER_NAME": game_crate_folder_name,
		"AUTOLOAD_LOCATION": autoload_location,
		"AUTOLOAD_RUST_NAME": autoload_rust_name,
		"AUTOLOAD_GDSCRIPT_NAME": autoload_gdscript_name,
		"USE_GDEXT_GEN": automatically_generate_gdextension,
		"GDEXTENSION_LOCATION": BKGUtils.get_relative_path(
			ProjectSettings.globalize_path(gdextension_location),
			project_global_path.path_join(rust_project_location).path_join(game_crate_folder_name).simplify_path()
		), # .gdextension path relative to Game crate directory
		"GDEXTENSION_MIN_VERSION_MAJOR": gdextension_min_version_major,
		"GDEXTENSION_MIN_VERSION_MINOR": gdextension_min_version_minor,
		"GODOT_CRATE_VERSION_KIND": godot_crate_version.kind_as_string(),
		"GODOT_CRATE_VERSION": godot_crate_version.value,
		"BEVY_CRATE_VERSION_KIND": bevy_crate_version.kind_as_string(),
		"BEVY_CRATE_VERSION": bevy_crate_version.value,
		"BKG_CRATE_VERSION_KIND": bkg_crate_version.kind_as_string(),
		"BKG_CRATE_VERSION": bkg_crate_version.value,
		"MACRO_DEPENDENCY": "\nmacros = { path = \"../" + macro_crate_folder_name + "\" }" if generate_macro_package else "",
		"GENERATE_MACRO_PACKAGE": generate_macro_package,
		"MACRO_CRATE_NAME": macro_crate_name,
		"MACRO_CRATE_FOLDER_NAME": macro_crate_folder_name,
		"GENERATE_CONFIG_TOML": generate_config_toml,
		"GODOT_EDITOR_PATH": godot_editor_path.replace("\\", "/"),
		"LIBCLANG_PATH": clang_path.replace("\\", "/"),
		"TARGET_DIR": BKGUtils.get_relative_path(
			rust_project_location.path_join("target"),
			project_global_path
		), # target directory relative to Godot project
	};

	var rust_project_global_path = project_global_path.path_join(rust_project_location).simplify_path();
	var rust_project_dir_access = DirAccess.open(rust_project_global_path);

	# Iterate and generate all Rust-related files for the project.
	for project_file: BKGProjectTemplateFiles.ProjectFile in BKGProjectTemplateFiles.files:
		if !project_file.file_condition.is_empty():
			if !FORMAT_VARS.get(project_file.file_condition):
				continue;

		var file_content = project_file.file_content;
		var file_global_path = rust_project_global_path.path_join(project_file.file_path.format(FORMAT_VARS));

		# Make sure directories exist!
		if !rust_project_dir_access.dir_exists(project_file.file_path.get_base_dir()):
			if DirAccess.make_dir_recursive_absolute(file_global_path.get_base_dir()) != OK:
				push_error("Could not make directories for " + file_global_path);
				continue;

		# Write to file
		if !save_to_file(file_global_path, file_content.format(FORMAT_VARS), project_global_path):
			continue;

	# Generate autoload GDScript file.
	var autoload_content = "extends " + autoload_rust_name;
	if save_to_file(ProjectSettings.globalize_path(autoload_location), autoload_content):
		print("Generated " + ProjectSettings.localize_path(autoload_location));

		print("");
		print_rich("[color=light_green]NOTE! Adding [b]" + ProjectSettings.localize_path(autoload_location) + "[/b] as an autoload for the project, but this will result in a couple [b]errors[/b] below. These errors will persist until the Rust code is compiled![/color]");

		var autoload_name = autoload_gdscript_name;
		if autoload_name.is_empty(): autoload_name = autoload_rust_name + "GD";
		plugin.add_autoload_singleton(autoload_name, autoload_location);

		print_rich("[color=light_green]Okay, expected errors should end here.[/color]");

func save_to_file(path: String, content: String, print_generated_path_relative_to: String = "") -> bool:
	var file = FileAccess.open(path, FileAccess.WRITE);
	if file == null:
		push_error("Could not open file for writing " + path);
		return false;
	file.store_string(content + "\n");
	file.close();
	if !print_generated_path_relative_to.is_empty():
		print("Generated " + BKGUtils.get_relative_path(path, print_generated_path_relative_to));
	return true;
