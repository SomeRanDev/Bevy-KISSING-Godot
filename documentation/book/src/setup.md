# Setup

Bevy💋Godot is built on top of [gdext](https://github.com/godot-rust/gdext), so you should start by familiarizing yourself with that project!

## Creating a New Project

### 1. Create New Project
Create a new project as you would with [gdext](https://github.com/godot-rust/gdext). Here's a convenient link to its [book](https://godot-rust.github.io/book/)!

This includes creating a `.gdextension` file AND creating your own `ExtensionLibrary` struct.
```rust,noplayground
# use godot::prelude::*;
# 
struct PoopPeeExtension;

#[gdextension]
unsafe impl ExtensionLibrary for PoopPeeExtension {
	fn on_stage_init(level: InitStage) {}
	fn on_stage_deinit(level: InitStage) {}
}
```

### 2. Install Addon
Download and install the Bevy💋Godot addon from `addons/` in the main [respository](https://github.com/SomeRanDev/Bevy-KISSING-Godot/tree/main). (Just copy and paste the `addons/` folder to the top level of your Godot project; say YES to merge if prompted.)

### 3. Add Rust Crates
Add Bevy and Bevy💋Godot to your Rust project:

```bash
# Become a Bevy enjoyer.
cargo add bevy
```

```bash
# Don't lose sight of Godot.
cargo add bevy_kissing_godot --git https://github.com/SomeRanDev/Bevy-KISSING-Godot
```

### 4. Create Bevy App Function
Create a function that takes a mutable reference to Bevy's `App`. This will be the entry point for Bevy integration. Add the `bevy_kissing_godot::prelude::kiss_bevy` attribute with your desired name for your Godot autoload `Node`.

```rust,noplayground
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# 
#[kiss_bevy(PoopPeeKisser)]
fn main(app: &mut App) {
	// do stuff with app
}
```

### 5. Add Autoload Node
Once you successfully compile your Rust stuff, you need to add your autoload `Node` (in our case `PoopPeeKisser`) to the "Autoload" section of your Godot project settings.

First create a GDScript file that extends from your Rust node:
```gdscript
class_name PoopPeeKisserGDScript extends PoopPeeKisser
```

Then you'll need to:
 * Go to `Project > Project Settings > Globals > Autoload`
 * Set the GDScript file path to "Path"
 * Press `+ Add` button

### 6. You're Now Ready to Go!

Read the title of this step to know if you're ready to go.
