@tool
class_name BKGEditorPlugin extends EditorPlugin

var inspector_plugin: BKGInspectorPlugin;

func _enter_tree() -> void:
	delete_export_plugin();

	inspector_plugin = BKGInspectorPlugin.new();
	add_inspector_plugin(inspector_plugin);

func _exit_tree() -> void:
	delete_export_plugin();

func delete_export_plugin():
	if inspector_plugin != null:
		remove_inspector_plugin(inspector_plugin);
		inspector_plugin = null; # Inherits RefCounted
