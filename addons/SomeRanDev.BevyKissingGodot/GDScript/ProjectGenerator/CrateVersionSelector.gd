@tool
class_name BKGCrateVersionSelector extends HBoxContainer

enum BKGCrateVersionKind {
	Version = 0,
	Git = 1,
	Path = 2,
}

class BKGCrateVersion extends RefCounted:
	var kind: BKGCrateVersionKind;
	var value: String;
	
	func _init(kind: BKGCrateVersionKind, value: String):
		self.kind = kind;
		self.value = value;
	
	func kind_as_string() -> String:
		match self.kind:
			BKGCrateVersionKind.Version:
				return "version";
			BKGCrateVersionKind.Git:
				return "git";
			BKGCrateVersionKind.Path:
				return "path";
		return "version";

@export var default_kind_index: BKGCrateVersionKind;
@export var default_version: String = "1.0.0";
@export var default_git: String = "";
@export var default_path: String = "";

@onready var kind: OptionButton = $Kind;
@onready var text: LineEdit = $TextContainer/Text;
@onready var file_dialog_button: BKGFileDialogButton = $TextContainer/FileDialogButton;

func _ready() -> void:
	kind.item_selected.connect(on_kind_item_selected);
	kind.select(default_kind_index);
	on_kind_item_selected(default_kind_index);

func get_value() -> BKGCrateVersion:
	return BKGCrateVersion.new(self.kind.get_selected_id(), self.text.text);

func on_kind_item_selected(index: int) -> void:
	file_dialog_button.visible = index == 2;

	match index:
		0:
			text.text = default_version;
		1:
			text.text = default_git;
		2:
			text.text = default_path;
