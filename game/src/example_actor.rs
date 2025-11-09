use engine::{Actor, actor};

actor! {
	ExampleActor {
		health: i32 = 100,
		label: String = "ExampleActor".to_string(),
	}
}

impl ExampleActor {
	pub fn example_method(&self) {
		println!("This is an example method called by: {}", self.label.get());
	}
}

impl Actor for ExampleActor {
	fn on_begin_play(&self) {
		println!("begin_play id={:?}", self.get_id());
		self.health.set(70);
	}

	fn on_tick(&self, _dt: f64) {
		// println!("tick {dt} id={:?}", self.get_id());
	}

	fn name(&self) -> &'static str {
		std::any::type_name::<Self>()
	}

	fn on_end_play(&self) {
		println!("Default end_play()");
	}
}
