mod error;
mod utils;

#[cfg(feature = "node_triggerables")]
pub mod add_child;

#[cfg(feature = "node_triggerables")]
pub mod remove_child;

#[cfg(feature = "node_triggerables")]
pub mod queue_free;

#[cfg(feature = "node_triggerables")]
pub mod set_godot_property;

#[cfg(feature = "node_triggerables")]
pub mod run_code_on_node;

#[cfg(feature = "node_triggerables")]
pub mod run_code_on_node_with_params;

#[cfg(feature = "node_triggerables")]
pub mod set_position_3d;

#[cfg(feature = "node_triggerables")]
pub mod set_rotation_3d;

#[cfg(feature = "node_triggerables")]
pub mod set_scale_3d;
