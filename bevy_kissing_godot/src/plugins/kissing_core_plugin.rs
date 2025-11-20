use crate::prelude::*;

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;

pub struct KissingCorePlugin;

impl Plugin for KissingCorePlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<ProcessDelta>();
		app.init_resource::<PhysicsProcessDelta>();

		app.add_schedule(Schedule::new(Process))
			.add_schedule(Schedule::new(PhysicsProcess));
	}
}
