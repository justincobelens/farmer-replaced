use crate::Actor;

#[derive(Debug, Clone)]
pub struct Tile {
	pub slot: Option<Box<dyn Actor>>,
}

impl Tile {
	pub fn is_empty(&self) -> bool {
		self.slot.is_some()
	}
}

impl Default for Tile {
	fn default() -> Self {
		Self {
			slot: Default::default(),
		}
	}
}
