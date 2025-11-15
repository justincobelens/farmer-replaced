use std::sync::Arc;

use crate::{
	actor::{Actor, ActorId, ActorRegistry},
	game_instance::instance,
	tick::listener::TickListener,
};

#[derive(Clone)]
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

	/// Despawn by id. Calls `on_end_play` and removes the actor from the registry.
	/// Returns true if an actor was removed.
	pub fn despawn_actor(&self, id: ActorId) -> bool {
		if let Some(actor) = self.registry.remove_actor(id) {
			actor.on_end_play();
			true
		} else {
			false
		}
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

	pub fn has_changes(&self) -> bool {
		todo!("Not implemented yet")
	}
}

impl TickListener for World {
	fn on_tick(&self, dt: f64) {
		self.registry.broadcast_tick(dt);
		println!("Tick dt={:?}", dt);

		instance().update();
	}
}
