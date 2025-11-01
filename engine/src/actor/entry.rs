use std::{any::Any, sync::Arc};

use crate::actor::{Actor, ActorId};

pub struct ActorEntry {
	pub id: ActorId,
	pub actor: Arc<dyn Actor>,
	pub erased: Arc<dyn Any + Send + Sync>,
}

impl ActorEntry {
	pub fn downcast<A: Actor>(&self) -> Option<Arc<A>> {
		Arc::downcast::<A>(self.erased.clone()).ok()
	}
}
