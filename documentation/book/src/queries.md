# Queries

Okay, so now we have components attached to our Godot through the editor, but how do we query for the `Gd<Node>`? That's where we use `GodotNodeId`!

## Accessing `Gd<T>` Node Instance

Every Bevy entity that correlates to a Godot node will have a `GodotNodeId` component. This is just a wrapper for a simple `u32` that is the unique Bevy💋Godot ID for the node. We can convert this ID into a `Gd<T>` reference using the non-send `AllNodes` resource.

```rust,noplayground
# use godot::prelude::*;
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# 
# #[kiss_bevy(PoopPeeKisser)]
# fn main(app: &mut App) {
# 	app.add_scedule(Process, update_health);
# }
# 
# #[derive(Component)]
# struct Health {
# 	value: real,
# }
# 
fn update_health(
	query: Query<(&mut Health, &GodotNodeId)>,
	delta: Res<ProcessDelta>,
	all_nodes: NonSend<AllNodes>,
) {
	for (health, godot_node_id) in query {
		let node_3d = godot_node_id::try_get_as<Node3D>(&all_nodes).unwrap();

		// Decrement health whenever the y position is below zero.
		if node_3d.get_position().y < 0. {
			health.value -= delta.as_real();
		}
	}
}
```

## Filtering Queries by Node Class

We can filter the query by the node's class by using `GodotNode`! The `GodotNode` struct takes a generic argument that you can use to specify the Godot class type.

Note this filter will include all child classes. So a `Node3D` filter will include `MeshInstance3D`s, etc.

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
	query: Query<
		(&Floaty, &GodotNodeId),
		With<GodotNode<CharacterBody3D>> // only CharacterBody3Ds
	>,
	delta: Res<ProcessDelta>,
	all_nodes: NonSend<AllNodes>,
) {
	for (floaty, godot_node_id) in query {
		// This unwrap is guarenteed to succeed cause they're ALL CharacterBody3D.
		let mut character =
			godot_node_id::try_get_as<CharacterBody3D>(&all_nodes).unwrap();

		// Move up a little
		let speed = delta.as_real() * floaty.vertical_speed;
		let old_position = character.get_position();
		character.set_position(old_position + Vector3::UP * speed);
	}
}
```
