use bevy::prelude::*;
use godot::prelude::*;

/// The node that receives and stores info from `SceneTree` events.
#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct TreeResponder {
	base: Base<Node>,

	added_nodes: Vec<Gd<Node>>,
	removed_nodes: Vec<InstanceId>,
}

impl TreeResponder {
	pub(crate) fn on_node_added(&mut self, node_added: Gd<Node>) {
		self.added_nodes.push(node_added);
	}

	pub(crate) fn on_node_removed(&mut self, node_removed: Gd<Node>) {
		self.removed_nodes.push(node_removed.instance_id());
	}

	pub(crate) fn take_added_nodes(&mut self) -> Vec<Gd<Node>> {
		std::mem::take(&mut self.added_nodes)
	}

	pub(crate) fn take_removed_nodes(&mut self) -> Vec<InstanceId> {
		std::mem::take(&mut self.removed_nodes)
	}
}

inventory::submit! {
	crate::kissing_node::kissing_node::KissingNode::new(
		"TreeResponder",
		|world, entity| crate::kissing_node::kissing_node::KissingNode::create_entity_with_godot_node_class_components::<TreeResponder>(world, entity),
	)
}
