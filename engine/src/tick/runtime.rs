use std::sync::Arc;
use std::thread;
use tokio::time::{self, Duration, Instant, MissedTickBehavior};

use crate::utility::Toggle;

pub trait TickListener: Send + Sync + 'static {
	fn on_tick(&self, dt: f64);
}

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

	pub fn subscribe(&self, l: Arc<dyn TickListener>) {
		self.listeners.write().push(l);
	}

	pub fn broadcast(&self, dt: f64) {
		// no alloc, no clone, just walk the slice
		let guard = self.listeners.read();
		for l in guard.iter() {
			l.on_tick(dt);
		}
	}
}

pub struct TickRuntime {
	toggle: Toggle,
	bus: Arc<TickBus>,
	hz: f64,
	clamp_factor: f64,
}

impl TickRuntime {
	pub fn variable_only(hz: f64) -> Arc<Self> {
		Arc::new(Self {
			toggle: Toggle::init_off(),
			bus: Arc::new(TickBus::new()),
			hz,
			clamp_factor: 2.0,
		})
	}

	pub fn bus(&self) -> Arc<TickBus> {
		self.bus.clone()
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
		let max_dt = Duration::from_secs_f64(period.as_secs_f64() * self.clamp_factor);

		loop {
			interval.tick().await;

			if self.toggle.is_off() {
				// reset so we don't send huge dt after pause
				last = Instant::now();
				continue;
			}

			let now = Instant::now();
			let mut dt = now - last;
			last = now;

			if dt > max_dt {
				dt = period; // clamp
			}

			self.bus.broadcast(dt.as_secs_f64());
		}
	}
}
