use crate::{
	events::add_child::on_add_child, events::queue_free::on_queue_free,
	events::remove_child::on_remove_child, events::run_code_on_node::on_run_code_on_untyped_node,
	events::set_position_3d::on_set_position_3d, events::set_rotation_3d::on_set_rotation_3d,
	events::set_scale_3d::on_set_scale_3d,
};

use bevy::prelude::*;

/// This enables observers that execute the Bevy💋Godot triggerable events.
///
/// This struct uses a builder pattern that should be used as so:
/// ```
/// #[kiss_bevy(node_name = MyApp)]
/// fn main(app: &mut App) {
///     app.add_plugins(
///         NodeTriggerables::new()
///             .add_remove_child() // Enable AddChild and RemoveChild
///             .queue_free(), // Enable QueueFree
///     );
/// }
/// ```
#[derive(Default)]
pub struct NodeTriggerables {
	add_remove_child: bool,
	queue_free: bool,
	transforms_3d: bool,
	run_code_on_untyped_node: bool,
}

impl NodeTriggerables {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn add_remove_child(mut self) -> Self {
		self.add_remove_child = true;
		self
	}

	pub fn queue_free(mut self) -> Self {
		self.queue_free = true;
		self
	}

	pub fn transforms_3d(mut self) -> Self {
		self.transforms_3d = true;
		self
	}

	pub fn run_code_on_untyped_node(mut self) -> Self {
		self.run_code_on_untyped_node = true;
		self
	}
}

impl Plugin for NodeTriggerables {
	fn build(&self, app: &mut App) {
		if self.add_remove_child {
			app.add_observer(on_add_child).add_observer(on_remove_child);
		}

		if self.queue_free {
			app.add_observer(on_queue_free);
		}

		if self.transforms_3d {
			app.add_observer(on_set_position_3d)
				.add_observer(on_set_rotation_3d)
				.add_observer(on_set_scale_3d);
		}

		if self.run_code_on_untyped_node {
			app.add_observer(on_run_code_on_untyped_node);
		}
	}
}
