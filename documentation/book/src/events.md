# Events

## Creating a Kissing event

In Bevy💋Godot, a "kissing" event is an event that is exposed and visible in the Godot editor. Nodes can connect their signals to these events.

To make a "kissing" event, just derive from `KissingEvent`. This works with both `Event` and `EntityEvent`. 
```rust,noplayground
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# 
/// An event you'd connect to Control.focus_entered or something...
#[derive(Event, KissingEvent)]
struct FocusEntered;

// ---

#[kiss_bevy]
fn setup(app: &mut App) {
	app.add_observer(on_focus);
}

fn on_focus(event: On<FocusEntered>) {
	// do something...
}
```

## Creating a Kissing entity event

If you want to track which entity/node triggered the signal, you probably want to use an `EntityEvent`. Note that you MUST use the `#[event_target]` attribute to mark the entity field:
```rust,noplayground
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# use godot::classes::Control;
# 
#[derive(EntityEvent, KissingEvent)]
struct MinimumSizeChanged(#[event_target] Entity);

// or

#[derive(EntityEvent, KissingEvent)]
struct MinimumSizeChanged {
	#[event_target]
	entity: Entity,
}
```

You can get the node from the `entity` using a query. Visit [Queries](./queries.html) for more details.

```rust,noplayground
#[kiss_bevy]
fn setup(app: &mut App) {
	app.add_observer(on_minimum_size_changed);
}

fn on_minimum_size_changed(
	event: On<MinimumSizeChanged>,
	query: Query<&GodotNodeId, With<GodotNode<Control>>,
	all_nodes: NonSend<AllNodes>,
) {
	let control: &GodotNodeId = query.get(event.entity).unwrap();
	let control: Gd<Control> = control.get_as::<Control>(&all_nodes);
	// do something with control
}
```

## Receiving arguments from signals

Of course, Godot signals also pass data! We can selectively receive data by adding fields to our Kissing event with the `#[godot_signal_arg(index = <INDEX>)]` attribute. `INDEX` is the index of the argument we want the field to receive the data from.

For example, the `OptionButton.item_selected` signal provides a single argument `index: int`. So we need to add an `int`-compatible type as a field to our event and annotate it  with `#[godot_signal_arg(index = 0)]`.
```rust,noplayground
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# use godot::classes::OptionButton;
# 
#[derive(EntityEvent, KissingEvent)]
struct ItemSelected {
	#[event_target]
	entity: Entity,

	#[godot_signal_arg(index = 0)]
	index: i32,
}

// ---

#[kiss_bevy]
fn setup(app: &mut App) {
	app.add_observer(on_item_selected);
}

fn on_item_selected(
	event: On<ItemSelected>,
	query: Query<&GodotNodeId, With<GodotNode<OptionButton>>,
	all_nodes: NonSend<AllNodes>,
) {
	let option_button_id = query.get(event.entity).unwrap();
	let option_button = option_button_id.get_as::<OptionButton>(&all_nodes);
	godot_print!("Selected option {}", option_button.get_selected());
}
```

## Receiving `Gd<T>` objects from signals

In some situations, you'll need to receive a `Gd<T>` object from a Godot signal. For example, `Tree.button_clicked` passes a `Gd<TreeItem>` for the first argument. For this, the `gd_handle` argument can be added to `#[godot_signal_arg]` to signify the object needs to be stored as a `GdHandle`.

```rust,noplayground
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# use godot::classes::TreeItem;
# 
#[derive(Event, KissingEvent)]
struct TreeItemButtonClicked {
	#[godot_signal_arg(index = 0, gd_handle)]
	tree_item: GdHandle<TreeItem>,
}
```

A `GdHandle` can be converted to a `Gd<T>` using a `NonSend<GdHandleUnlocker>`.

```rust
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# use godot::classes::TreeItem;
# 
fn on_tree_item_button_clicked(
	event: On<TreeItemButtonClicked>,
	unlocker: NonSend<GdHandleUnlocker>
) {
	let item: Gd<TreeItem> = event.tree_item.to_gd(&unlocker);
	// do something with `item`...
}
```

## Custom conversion from signal Variant argument

Godot provides the signal's arguments as `Variant`s when the signal is triggered. Bevy💋Godot then uses the `Variant`'s `to` function to convert them to their argument's type. However, if you'd like to directly control how a `Variant` argument is converted into the `Event` field, the `from_variant` argument can be used.

```rust,noplayground
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# use godot::classes::TreeItem;
# 
#[derive(Event, KissingEvent)]
struct TreeItemButtonClicked {
	// Instead of storing the `TreeItem`, store its index
	#[godot_signal_arg(index = 0, from_variant = get_tree_item_index)]
	tree_item_index: i32,
}

// Given the `Variant` representation of a `TreeItem`, get its index
fn get_tree_item_index(v: &Variant) -> i32 {
	let tree_item: Gd<TreeItem> = v.to();
	tree_item.get_index()
}
```

## Extra fields

All fields on the `KissingEvent` struct must be annotated with `#[event_target]`, `#[godot_signal_arg]`, OR `#[godot_signal_value]`. `#[godot_signal_value]` is helpful if you want to have a constant value to fill a field when the event is triggered from the Godot editor.

```rust
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# use godot::classes::Control;
# 
#[derive(Event, KissingEvent)]
struct UpdateText {
	// If called from Godot, this value will be an empty String
	#[godot_signal_value("".to_string())]
	text: String,
}

fn my_system(mut commands: Commands) {
	// But in your Rust code elsewhere, you can provide a custom value
	commands.trigger(UpdateText {
		text: "my custom text"
	});
}
```

## Manually connecting signals

All `KissingEvent`-derived structs generate a `typed_slot` function you can use to connect to Godot signals manually. Its arguments will be the fields of the struct (plus entity if an `EntityEvent`). However, please note you DO need to have access to the node's Bevy entity upon connecting.

So if you had a `Gd<OptionButton>` (and its `Entity`), you could manually connect it to the `ItemSelected` event above by doing:
```rust
let option_button: Gd<OptionButton> = /* ... */;
let option_button_entity: Entity = /* ... */;

option_button.signals().item_selected.connect(move |index: i32| {
	ItemSelected::typed_slot(option_button_entity, index);
});
```
