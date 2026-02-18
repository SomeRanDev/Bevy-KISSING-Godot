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

## Bevy components in the Godot editor???

Using a Godot addon, the Godot's Nodes' inspectors now have a component list you can add your Bevy components to. Simply add the `#[kissing_component]` attribute and `KissingComponent` derive to a Bevy component to make it available!

You can use `#[export]` to allow properties to be modified in the editor.

Check out the [Components](./components.html) section for more details.

```rust,noplayground
# use godot::prelude::*;
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# 
#[derive(Component, KissingComponent)]
#[kissing_component]
struct Grid {
	#[export(initial_value = Vector2i::new(10, 10))]
	count: Vector2i,

	#[export(initial_value = 2.)]
	spacing: real,

	#[export]
	offset: Vector2i,
}
```


## Multithreading?

Multithreading is supported as it is in Bevy, but only if you write functions that don't access `NonSend` resources. Accessing Godot's scene tree, all Godot nodes, and all Godot resources requires `NonSend`, so multithreading is only applicable to behavior and state stored entirely Rust-side.
