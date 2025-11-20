use bevy::prelude::*;

/// Implement this trait on a custom node with `KissingNode` to access the Bevy entity
/// of the node upon initialization.
pub trait BevyEntityReady {
	fn bevy_entity_ready<'a>(&mut self, entity: EntityWorldMut<'a>);
}
