use engine::{Actor, actor};

actor! {
	Example2Actor {
		health: i32 = 100,
		label: String = "Example2Actor".to_string(),
	}
}

impl Example2Actor {
	pub fn another_example_method(&self) {
		println!("from another exmple method health: {:?}", self.health.get());

		println!("This is an example method, called by: {}", self.label.get());
	}
}

impl Actor for Example2Actor {
	fn on_begin_play(&self) {
		self.health.set(32);
	}

	fn on_end_play(&self) {
		println!("Default end_play()");
	}

	fn on_tick(&self, _dt: f64) {
		if !self.check_health(30) {
			println!("health unhealthy, fixing it");
			self.health.set(100);
		}
		self.health.set(self.health.get() - 1);
	}

	fn name(&self) -> &'static str {
		std::any::type_name::<Self>()
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
