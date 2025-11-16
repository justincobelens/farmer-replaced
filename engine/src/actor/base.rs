use crate::{
	actor::{ActorId, Property},
	math::Transform,
};
use std::any::Any;

pub trait HasId {
	fn id_prop(&self) -> &Property<ActorId>;
}

pub trait HasTransform {
	fn transform(&self) -> &Property<Transform>;
}

pub trait ActorBase: Send + Sync + 'static {
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;

	fn set_id(&self, id: ActorId);
	fn get_id(&self) -> ActorId;
	fn get_transform(&self) -> Transform;
}

impl<T> ActorBase for T
where
	T: Any + Send + Sync + 'static + HasId + HasTransform,
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

	fn get_id(&self) -> ActorId {
		self.id_prop().get()
	}

	fn get_transform(&self) -> Transform {
		self.transform().get()
	}
}
