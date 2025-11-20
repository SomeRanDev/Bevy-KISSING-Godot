@tool
class_name BKGInspectorPlugin extends EditorInspectorPlugin

const ComponentEditor = preload("res://addons/SomeRanDev.BevyKissingGodot/Scenes/ComponentEditor.tscn");

func _can_handle(object: Object) -> bool:
	return object is Node;

func _parse_begin(object: Object) -> void:
	var control: BKGComponentEditor = ComponentEditor.instantiate();
	control.setup(object);
	add_property_editor("bevy_components", control, false, "Bevy Components");
