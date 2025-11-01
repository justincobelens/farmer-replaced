use std::sync::Arc;

use crate::tick::listener::TickListener;

#[derive(Default)]
pub struct TickBus {
	listeners: parking_lot::RwLock<Vec<Arc<dyn TickListener>>>,
}

impl TickBus {
	pub fn new() -> Self {
		Self {
			listeners: parking_lot::RwLock::new(Vec::new()),
		}
	}

	pub fn subscribe(&self, object: Arc<dyn TickListener>) {
		self.listeners.write().push(object);
	}

	pub fn broadcast(&self, dt: f64) {
		let guard = self.listeners.read();
		for listener in guard.iter() {
			listener.on_tick(dt);
		}
	}
}
