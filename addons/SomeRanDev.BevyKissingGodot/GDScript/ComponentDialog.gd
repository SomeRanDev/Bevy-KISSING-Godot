@tool
class_name BKGAddDialog extends ConfirmationDialog

# ---

const PROPERTY = preload("res://addons/SomeRanDev.BevyKissingGodot/Scenes/ComponentProperty.tscn");

# ---

signal on_component_added(component_name: String, data: Dictionary);
signal on_component_edited(index: int, data: Dictionary);

# ---

@export var left: Control;
@export var component_list: Tree;
@export var properties_list: HFlowContainer;
@export var description: RichTextLabel;

@export var style_box: StyleBoxFlat;
@export var properties_style_box: StyleBoxFlat;

# ---

var root: TreeItem;
var components: Array;
var properties: Array[BKGComponentProperty] = [];
var last_id: float = -1.0;
var edit_index: int = -1;

# ---

## Ready.
func _ready() -> void:
	description.set_theme_type_variation("EditorHelpBitContent");

	get_ok_button().connect("pressed", close_and_emit_data);
	component_list.connect("item_activated", close_and_emit_data);
	component_list.connect("item_selected", on_component_selected);

	#style_box.bg_color = get_theme_color("dark_color_1", "Editor");
	#add_theme_stylebox_override("panel", style_box);

	properties_style_box.bg_color = get_theme_color("dark_color_2", "Editor");

## Refreshes the value of [member components] if necessary.
func load_components_if_necessary() -> bool:
	if !ClassDB.class_exists("KissingComponentRegistry"):
		return false;

	var id = ClassDB.class_call_static("KissingComponentRegistry", "get_compilation_id");
	if last_id != id:
		components = ClassDB.class_call_static("KissingComponentRegistry", "find_all_kissing_components");

	return true;

## Called after [method show] to prepare for interaction.
##
## If creating a new component, [param edit_index] should be -1 and the other
## parameters can be anything.
##
## If editing a component, [param edit_index] should be the index of the
## component's data in the edited object's "bevy_components" metadata.
func on_open(edit_index: int, component_name: String, old_data: Dictionary) -> void:
	self.edit_index = edit_index;

	if !load_components_if_necessary():
		return; # Could not load, ignore everything...

	# If we're editing something, hide component list and find the component.
	if edit_index >= 0:
		left.visible = false;
		setup_fields([], {});
		for c in components:
			if component_name == c.get("name"):
				setup_fields(c.get("fields"), old_data);
				break;
		return;

	# If we're adding a new component, make the list's side visible.
	left.visible = true;
	clear_component_list();
	on_component_selected();

	# Generate initial component list options.
	for component in components:
		var item := root.create_child();
		item.set_text(0, component.get("name"));
		item.set_metadata(0, component);

## Removes all elements from [member component_list] execpt the [member root].
func clear_component_list() -> void:
	component_list.clear();
	root = component_list.create_item();

## Generates a [Dictionary] where the keys are the component property names and
## the values are their text values from the inputs on the dialog.
func generate_data() -> Dictionary:
	var result = {};
	for property in properties:
		result.set(
			property.get_property_name(),
			property.get_value()
		);
	return result;

## Closes the dialog and emits the correct signal.
func close_and_emit_data() -> void:
	hide();
	emit_data();

## Emits the correct signal based on whether this is a component edit or 
## creation with the new data.
func emit_data() -> void:
	if edit_index >= 0:
		on_component_edited.emit(edit_index, generate_data());
		return;
	
	var selected = component_list.get_selected();
	if selected == null:
		return;

	var text = selected.get_text(0);
	if text.is_empty():
		return;

	on_component_added.emit(text, generate_data());

## Called when an item in [member component_list] is selected.
## Updates the properties listed on the right side.
func on_component_selected() -> void:
	var selected = component_list.get_selected();
	if selected == null:
		description.text = "";
		setup_fields([], {});
		return;

	var data = selected.get_metadata(0);
	description.text = data.get("docs");
	setup_fields(data.get("fields"), {});

## Generates the "property" list controls for a component given its "fields"
## data.
##
## The initial values for the fields can be provided in [param initial_value],
## with the key being the name of the property and the value being its initial
## value. If [param initial_values] does not contain an entry for a property,
## the [param fields]'s "default_value" will be used instead. 
func setup_fields(fields: Array, initial_values: Dictionary) -> void:
	if !properties.is_empty():
		for p in properties:
			properties_list.remove_child(p);
			p.queue_free();
		properties = [];

	for field in fields:
		var property: BKGComponentProperty = PROPERTY.instantiate();
		properties_list.add_child(property);
		properties.push_back(property);
		var initial_value = initial_values.get(field.get("name"), field.get("default_value", ""));
		property.setup(field, initial_value);
