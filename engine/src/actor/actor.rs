use crate::actor::base::ActorBase;

pub trait Actor: ActorBase {
	fn on_begin_play(&self) {}
	fn on_tick(&self, _dt: f64) {}
	fn on_end_play(&self) {}
	fn name(&self) -> &'static str {
		std::any::type_name::<Self>()
	}
}
