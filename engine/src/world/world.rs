use std::sync::Arc;

use crate::{
	actor::{Actor, ActorRegistry},
	instance,
	tick::listener::TickListener,
};

pub struct World {
	registry: Arc<ActorRegistry>,
}

impl World {
	pub fn new() -> Arc<World> {
		Arc::new(Self {
			registry: Arc::new(ActorRegistry::new()),
		})
	}

	pub fn spawn_actor<A: Actor>(&self, actor: A) -> Arc<A> {
		let actor = Arc::new(actor);
		actor.set_id(instance().alloc_actor_id());
		self.registry.add_actor(actor.clone());
		actor.on_begin_play();
		actor
	}

	/// Get len of total actors
	pub fn get_actor_count(&self) -> usize {
		self.registry.len()
	}

	/// Get the first actor of a class
	pub fn get_actor_of_class<A: Actor>(&self) -> Option<Arc<A>> {
		self.registry.get_actor_of_class()
	}

	/// Get all actors of a class
	pub fn get_all_of_class<A: Actor>(&self) -> Vec<Arc<A>> {
		self.registry.get_all_of_class()
	}

	/// Graceful shutdown
	pub fn shutdown(&self) {
		self.registry.broadcast_end_play();
	}
}

impl TickListener for World {
	fn on_tick(&self, dt: f64) {
		self.registry.broadcast_tick(dt);
	}
}
