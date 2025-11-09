pub mod ui {

	pub struct UiObject {}

	impl UiObject {
		pub fn new() -> UiObject {
			UiObject {}
		}
	}
}

pub use ui::UiObject;
