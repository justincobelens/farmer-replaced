pub struct Window {
	pub width: f32,
	pub height: f32,
}

impl Window {
	pub fn new(width: f32, height: f32) -> Self {
		Self { width, height }
	}
}
