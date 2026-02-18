# On Bevy Entity Ready

`BevyEntityReady` is a trait you can implement on a custom gdext Node to run a function immediately after its Bevy entity is created. You can add components and do other things to it.

```rust,noplayground
# use godot::prelude::*;
# use godot::classes::Sprite2D;
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# 
#[derive(GodotClass)]
#[class(init, base=Sprite2D)]
struct Player {
    base: Base<Sprite2D>,

    #[export(range = (0, 100))]
    initial_hp: u32,
}

#[derive(Component)]
struct Health {
	hp: u32,
}

// MAKE SURE YOU ADD #[godot_dyn]
#[godot_dyn]
impl BevyEntityReady for Player {
	fn bevy_entity_ready<'a>(&mut self, entity: EntityWorldMut<'a>) {
		// It'd probably be better to do this as a kissing component,
		// but this is just an example of how it works so shut up.
		entity.insert(Health {
			hp: self.initial_hp
		});
	}
}
```
