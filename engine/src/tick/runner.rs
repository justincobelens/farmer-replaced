use super::error::Result;
use std::sync::Arc;
use tokio::time::{Duration, Instant, sleep};

use crate::{World, utility::Toggle};

const TICK_RATE: f64 = 1.0; // [hz]
pub fn spawn_once_tick_runner(toggle: Toggle, world: Arc<World>) {
	TickRunner::spawn_once(toggle, world);
}

struct TickRunner {
	on_off_toggle: Toggle,
	last_tick: Option<Instant>,
	last_tick_dt: Option<Duration>,
	world: Arc<World>,
}

impl TickRunner {
	fn new(toggle: Toggle, world: Arc<World>) -> TickRunner {
		TickRunner {
			on_off_toggle: toggle,
			last_tick: None,
			last_tick_dt: None,
			world,
		}
	}

	fn spawn_once(toggle: Toggle, world: Arc<World>) {
		let builder = std::thread::Builder::new().name("tick-runner".to_string());
		let world_clone = world.clone();

		let _ = builder.spawn(move || {
			tokio::runtime::Runtime::new()
				.expect("Failed to create tick tokio runtime")
				.block_on(async { TickRunner::new(toggle, world_clone).run().await })
		});
	}

	pub async fn run(&mut self) {
		let period = Duration::from_secs_f64(1. / TICK_RATE);
		println!("Period: {:?}", period);
		let mut interval = spin_sleep_util::interval(period);

		loop {
			let async_pre_sleep_ms: f32 = TICK_RATE as f32 / 4.;
			sleep(Duration::from_millis(async_pre_sleep_ms as u64)).await;

			interval.tick();

			if self.on_off_toggle.is_off() {
				continue;
			}

			match self.execute_tick().await {
				Ok(_) => {}
				Err(_) => {}
			}
		}
	}

	pub async fn execute_tick(&mut self) -> Result<()> {
		let now = Instant::now();
		if self.last_tick.is_none() {
			self.last_tick = Some(now);
			return Ok(());
		}

		// unwrap ok due to check above
		let dt = now.duration_since(self.last_tick.unwrap());

		self.last_tick = Some(now);
		self.last_tick_dt = Some(dt);

		self.world.tick(dt.as_secs_f64());
		Ok(())
	}
}
