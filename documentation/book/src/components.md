# Components

## Creating a Bevy component

Let's start with a normal Bevy component.
```rust,noplayground
# use bevy::prelude::*;
# 
#[derive(Component)]
struct Health {
	flags: u32,
	current_hp: u32,
	maximum_hp: u32,
}
```

This is a completely valid and usable component in Bevy💋Godot! Add it to any entity as you like!

## Creating a Kissing component

In Bevy💋Godot, a "kissing" component is a component that is exposed and visible in the Godot editor (it crosses the boundary and kisses Godot).

To make a "kissing" component, just add the `KissingComponent` derive.
```rust,noplayground
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# 
#[derive(Component, KissingComponent)]
struct Health {
	flags: u32,
	current_hp: u32,
	maximum_hp: u32,
}
```

This will appear in the list of components in the Godot editor, but it won't have any fields to edit!!

## Creating a configurable Kissing component

To allow the fields of your kissing component to be editable in Godot, you use the `#[export]` attribute. This attribute works exactly [as it does in gdext](https://godot-rust.github.io/docs/gdext/master/godot/register/derive.GodotClass.html#properties-and-exports) (the attribute is passed verbatim to gdext). This means you can use all the gdext parameters for this attribute as well!

```rust,noplayground
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# 
#[derive(Component, KissingComponent)]
struct Health {
	#[export(enum = (Segmented = 1, Round = 2, Stacked = 3))] // added
	flags: u32,

	#[export(range = (0, 1000))] // added
	maximum_hp: u32,

	current_hp: u32,
}
```

## Setting initial value

To set the initial (and default) value for a property on a kissing component, the `#[initial_value]` attribute can be used. The expression is passed to an `#[init(val = X)]` attribute on the component's editor object.

```rust,noplayground
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# 
#[derive(Component, KissingComponent)]
struct Health {
	#[export(enum = (Segmented = 1, Round = 2, Stacked = 3))]
	#[initial_value = 2] // added
	flags: u32,

	#[export(range = (0, 1000))]
	#[initial_value = 10] // added
	maximum_hp: u32,

	current_hp: u32,
}
```

## Post-constructor

The kissing component is constructed directly from the values provided by the editor. However, you can run a function to modify the component *immediately* after it's constructed. Simply assign a function identifier to `on_construct` as an argument to `#[kissing_component]`.

```rust,noplayground
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# 
#[derive(Component, KissingComponent)]
#[kissing_component(on_construct = health_on_construct)] // added
struct Health {
	#[export(enum = (Segmented = 1, Round = 2, Stacked = 3))]
	#[initial_value = 2]
	flags: u32,

	#[export(range = (0, 1000))]
	#[initial_value = 10]
	maximum_hp: u32,

	current_hp: u32,
}

// The function just takes a mutable reference to the struct.
// Use Bevy's `on_add` if you want to make queries.
fn health_on_construct(component: &mut Health) {
	// We want the hp to start at the maximum value.
	component.current_hp = component.maximum_hp;
}
```
