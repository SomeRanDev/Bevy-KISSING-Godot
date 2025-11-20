use std::marker::PhantomData;

use bevy::prelude::*;
use godot::obj::{GodotClass, NoBase};

/// Used to mark an entity as having a `GodotNodeId` of type or descendant of `T`
#[derive(Component)]
pub struct GodotNode<T>(PhantomData<T>);

/// Constructs a `GodotNode`.
impl<T> Default for GodotNode<T> {
	fn default() -> Self {
		Self(PhantomData)
	}
}

/// Contains PhantomData, so safe to send across threads.
unsafe impl<T> Send for GodotNode<T> {}

/// Contains PhantomData, so safe to send across threads.
unsafe impl<T> Sync for GodotNode<T> {}

impl<T: GodotClass> GodotNode<T> {
	/// Given the type of a Godot class, inserts `GodotNode`s for its entire hierarchy.
	pub fn add_components_from_type<'a>(c: &mut EntityWorldMut<'a>) -> () {
		if T::class_id() == NoBase::class_id() {
			return;
		}
		c.insert(Self::default());
		GodotNode::<T::Base>::add_components_from_type(c);
	}
}
