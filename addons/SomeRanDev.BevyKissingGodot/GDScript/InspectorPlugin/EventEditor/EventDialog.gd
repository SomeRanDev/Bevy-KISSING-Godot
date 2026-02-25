@tool
class_name BKGAddEventDialog extends ConfirmationDialog

# ---

signal on_entry_added(component_name: String, data: Dictionary);
signal on_entry_edited(index: int, data: Dictionary);

# ---

@export var left: Control;
@export var right: Control;
@export var signal_list: Tree;
@export var event_list: Tree;
@export var description: RichTextLabel;

# ---

var signals_root: TreeItem;
var events_root: TreeItem;
var events: Array[Dictionary];
var last_id: float = -1.0;
var edit_index: int = -1;
var modifying_node: Node;

# ---

## Ready.
func _ready() -> void:
	description.set_theme_type_variation("EditorHelpBitContent");

	get_ok_button().pressed.connect(close_and_emit_data);
	event_list.item_activated.connect(close_and_emit_data);
	event_list.item_selected.connect(on_event_selected);
	signal_list.item_selected.connect(refresh_ok_enabled);
	event_list.item_selected.connect(refresh_ok_enabled);

func refresh_ok_enabled() -> void:
	get_ok_button().disabled = !(
		signal_list.get_selected() != null &&
		event_list.get_selected() != null
	);

## Refreshes the value of [member events] if necessary.
func load_events_if_necessary() -> bool:
	if !ClassDB.class_exists("KissingRegistry"):
		return false;

	var id = ClassDB.class_call_static("KissingRegistry", "get_compilation_id");
	if last_id != id:
		events = ClassDB.class_call_static("KissingRegistry", "find_all_kissing_events");

	return true;

## Called after [method show] to prepare for interaction.
##
## If creating a new component, [param edit_index] should be -1 and the other
## parameters can be anything.
##
## If editing a component, [param edit_index] should be the index of the
## event's data in the edited object's "bevy_events" metadata.
func on_open(
	modifying_node: Node,
	edit_index: int,
	component_name: String,
	old_data: Dictionary
) -> void:
	self.modifying_node = modifying_node;
	self.edit_index = edit_index;

	if !load_events_if_necessary():
		return; # Could not load, ignore everything...

	clear_signal_list();
	clear_event_list();
	on_event_selected(); # Clear description
	refresh_ok_enabled();

	# Generate list of signals.
	for signal_ in modifying_node.get_signal_list():
		var item := signals_root.create_child();
		var name: String = signal_.get("name");
		item.set_text(0, name);
		item.set_metadata(0, signal_);
		if edit_index >= 0:
			if old_data.get("signal") == name:
				item.select(0);

	# Generate list of events.
	for event in events:
		var item := events_root.create_child();
		var name: String = event.get("name");
		item.set_text(0, name);
		item.set_metadata(0, event);
		if edit_index >= 0:
			if old_data.get("event") == name:
				item.select(0);

## Removes all elements from [member signal_list] execpt the [member signals_root].
func clear_signal_list() -> void:
	signal_list.clear();
	signals_root = signal_list.create_item();

## Removes all elements from [member event_list] execpt the [member events_root].
func clear_event_list() -> void:
	event_list.clear();
	events_root = event_list.create_item();

## Generates a [Dictionary] where the keys are the component property names and
## the values are their text values from the inputs on the dialog.
func generate_data() -> Dictionary:
	var signal_name = signal_list.get_selected().get_text(0);
	var event_name = event_list.get_selected().get_text(0);
	return {
		"signal": signal_name,
		"event": event_name,
	};

## Closes the dialog and emits the correct signal.
func close_and_emit_data() -> void:
	hide();
	emit_data();

## Emits the correct signal based on whether this is a component edit or 
## creation with the new data.
func emit_data() -> void:
	if edit_index >= 0:
		on_entry_edited.emit(edit_index, generate_data());
		return;

	var selected = event_list.get_selected();
	if selected == null:
		return;

	var text = selected.get_text(0);
	if text.is_empty():
		return;

	on_entry_added.emit(generate_data());

## Called when an item in [member component_list] is selected.
## Updates the properties listed on the right side.
func on_event_selected() -> void:
	var selected = event_list.get_selected();
	if selected == null:
		description.text = "";
		return;

	var data = selected.get_metadata(0);
	description.text = data.get("docs");
