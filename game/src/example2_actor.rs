use engine::{Actor, actor};

actor! {
	Example2Actor {
		health: i32 = 100,
		label: String = "Example2Actor".to_string(),
	}
}

impl Actor for Example2Actor {
	fn on_begin_play(&self) {
		println!("begin_play id={:?} and name={:?}", self.get_id(), self.name());

		self.health.set(32);
	}

	fn on_tick(&self, _dt: f64) {
		if !self.check_health(30) {
			println!("health unhealthy, fixing it");
			self.health.set(100);
		}
		self.health.set(self.health.get() - 1);
	}
}

impl Example2Actor {
	fn check_health(&self, num: i32) -> bool {
		if self.health.get() < num {
			return false;
		}
		return true;
	}
}
