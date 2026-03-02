# Intro + Q&A

Bevy💋Godot (_pronounced "Bevy Kissing Godot"_) is a framework that adds Bevy's ECS on top of Godot.

This project is HEAAAVILY inspired by [Godot Bevy](https://github.com/bytemeadow/godot-bevy), and it would almost certainly not exist if it didn't come up with the idea first. On the off chance someone besides me is reading this, I would highly recommend trying that project first.

I chose to create my own library for three major reasons:
 * I want the ability to add Bevy components on Godot nodes in the Godot editor.

 * My lack of knoweldge on the internals of Bevy and Godot-Bevy made me paranoid I may encounter a bug or unexpected behavior I could not fix myself.

 * Godot-Bevy automates a bit too much for me. I want to create my own `ExtensionLibrary`, I want to access Godot nodes via a `NonSend`, and some third thing I'm too lazy to think of but I'm sure there's something idk it's been a couple months since I tinkered with it.

So I decided I could fix these things by creating my own Bevy ECS in Godot project from the ground up.

## Why Bevy ECS?

Because I like using components in gamedev. And I like Rust. Idk it fun.

## Why is it named that?

One *may* assume the name exists to reduce confusion with Godot-Bevy and distinguish this from inevitable future Bevy + Godot projects, but **actually** I don't care about those things at all and it's because I ship two male personifications of Bevy and Godot in a toxic yaoi relationship where they kiss passionately.

## Add Bevy components in the Godot editor?

Using a Godot addon, the Godot's Nodes' inspectors now have a component list you can add your Bevy components to. Simply add the `KissingComponent` derive to a Bevy component to make it available!

You can use `#[export]` to allow properties to be modified in the editor (with the **exact** same options from [gdext](https://godot-rust.github.io/docs/gdext/master/godot/register/derive.GodotClass.html#export-properties--export)!).

Check out the [Components](./components.html) section for more details.

```rust,noplayground
# use godot::prelude::*;
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# 
#[derive(Component, KissingComponent)]
struct Grid {
	#[export]
	#[initial_value = Vector2i::new(10, 10)]
	size: Vector2i,

	#[export(range = (0., 10.))]
	#[initial_value = 2.]
	spacing: real,

	#[export(enum = (Orthographic, Isometric))]
	kind: i32,
}
```

## Connect Godot signals to Bevy events in the Godot editor?

Using the same Godot addon, Godot nodes can ALSO have their signals connected to Bevy events. Just derive from `KissingEvent` and the event will become available in the Godot editor!

Check out the [Events](./events.html) section for more details.

```rust,noplayground
# use godot::prelude::*;
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# 
/// A unit Event can connect to any signal.
#[derive(Event, KissingEvent)]
struct OnAnyGodotSignal;

/// Event intended to connect to Button.toggled
#[derive(EntityEvent, KissingEvent)]
struct OnMyButtonPressed {
	// Receive entity for Button node.
	#[event_target]
	entity: Entity,

	// Receive first argument of signal; it must be a bool.
	#[godot_signal_arg(0)]
	toggled_on: bool,
}
```

## Multithreading?

Multithreading is supported as it is in Bevy, but only if you write functions that don't access `NonSend` resources. Accessing Godot's scene tree, all Godot nodes, and all Godot resources requires `NonSend`, so multithreading is only applicable to behavior and state stored entirely Rust-side.
