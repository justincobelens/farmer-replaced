use std::sync::Arc;

use crate::{Actor, World};

pub struct Commands {
	world: Arc<World>,
}

impl Commands {
	pub fn new(world: Arc<World>) -> Self {
		Commands { world }
	}

	pub fn spawn<A: Actor>(&self, actor: A) -> Arc<A> {
		self.world.spawn_actor(actor)
	}

	pub fn get_actor_of_class<A: Actor>(&self) -> Option<Arc<A>> {
		self.world.get_actor_of_class()
	}
}
