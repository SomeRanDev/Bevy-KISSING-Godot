use godot::prelude::*;

pub struct GdHandle<T: GodotClass>(InstanceId, std::marker::PhantomData<T>);

impl<T: GodotClass> GdHandle<T> {
	pub fn from_variant(variant: &Variant) -> GdHandle<T> {
		let gd_object: Gd<T> = variant.to();
		GdHandle(gd_object.instance_id(), std::marker::PhantomData)
	}

	pub fn to_gd(&self, #[expect(unused)] unlocker: &GdHandleUnlocker) -> Gd<T> {
		Gd::from_instance_id(self.0)
	}
}

/// Contains PhantomData, so safe to send across threads.
unsafe impl<T: GodotClass> Send for GdHandle<T> {}

/// Contains PhantomData, so safe to send across threads.
unsafe impl<T: GodotClass> Sync for GdHandle<T> {}

// ---

/// We need to make sure [GdHandle] is only converted to [godot::prelude::Gd]
/// on the main thread. To do this, [GdHandleUnlocker] exists as a non-send
/// resource that MUST be passed to the [GdHandle::to_gd] function for it to
/// work.
///
/// [GdHandleUnlocker] is intentionally non-constructable so users can only
/// access it via `NonSend<GdHandleUnlocker>`.
pub struct GdHandleUnlocker(());

impl GdHandleUnlocker {
	pub(crate) fn new() -> Self {
		Self(())
	}
}
