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

Of course, Godot signals also pass data! We can selectively receive data by adding fields to our Kissing event with the `#[godot_signal_arg(INDEX)]` attribute. `INDEX` is the index of the argument we want the field to receive the data from.

For example, the `OptionButton.item_selected` signal provides a single argument `index: int`. So we need to add an `int`-compatible type as a field to our event and annotate it  with `#[godot_signal_arg(0)]`.
```rust,noplayground
# use bevy::prelude::*;
# use bevy_kissing_godot::prelude::*;
# use godot::classes::Control;
# 
#[derive(EntityEvent, KissingEvent)]
struct ItemSelected {
	#[event_target]
	entity: Entity,

	#[godot_signal_arg(0)]
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
