use engine::{
	Actor, actor,
	math::{Transform, Vector},
};

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

		if self.get_transform().location.x > 20.0 {
			let mut new_transform = Transform::default();

			let y = self.transform.get().location.y;
			new_transform.set_location(Vector::new(0., y, 0.));
			self.transform.set(new_transform);
			self.add_y(1.0);
		}

		if self.get_transform().location.y > 35.0 {
			let mut new_transform = Transform::default();

			let x = self.transform.get().location.x;
			new_transform.set_location(Vector::new(x, 0., 0.));
			self.transform.set(new_transform);
		}
	}
}

impl ExampleActor {
	pub fn add_x(&self, x: f32) {
		let mut transform = self.transform.get();
		let location = transform.location + Vector::new(x, 0.0, 0.0);
		transform.set_location(location);
		self.transform.set(transform);
	}

	pub fn add_y(&self, y: f32) {
		let mut transform = self.transform.get();
		let location = transform.location + Vector::new(0., y, 0.0);
		transform.set_location(location);
		self.transform.set(transform);
	}
}
