use crate::prelude::*;

use bevy::prelude::*;

pub struct KissingInputPlugin;

impl Plugin for KissingInputPlugin {
	fn build(&self, app: &mut App) {
		app.add_schedule(Schedule::new(GodotInput));

		app.insert_non_send_resource(InputEventArgument(None));
	}
}
