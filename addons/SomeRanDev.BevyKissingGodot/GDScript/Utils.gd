class_name BKGUtils extends Object

## Splits a string by the contents of a regex.
static func split_by_regex(string: String, regex: RegEx) -> PackedStringArray:
	var result = PackedStringArray();
	var index = 0;
	for regex_match in regex.search_all(string):
		var previous_content = regex_match.subject.substr(index, regex_match.get_start() - index);
		index = regex_match.get_end();
		result.push_back(previous_content);
	if index < string.length():
		result.push_back(string.substr(index, string.length() - index));
	return result;

## Get relative file path of [param path] to [param relative_to].
static func get_relative_path(path: String, relative_to: String) -> String:
	# Parse the "parts" of both paths.
	var any_slash_regex = RegEx.create_from_string("[/\\\\]");
	var path_parts = split_by_regex(path, any_slash_regex);
	var relative_to_parts = split_by_regex(relative_to, any_slash_regex);

	# Count the number of common ancestors.
	var common_ancestor_count = 0;
	while (
		common_ancestor_count < path_parts.size() &&
		common_ancestor_count < relative_to_parts.size() &&
		path_parts[common_ancestor_count] == relative_to_parts[common_ancestor_count]
	):
		common_ancestor_count += 1;

	# If nothing in common, just return the original path.
	if common_ancestor_count == 0:
		return path;

	# Add ".." for each directory up to the common ancestor.
	var relative_path_parts = []
	for i in range(common_ancestor_count, relative_to_parts.size()):
		relative_path_parts.push_back("..")

	# Add the remaining parts of the destination path.
	for i in range(common_ancestor_count, path_parts.size()):
		relative_path_parts.push_back(path_parts[i])

	# Return the final path.
	return "/".join(relative_path_parts)
