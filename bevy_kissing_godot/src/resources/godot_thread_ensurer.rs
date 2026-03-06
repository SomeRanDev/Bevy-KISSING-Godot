/// We need to make sure [`GdHandle`] is only converted to [`godot::prelude::Gd`]
/// on the main thread. To do this, [`GodotThreadEnsurer`] exists as a non-send
/// resource that MUST be passed to the [GdHandle::to_gd] function for it to
/// work.
///
/// [`GodotThreadEnsurer`] is intentionally non-constructable so users can only
/// access it via `NonSend<GodotThreadEnsurer>` or `world.get_non_send_resource::<GodotThreadEnsurer>()`.
///
/// Users can also create functions that take a `&GodotThreadEnsurer` to ensure
/// they only run on a Godot-API compatible thread.
pub struct GodotThreadEnsurer(());

impl GodotThreadEnsurer {
	pub(crate) fn new() -> Self {
		Self(())
	}
}
