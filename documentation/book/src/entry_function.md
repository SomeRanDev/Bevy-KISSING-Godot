# Entry Function (`kiss_bevy` attribute)

`kiss_bevy` is the macro attribute used to dictate the entry function.

```rust,noplayground
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# 
#[kiss_bevy(node_name = PoopPeeKisser)]
fn main(app: &mut App) {
	// do stuff with app
}
```

## Declaration and Arguments

In addition to the autoload `Node` name argument, it has two more optional arguments. The declaration could be seen as something like this (assume `Option` arguments are optional):
```rust,noplayground
kiss_bevy(
	node_name = Ident,
	process_wrapper = Option<Path>,
	physics_process_wrapper = Option<Path>,
)
```

`process_wrapper` and `physics_process_wrapper` can be assigned paths to macros that take two expression arguments:
 * The first is the original expression that would be generated if the macro wasn't passed.
 * The second is the `self` expression for the autoload `Node`.

`process_wrapper` wraps the `process` expression of the generated Bevy app node. `physics_process_wrapper` does the same, but for the `physics_process`.

## Example

For example, you can configure panic capturing like this:
```rust,noplayground
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# 
/// Wraps the process call with a panic catcher.
macro_rules panic_catcher {
	($original_expression: expr, $self: expr) => {
		let result = std::panic::catch_unwind(|| {
			$original_expression
		});
		if result.is_err() {
			println!("Panic happened!");

			// Check bevy_kissing_godot::kissing_app for all `self.app` functions.
			$self.app.clear_app();
		}
	}
}

#[kiss_bevy(
	node_name = MyAppNodeName,
	process_wrapper = panic_catcher,
	physics_process_wrapper = panic_catcher
)]
fn main(app: &mut App) {
	// Do stuff with `app`...
}
```