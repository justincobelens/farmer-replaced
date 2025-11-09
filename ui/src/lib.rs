pub mod ui {
	use std::sync::Arc;

	pub struct UiObject {}

	impl UiObject {
		pub fn new() -> UiObject {
			UiObject {}
		}
	}
}

pub use ui::UiObject;
