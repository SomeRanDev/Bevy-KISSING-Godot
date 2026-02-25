@tool
@abstract
class_name BKGEditorProperty extends EditorProperty

# ---

@export var bottom_editor: Control;
@export var add_button: Button;
@export var remove_button: Button;
@export var component_list: Tree;

# ---

## The dialog that lists components/events to add.
var dialog: ConfirmationDialog;

## The root of [field component_list] that items should be added to.
var root: TreeItem;

## The node actively being modified by this property.
var modifying_node: Node;

# ---

@abstract
func get_dialog_scene() -> PackedScene;

@abstract
func get_meta_storage_name() -> String;

@abstract
func update_item_from_data(item: TreeItem, data: Dictionary) -> void;

# ---

## Called after initialization but before [func _ready].
## Provides the [Node] that is modified by this editor property.
func setup(node: Node) -> void:
	modifying_node = node;

## Ready.
func _ready() -> void:
	set_bottom_editor(bottom_editor)

	add_button.icon = get_theme_icon("Add", "EditorIcons");
	remove_button.icon = get_theme_icon("Remove", "EditorIcons");

	add_button.pressed.connect(on_add_clicked);
	remove_button.pressed.connect(on_remove_clicked);

	root = component_list.create_item();
	component_list.item_activated.connect(on_component_list_activated);

	if modifying_node != null:
		var meta_name := get_meta_storage_name();
		checked = modifying_node.has_meta(meta_name);

		var components = modifying_node.get_meta(meta_name, []);
		for c in components:
			make_item_from_data(c);

	property_checked.connect(on_checked);
	on_checked("", checked);

## Called when the checkbox (left of the "Bevy Component" label) for this
## editor property is checked.
func on_checked(_property: StringName, checked: bool) -> void:
	bottom_editor.visible = checked;
	add_button.visible = checked;
	remove_button.visible = checked;

	if modifying_node == null:
		return;

	var meta_name := get_meta_storage_name();
	if checked && !modifying_node.has_meta(meta_name):
		modifying_node.set_meta(meta_name, []);
	elif !checked && modifying_node.has_meta(meta_name):
		modifying_node.remove_meta(meta_name);

## If [param dialog] does not exist, creates it.
func ensure_dialog_exists() -> void:
	if dialog != null:
		return;

	dialog = get_dialog_scene().instantiate();
	get_window().add_child(dialog);
	dialog.on_entry_added.connect(on_entry_added);
	dialog.on_entry_edited.connect(on_entry_edited);
	dialog.hide();

## Generates a new [TreeItem] given [param name] and its corresponding
## component data.
func make_item_from_data(data: Dictionary) -> void:
	var item := root.create_child();
	update_item_from_data(item, data);

## Returns a component's property values given its [param index] in the list of
## components in the [member modifying_node]'s "bevy_event" metadata.
func get_data_from_index(index: int) -> Dictionary:
	var meta_name := get_meta_storage_name();
	var bevy_event := modifying_node.get_meta(meta_name, []) as Array;
	if index >= 0 && index < bevy_event.size():
		return bevy_event[index];
	else:
		return {};

## Called when the "Add" button is clicked.
func on_add_clicked() -> void:
	if modifying_node == null:
		return;

	ensure_dialog_exists();
	dialog.popup_centered();
	dialog.on_open(modifying_node, -1, "", {});

## Called when the "Remove" button is clicked.
func on_remove_clicked() -> void:
	var item := component_list.get_selected();
	if item == null:
		return;

	var meta_name := get_meta_storage_name();
	var index := item.get_index();
	var bevy_event := modifying_node.get_meta(meta_name, []) as Array;
	if index >= 0 && index < bevy_event.size():
		bevy_event.remove_at(index);
		root.remove_child(item);
		item.free();

## Called when an item is double clicked so it may be edited.
func on_component_list_activated() -> void:
	var item := component_list.get_selected();
	if item == null:
		return;

	ensure_dialog_exists();
	dialog.popup_centered();
	dialog.on_open(
		modifying_node,
		item.get_index(),
		item.get_text(0),
		get_data_from_index(item.get_index())
	);

## Connected to [member dialog]'s [signal BKGAddDialog.on_component_added].
func on_entry_added(new_data: Dictionary) -> void:
	var meta_name := get_meta_storage_name();
	var bevy_event := modifying_node.get_meta(meta_name, []) as Array;
	bevy_event.push_back(new_data);
	modifying_node.set_meta(meta_name, bevy_event);

	make_item_from_data(new_data);

## Connected to [member dialog]'s [signal BKGAddDialog.on_component_edited].
func on_entry_edited(index: int, new_data: Dictionary) -> void:
	var meta_name := get_meta_storage_name();
	var bevy_event := modifying_node.get_meta(meta_name, []) as Array;
	if index >= 0 && index < bevy_event.size():
		bevy_event[index] = new_data;
		modifying_node.set_meta(meta_name, bevy_event);
		
		var item = root.get_child(index);
		if item:
			update_item_from_data(item, new_data);
