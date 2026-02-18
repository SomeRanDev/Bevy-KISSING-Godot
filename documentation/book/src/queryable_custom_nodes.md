# Make Your Nodes Queryable

You can still make custom Node classes as you would in gdext, BUT your custom nodes will not be queryable unless you add the `KissingNode` derive.

```rust,noplayground
# use godot::prelude::*;
# use godot::classes::Sprite2D;
# use bevy_kissing_godot::prelude::*;
# 
#[derive(GodotClass, KissingNode)]
#[class(init, base=Sprite2D)]
struct Player {
    base: Base<Sprite2D>,
}
```

This will now work:

```rust,noplayground
# use godot::prelude::*;
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# 
# #[kiss_bevy(PoopPeeKisser)]
# fn main(app: &mut App) {
# 	app.add_scedule(Process, update_float);
# }
# 
# #[derive(Component)]
# struct Floaty {
# 	vertical_speed: real,
# }
# 
fn update_float(
	player: Single<
		(&Floaty, &GodotNodeId),
		With<GodotNode<Player>> // only Player
	>,
	delta: Res<ProcessDelta>,
	all_nodes: NonSend<AllNodes>,
) {
	let (floaty, player_id) = player.into_inner();
	// do stuff
}
```
