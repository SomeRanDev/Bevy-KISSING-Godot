@tool
class_name BKGFileDialogButton extends Button

@export var line_edit: LineEdit;
@export var is_directory: bool;
@export var is_save: bool;
@export var project_only: bool;
@export var filter: String;
@export var relative_to_project: bool;

var file_dialog: EditorFileDialog;

func _ready() -> void:
	pressed.connect(on_pressed);

func on_pressed() -> void:
	ensure_file_dialog();
	file_dialog.popup_centered();

func ensure_file_dialog() -> void:
	if file_dialog:
		return;

	file_dialog = EditorFileDialog.new();
	file_dialog.file_mode = FileDialog.FILE_MODE_OPEN_DIR if is_directory else (FileDialog.FILE_MODE_SAVE_FILE if is_save else FileDialog.FILE_MODE_OPEN_FILE);
	file_dialog.access = FileDialog.ACCESS_RESOURCES if project_only else FileDialog.ACCESS_FILESYSTEM;
	if filter:
		file_dialog.add_filter(filter);
	if is_directory:
		file_dialog.dir_selected.connect(on_path_selected);
	else:
		file_dialog.file_selected.connect(on_path_selected);
	add_child(file_dialog);

func on_path_selected(path: String) -> void:
	line_edit.text = "";
	if relative_to_project:
		path = BKGUtils.get_relative_path(path, ProjectSettings.globalize_path("res://"));
	line_edit.text = path;
