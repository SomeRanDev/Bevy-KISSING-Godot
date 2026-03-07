use bevy::ecs::schedule::ScheduleLabel;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Process;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PhysicsProcess;

#[cfg(feature = "input")]
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GodotInput;
