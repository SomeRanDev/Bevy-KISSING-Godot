use crate::prelude::GodotThreadEnsurer;

use godot::prelude::*;

pub struct GdHandle<T: GodotClass>(InstanceId, std::marker::PhantomData<T>);

impl<T: GodotClass> GdHandle<T> {
	pub fn from_variant(variant: &Variant) -> GdHandle<T> {
		let gd_object: Gd<T> = variant.to();
		GdHandle(gd_object.instance_id(), std::marker::PhantomData)
	}

	pub fn to_gd(&self, _: &GodotThreadEnsurer) -> Gd<T> {
		Gd::from_instance_id(self.0)
	}
}

/// Contains PhantomData, so safe to send across threads.
unsafe impl<T: GodotClass> Send for GdHandle<T> {}

/// Contains PhantomData, so safe to send across threads.
unsafe impl<T: GodotClass> Sync for GdHandle<T> {}
