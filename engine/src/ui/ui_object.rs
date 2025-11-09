use parking_lot::RwLock;
use std::sync::Arc;

use crate::World;
use crate::tick::listener::TickListener;

pub struct UiObject {
	world: RwLock<Option<Arc<World>>>,
}

impl UiObject {
	pub fn new() -> UiObject {
		UiObject {
			world: RwLock::new(None),
		}
	}

	pub fn with_world(&self, world: Option<Arc<World>>) {
		*self.world.write() = world;
	}

	pub fn render(&self) {
		if let Some(world) = &*self.world.read() {
			let actors = world.get_actor_count();
			println!("tick render with world with {:?} actors", actors);
		} else {
			println!("tick without world");
		}
	}
}

impl TickListener for UiObject {
	fn on_tick(&self, _dt: f64) {
		self.render()
	}
}
