// ------------------------
// * Top-Level Attributes *
// ------------------------

#![allow(unused_imports)]

// ------------------
// * Normal Exports *
// ------------------

pub use crate::bevy_entity_ready::BevyEntityReady;
pub use crate::components::{godot_node::GodotNode, godot_node_id::GodotNodeId};
pub use crate::plugins::kissing_core_plugin::KissingCorePlugin;
pub use crate::resources::{
	physics_process_delta::PhysicsProcessDelta, process_delta::ProcessDelta,
};
pub use crate::scedules::{PhysicsProcess, Process};
pub use crate::types::{
	GodotNodeQueryUtils, QueryGodotNode, QueryGodotNodeWith, SingleGodotNode, SingleGodotNodeWith,
};

// -----------------
// * Macro Exports *
// -----------------

pub use bevy_kissing_godot_macros::{KissingComponent, KissingNode, kiss_bevy, kiss_node};

// -----------------------------
// * Third-Party Crate Exports *
// -----------------------------

// Macro-generated code requires access to `inventory` crate.
pub use inventory as bevy_kissing_godot_inventory;
