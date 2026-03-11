use crate::kissing_event::kissing_event_bridge::KissingEventBridge;
use crate::prelude::*;

use bevy::ecs::world::CommandQueue;
use bevy::prelude::*;
use godot::prelude::*;

/// The node that queues events from Godot signals.
#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct CommandQueueNode {
	base: Base<Node>,
	queue: CommandQueue,

	#[init(val = KissingEventBridge::new())]
	event_bridge: KissingEventBridge,
}

impl CommandQueueNode {
	/// Once a Godot signal that is connected to a kissing event is triggered, this
	/// is called to find and queue the kissing event.
	///
	/// Various [`Variant`] arguments are bound to the [`Callable`] that eventually
	/// leads to this call; the final layout of [`arguments`] is expected to be:
	/// ```
	/// [...signal_arguments, callbacks_index, bevy_entity, kissing_app_node]
	/// ```
	pub(crate) fn on_kissing_signal(&mut self, arguments: &[&Variant]) {
		let arguments_len = arguments.len();
		let Some(entity) = arguments
			.get(arguments_len - 2)
			.and_then(|v| v.to_bevy_entity().ok())
		else {
			return;
		};
		let Some(trigger) = arguments
			.get(arguments_len - 3)
			.and_then(|v| v.try_to::<u32>().ok())
			.and_then(|id| self.event_bridge.get_trigger_callback(id))
		else {
			return;
		};

		(trigger)(&mut self.queue, entity, arguments);
	}

	/// Pushes a Bevy command to the queue.
	pub fn push(&mut self, command: impl Command) {
		self.queue.push(command);
	}

	/// We cannot have an active bind to `Gd<CommandQueueNode>` exist while the commands
	/// are being triggered, so instead the queue is removed and applied elsewhere.
	pub(crate) fn take_queue(&mut self) -> Option<CommandQueue> {
		if self.queue.is_empty() {
			return None;
		}
		Some(std::mem::take(&mut self.queue))
	}

	pub(crate) fn set_signal_callback(&mut self, callable: Callable) {
		self.event_bridge.set_signal_callback(callable);
	}

	pub(crate) fn apply_kissing_events<'a>(&self, node: &mut Gd<Node>, entity: Entity) {
		self.event_bridge.apply_kissing_events(node, entity);
	}
}

inventory::submit! {
	crate::kissing_node::kissing_node::KissingNode::new(
		"CommandQueueNode",
		|world, entity| crate::kissing_node::kissing_node::KissingNode::create_entity_with_godot_node_class_components::<CommandQueueNode>(world, entity),
	)
}
