use crate::actor::base::ActorBase;

pub trait Actor: ActorBase {
	fn on_begin_play(&self) {
		println!("Default begin_play()");
	}
	fn on_tick(&self, _dt: f32) {
		println!("Default on_tick()");
	}
	fn on_end_play(&self) {
		println!("Default end_play()");
	}
	fn name(&self) -> &'static str {
		std::any::type_name::<Self>()
	}
}
