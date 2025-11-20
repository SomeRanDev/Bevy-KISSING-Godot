@tool
class_name BKGComponentEditor extends EditorProperty

# ---

const COMPONENT_DIALOG = preload("res://addons/SomeRanDev.BevyKissingGodot/Scenes/ComponentDialog.tscn");

# ---

@export var bottom_editor: Control;
@export var add_button: Button;
@export var remove_button: Button;
@export var component_list: Tree;

# ---

## The dialog that lists components to add.
var dialog: BKGAddDialog;

## The root of [field component_list] that items should be added to.
var root: TreeItem;

## The node actively being modified by this property.
var modifying_node: Node;

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

	root = component_list.create_item();
	component_list.item_activated.connect(on_component_list_activated);

	if modifying_node != null:
		checked = modifying_node.has_meta("bevy_components");

		var components = modifying_node.get_meta("bevy_components", []);
		for c in components:
			make_item_from_data(c.get("name"), c.get("data"));

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

	if checked && !modifying_node.has_meta("bevy_components"):
		modifying_node.set_meta("bevy_components", []);
	elif !checked && modifying_node.has_meta("bevy_components"):
		modifying_node.remove_meta("bevy_components");

## If [param dialog] does not exist, creates it.
func ensure_dialog_exists() -> void:
	if dialog != null:
		return;

	dialog = COMPONENT_DIALOG.instantiate();
	get_window().add_child(dialog);
	dialog.on_component_added.connect(on_component_added);
	dialog.on_component_edited.connect(on_component_edited);
	dialog.hide();

## Generates a new [TreeItem] given [param name] and its corresponding
## component data.
func make_item_from_data(name: String, data: Dictionary) -> void:
	var item := root.create_child();
	item.set_text(0, name);

## Returns a component's property values given its [param index] in the list of
## components in the [member modifying_node]'s "bevy_components" metadata.
func get_data_from_index(index: int) -> Dictionary:
	var bevy_components := modifying_node.get_meta("bevy_components", []) as Array;
	if index >= 0 && index < bevy_components.size():
		return bevy_components[index].get("data");
	else:
		return {};

## Called when the "Add" button is clicked.
func on_add_clicked():
	if modifying_node == null:
		return;

	ensure_dialog_exists();
	dialog.popup_centered();
	dialog.on_open(-1, "", {});

## Called when an item is double clicked so it may be edited.
func on_component_list_activated() -> void:
	var item = component_list.get_selected();
	if item == null:
		return;

	ensure_dialog_exists();
	dialog.popup_centered();
	dialog.on_open(item.get_index(), item.get_text(0), get_data_from_index(item.get_index()));

## Connected to [member dialog]'s [signal BKGAddDialog.on_component_added].
func on_component_added(component_name: String, data: Dictionary) -> void:
	var bevy_components := modifying_node.get_meta("bevy_components", []) as Array;
	bevy_components.push_back({
		"name": component_name,
		"data": data
	});
	modifying_node.set_meta("bevy_components", bevy_components);
	
	make_item_from_data(component_name, data);

## Connected to [member dialog]'s [signal BKGAddDialog.on_component_edited].
func on_component_edited(index: int, data: Dictionary) -> void:
	var bevy_components := modifying_node.get_meta("bevy_components", []) as Array;
	if index >= 0 && index < bevy_components.size():
		bevy_components[index].set("data", data);
