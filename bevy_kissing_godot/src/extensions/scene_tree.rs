use crate::{kissing_app::COMMAND_QUEUE_NODE_NAME, nodes::command_queue_node::CommandQueueNode};

use bevy::prelude::*;
use godot::prelude::*;

/// Adds additional functions to SceneTree for easy access in macro-generated code.
pub trait SceneTreeExt {
	/// Pushes a command to be triggered once Bevy's world is free.
	fn push_to_command_queue(&mut self, event: impl Command);
}

impl SceneTreeExt for SceneTree {
	fn push_to_command_queue(&mut self, event: impl Command) {
		let Ok(mut command_queue_node) = self
			.get_meta(COMMAND_QUEUE_NODE_NAME)
			.try_to::<Gd<CommandQueueNode>>()
		else {
			godot::prelude::godot_error!("Failed to execute typed slot for {}", stringify!(#ident));
			return;
		};

		command_queue_node.bind_mut().push(event);
	}
}
