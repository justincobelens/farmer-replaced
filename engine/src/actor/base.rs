use crate::actor::{ActorId, Property};
use std::any::Any;

pub trait HasId {
	fn id_prop(&self) -> &Property<ActorId>;
}

pub trait ActorBase: Send + Sync + 'static {
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;

	fn set_id(&self, id: ActorId);
	fn get_id_internal(&self) -> ActorId;
}

impl<T> ActorBase for T
where
	T: Any + Send + Sync + 'static + HasId,
{
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}

	fn set_id(&self, id: ActorId) {
		self.id_prop().set(id);
	}

	fn get_id_internal(&self) -> ActorId {
		self.id_prop().get()
	}
}
