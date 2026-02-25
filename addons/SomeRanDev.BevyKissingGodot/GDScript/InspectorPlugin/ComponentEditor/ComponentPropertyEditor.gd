@tool
class_name BKGComponentEditorProperty extends BKGEditorProperty

# ---

const COMPONENT_DIALOG = preload("res://addons/SomeRanDev.BevyKissingGodot/Scenes/ComponentEditor/ComponentDialog.tscn");

# ---

func get_dialog_scene() -> PackedScene:
	return COMPONENT_DIALOG;

func get_meta_storage_name() -> String:
	return "bevy_components";

func update_item_from_data(item: TreeItem, data: Dictionary) -> void:
	item.set_text(0, data.get("name"));
