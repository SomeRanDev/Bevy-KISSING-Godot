@tool
class_name BKGComponentProperty extends Control

# ---

@export var label: RichTextLabel;
@export var type: Label;
@export var description: Label;
@export var value: LineEdit;

# ---

var property_name: String;

# ---

## Returns the internal name of the component's property.
func get_property_name() -> String:
	return property_name;

## Returns the contents from the text input for this property.
func get_value() -> String:
	return value.text;

## Called after being added as a child to initialize for user interaction.
##
## [param data] should be a [Dictionary] entry from the Bevy components'
## "field" list entry.
##
## The text input is set to [param initial_value] when this is called.
func setup(data: Dictionary, initial_value: String) -> void:
	property_name = data.get("name", "???") as String;

	label.clear();
	label.push_bold();
	label.add_text(property_name.capitalize());

	type.text = data.get("type_string", "???");

	var desc = data.get("description", "").strip_edges();
	description.text = desc;
	description.visible = !desc.is_empty();

	value.text = initial_value;
