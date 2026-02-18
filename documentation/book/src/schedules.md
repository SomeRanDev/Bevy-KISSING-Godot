# Schedules

Bevy💋Godot's supported scedules are a little inconsistent, but here's the breakdown:

## Startup

Bevy💋Godot DOES support `Startup`. It runs immediately after the scene tree is ready, so it basically runs as a `_ready` function for your initial scene.

```rust,noplayground
# use godot::prelude::*;
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# 
#[kiss_bevy(PoopPeeKisser)]
fn main(app: &mut App) {
	app.add_scedule(Startup, on_game_start)
}

fn on_game_start(scene_tree: NonSend<Gd<SceneTree>>) {
	godot_print!("The game has started with {} nodes!", scene_tree.get_node_count());
}
```

## Update

`Update` is not supported in Bevy💋Godot. You should use `Process` instead.

## Process

`Process` runs every "process" frame in Godot. It runs in the `_process` function in your autoload `Node`.

You can use the `ProcessDelta` resource to access the delta value for that process frame.

```rust,noplayground
# use godot::prelude::*;
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# 
#[kiss_bevy(PoopPeeKisser)]
fn main(app: &mut App) {
	app.add_scedule(Process, on_game_update)
}

fn on_game_update(delta: Res<ProcessDelta>) {
	godot_print!("The process frame ran with {} delta.", *delta);
}
```

## PhysicsProcess

`PhysicsProcess` is the same as `Process` except for the `_physics_process` frame.

Similarly, you can use the `PhysicsProcessDelta` resource to access its delta.

```rust,noplayground
# use godot::prelude::*;
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# 
#[kiss_bevy(PoopPeeKisser)]
fn main(app: &mut App) {
	app.add_scedule(PhysicsProcess, on_game_update_physics)
}

fn on_game_update_physics(delta: Res<PhysicsProcessDelta>) {
	godot_print!("The physics process frame ran with {} delta.", *delta);
}
```