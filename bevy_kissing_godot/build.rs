#[path = "build/mod.rs"]
mod build_utils;

fn main() {
	// Generate OUT_DIR/add_components_for_node.rs
	build_utils::build_add_components_for_node::build_add_components_for_node();
}
