# Bevy💋Godot
Use Bevy in Godot by forcing them to smooch a whole bunch. This is a personal project I'm creating from scratch so I understand the inner workings of Godot/Bevy interactions.

[📚 Read the book here! 📚](https://somerandev.github.io/Bevy-KISSING-Godot/book/)

&nbsp;

## Automatic Setup

If you don't have a `gdext` Rust project set up for your project already, this method is a good place to start!

### Screenshot

This uses the Bevy💋Godot Rust Project Generation Tool shown below:

<img width="601" height="316" alt="Bevy💋Godot Rust Project Generation Tool" src="https://github.com/user-attachments/assets/c1657736-d89c-4f68-a651-b2d42008de1c" />

### Instructions

1) Install the addon located in `addons/` to your project.
2) Use the `Project -> Tools -> Generate Bevy💋Godot Rust Project` tool to generate a Rust project.
3) Go to the location you set for the Rust project and run `cargo build` (*make sure it's the same directory as the `Cargo.toml` with `[workspace]`*).
> [!WARNING]
> If you enable `.gdextension` generation via `gdext-gen`, you may need to specify the platform even if it's for your desktop development platform:
> ```
> cargo build --target x86_64-pc-windows-msvc
> ```

&nbsp;

## Manual Setup

If you already have a `gdext` Rust project in use, you can easily add Bevy💋Godot.

### Instructions
1) Install the addon located in `addons/` to your project.
2) Add the `bevy` and `bevy_kissing_godot` Rust crates:
```
cargo add bevy
cargo add bevy_kissing_godot --git https://github.com/SomeRanDev/Bevy-KISSING-Godot
```
3) Create the Bevy app entry function.
```rust
use bevy::prelude::*;
use bevy_kissing_godot::prelude::*;

#[kiss_bevy(node_name = YourNameForBevyAutoloadNode)]
fn main(app: &mut App) {
    // do stuff with app
}
```
4) Add an autoload node that extends from your native Node (in this case `YourNameForBevyAutoloadNode`).
```gdscript
# Create a GDScript file with just this and add to `Project > Project Settings > Globals > Autoload`
extends YourNameForBevyAutoloadNode
```

&nbsp;

## Similar Projects!
 * [godot-bevy](https://github.com/bytemeadow/godot-bevy) - Bring the power of Bevy to your Godot projects
 * [bevy_godot4](https://github.com/jrockett6/bevy_godot4) - Bring Bevy's ECS to Godot4
