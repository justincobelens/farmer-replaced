use std::sync::Arc;
use std::thread;
use tokio::time::{self, Duration, Instant, MissedTickBehavior};

use crate::{
	tick::{bus::TickBus, listener::TickListener},
	utility::Toggle,
};

pub struct TickRuntime {
	toggle: Toggle,
	bus: Arc<TickBus>,
	hz: f64,
}

impl TickRuntime {
	pub fn variable(hz: f64) -> Arc<Self> {
		Arc::new(Self {
			toggle: Toggle::init_off(),
			bus: Arc::new(TickBus::new()),
			hz,
		})
	}

	pub fn subscribe(&self, object: Arc<dyn TickListener>) {
		self.bus.subscribe(object);
	}

	pub fn resume(&self) {
		self.toggle.turn_on();
	}

	pub fn pause(&self) {
		self.toggle.turn_off();
	}

	pub fn start_thread(self: Arc<Self>) {
		let _ = thread::Builder::new().name("tick-runtime".to_string()).spawn(move || {
			let rt = tokio::runtime::Runtime::new().expect("failed to create tick runtime");
			rt.block_on(self.run());
		});
	}

	async fn run(self: Arc<Self>) {
		let period = Duration::from_secs_f64(1.0 / self.hz);
		let start = Instant::now() + period;
		let mut interval = time::interval_at(start, period);
		interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

		let mut last = Instant::now();

		loop {
			interval.tick().await;

			if self.toggle.is_off() {
				// reset so we don't send huge dt after pause
				last = Instant::now();
				continue;
			}

			let now = Instant::now();
			let dt = now - last;
			last = now;

			self.bus.broadcast(dt.as_secs_f64());
		}
	}
}
