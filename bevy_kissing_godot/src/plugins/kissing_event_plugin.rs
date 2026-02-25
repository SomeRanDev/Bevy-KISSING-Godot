use crate::{kissing_event::kissing_event::KissingEvent, prelude::*};

use bevy::prelude::*;

/// This plugin must be installed to use [`KissingEvent`]s.
pub struct KissingEventPlugin;

impl Plugin for KissingEventPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Process, update_events);
	}
}

fn update_events(mut commands: Commands) {
	for event in inventory::iter::<KissingEvent> {
		event.run_commands_callback(&mut commands);
	}
}
