// ------------------------
// * Top-Level Attributes *
// ------------------------

#![allow(unused_imports)]

// ------------------
// * Normal Exports *
// ------------------

pub use crate::bevy_entity_ready::BevyEntityReady;
pub use crate::components::{
	gd_tracker_id::GodotNodeId, gd_tracker_id::GodotResourceId, godot_node::GodotNode,
};
pub use crate::extensions::{entity::EntityExt, variant::VariantExt};
pub use crate::kissing_component::kissing_component_field::KissingComponentField;
pub use crate::plugins::kissing_core_plugin::KissingCorePlugin;
pub use crate::resources::{
	gd_handle::GdHandle, gd_tracker::AllNodes, gd_tracker::AllResources,
	godot_thread_ensurer::GodotThreadEnsurer, input_event_argument::InputEventArgument,
	physics_process_delta::PhysicsProcessDelta, process_delta::ProcessDelta,
};
pub use crate::scedules::{PhysicsProcess, Process};
pub use crate::types::{
	GodotNodeQueryUtils, QueryGodotNode, QueryGodotNodeWith, SingleGodotNode, SingleGodotNodeWith,
};

// -----------------------
// * Conditional Exports *
// -----------------------

#[cfg(feature = "input")]
pub use crate::scedules::GodotInput;

#[cfg(feature = "node_triggerables")]
pub use crate::{
	events::{
		add_child::AddChild, queue_free::QueueFree, remove_child::RemoveChild,
		run_code_on_node::RunCodeOnNode, run_code_on_node_with_params::RunCodeOnNodeWithParams,
		set_godot_property::SetGodotProperty, set_position_3d::SetPosition3D,
		set_rotation_3d::SetRotation3D, set_scale_3d::SetScale3D,
	},
	plugins::node_triggerables::NodeTriggerables,
};

// -----------------
// * Macro Exports *
// -----------------

pub use bevy_kissing_godot_macros::{
	KissingComponent, KissingEvent, KissingNode, kiss_bevy, plugin_and_kissing_component,
};

// -----------------------------
// * Third-Party Crate Exports *
// -----------------------------

// Macro-generated code requires access to `inventory` crate.
#[doc(hidden)]
pub use inventory as bevy_kissing_godot_inventory;
