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
		println!("begin_play with 10 sec sleep id={:?}", self.get_id());
		std::thread::sleep(std::time::Duration::from_secs(10));
		println!("finished sleep, this shouldn't have blocked tick")
	}

	fn on_tick(&self, _dt: f64) {
		if self.health.get() < 100 {
			println!("health below 100 from tick")
		}
	}

	fn name(&self) -> &'static str {
		std::any::type_name::<Self>()
	}

	fn on_end_play(&self) {
		println!("Default end_play()");
	}
}
