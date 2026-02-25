@tool
class_name BKGInspectorPlugin extends EditorInspectorPlugin

const ComponentEditor = preload("res://addons/SomeRanDev.BevyKissingGodot/Scenes/ComponentEditor/ComponentEditorProperty.tscn");
const EventEditor = preload("res://addons/SomeRanDev.BevyKissingGodot/Scenes/EventEditor/EventEditorProperty.tscn");

func _can_handle(object: Object) -> bool:
	return object is Node;

func _parse_begin(object: Object) -> void:
	var components_control: BKGComponentEditorProperty = ComponentEditor.instantiate();
	components_control.setup(object);
	add_property_editor("bevy_components", components_control, false, "Bevy Components");

	var events_control: BKGEventEditorProperty = EventEditor.instantiate();
	events_control.setup(object);
	add_property_editor("bevy_events", events_control, false, "Bevy Events");
