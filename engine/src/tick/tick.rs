use std::sync::Arc;

use crate::{World, tick::runner::spawn_once_tick_runner, utility::Toggle};

pub struct Tick {
	on_off_toggle: Toggle,
}

impl Tick {
	pub fn new() -> Tick {
		let on_off_toggle = Toggle::init_off();
		Tick { on_off_toggle }
	}

	pub fn start(&self, world: Arc<World>) {
		spawn_once_tick_runner(self.on_off_toggle.clone(), world);
	}

	pub fn resume(&self) {
		println!("Tick resumed");
		self.on_off_toggle.turn_on();
	}
	pub fn pause(&self) {
		println!("Tick paused");
		self.on_off_toggle.turn_off();
	}
}
