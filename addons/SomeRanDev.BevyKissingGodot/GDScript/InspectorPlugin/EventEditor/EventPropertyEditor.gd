@tool
class_name BKGEventEditorProperty extends BKGEditorProperty

# ---

const EVENT_DIALOG = preload("res://addons/SomeRanDev.BevyKissingGodot/Scenes/EventEditor/EventDialog.tscn");

# ---

func get_dialog_scene() -> PackedScene:
	return EVENT_DIALOG;

func get_meta_storage_name() -> String:
	return "bevy_events";

func update_item_from_data(item: TreeItem, data: Dictionary) -> void:
	item.set_text(0, data.get("signal") + " → " + data.get("event"));
