use crate::{prelude::*, resources::entity_preregister::EntityPreregister};

use bevy::prelude::*;

pub struct KissingCorePlugin;

impl Plugin for KissingCorePlugin {
	fn build(&self, app: &mut App) {
		app.add_schedule(Schedule::new(bevy::prelude::Startup))
			.add_schedule(Schedule::new(Process))
			.add_schedule(Schedule::new(PhysicsProcess));

		app.init_resource::<ProcessDelta>()
			.init_resource::<PhysicsProcessDelta>();

		app.insert_non_send_resource(AllNodes::default())
			.insert_non_send_resource(AllResources::default())
			.insert_non_send_resource(GodotThreadEnsurer::new())
			.insert_non_send_resource(EntityPreregister::default());
	}
}
