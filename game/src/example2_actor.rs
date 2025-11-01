use engine::{Actor, actor};

actor! {
	Example2Actor {
		health: i32 = 100,
		label: String = "Example2Actor".to_string(),
	}
}

impl Example2Actor {
	pub fn another_example_method(&self) {
		println!("This is an example method, called by: {}", self.label.get());
	}
}

impl Actor for Example2Actor {
	fn on_begin_play(&self) {
		println!("begin_play with 10 sec sleep id={:?}", self.get_id());
		std::thread::sleep(std::time::Duration::from_secs(10));
		println!("finished sleep, this shouldn't have blocked tick")
	}

	fn on_tick(&self, dt: f32) {
		println!("tick {dt} id={:?} and blocking with 2500", self.get_id());
		std::thread::sleep(std::time::Duration::from_millis(2500));
		println!("tick {dt} id={:?} and done blocking", self.get_id());
	}

	fn name(&self) -> &'static str {
		std::any::type_name::<Self>()
	}

	fn on_end_play(&self) {
		println!("Default end_play()");
	}
}
