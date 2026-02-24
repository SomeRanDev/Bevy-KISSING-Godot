@tool
class_name BKGEditorPlugin extends EditorPlugin

const GENERATE_PROJECT_TOOL_MENU_TEXT = "Generate Bevy💋Godot Rust Project";

var inspector_plugin: BKGInspectorPlugin;
var project_generator: BKGProjectGenerator;

func _enter_tree() -> void:
	delete_export_plugin();
	inspector_plugin = BKGInspectorPlugin.new();
	add_inspector_plugin(inspector_plugin);

	delete_project_generator();
	project_generator = BKGProjectGenerator.new();
	add_child(project_generator);
	add_tool_menu_item(GENERATE_PROJECT_TOOL_MENU_TEXT, open_generate_project_dialog);

func _exit_tree() -> void:
	delete_export_plugin();
	delete_project_generator();
	remove_tool_menu_item(GENERATE_PROJECT_TOOL_MENU_TEXT);

func open_generate_project_dialog() -> void:
	project_generator.open_generate_project_dialog(self);

func delete_export_plugin() -> void:
	if inspector_plugin != null:
		remove_inspector_plugin(inspector_plugin);
		inspector_plugin = null; # Inherits RefCounted

func delete_project_generator() -> void:
	if project_generator != null:
		remove_child(project_generator);
		project_generator.queue_free();
		project_generator = null;
