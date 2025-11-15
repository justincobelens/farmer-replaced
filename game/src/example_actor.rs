use engine::{Actor, actor};

actor! {
	ExampleActor {
		health: i32 = 100,
		label: String = "ExampleActor".to_string(),
	}
}

impl Actor for ExampleActor {
	fn on_begin_play(&self) {
		println!("begin_play id={:?} and name={:?}", self.get_id(), self.name());
		self.health.set(70);
	}

	fn on_tick(&self, _dt: f64) {
		self.add_x(1.0);
	}
}

impl ExampleActor {
	pub fn add_x(&self, _x: f32) {}
}
