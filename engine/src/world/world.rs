use std::sync::Arc;

use crate::{
	actor::{Actor, ActorEvent, ActorRegistry},
	instance,
	tick::Tick,
};

pub struct World {
	registry: Arc<ActorRegistry>,
	tick: Arc<Tick>,
}

impl World {
	pub fn new() -> Arc<World> {
		let registry = Arc::new(ActorRegistry::new());
		let tick = Arc::new(Tick::new());

		let world = Arc::new(Self { registry, tick });

		world.tick.start(world.clone());
		world.resume();
		world
	}

	pub async fn spawn_actor<A: Actor>(&self, actor: A) -> Arc<A> {
		let actor = Arc::new(actor);

		actor.set_id(instance().alloc_actor_id());

		self.registry.add_actor_entry(actor.clone()).await;
		actor.on_begin_play();

		actor
	}

	/// Get len of total actors
	pub async fn get_actor_count(&self) -> usize {
		self.registry.len().await
	}

	/// Get the first actor of a class
	pub async fn get_actor_of_class<A: Actor>(&self) -> Option<Arc<A>> {
		self.registry.get_actor_of_class().await
	}

	/// Get all actors of a class
	pub async fn get_all_of_class<A: Actor>(&self) -> Vec<Arc<A>> {
		self.registry.get_all_of_class().await
	}

	/// World wide tick broadcast
	pub fn tick(&self, dt: f64) {
		self.registry.broadcast(ActorEvent::Tick(dt))
	}

	/// Graceful shutdown
	pub fn shutdown(&self) {
		self.registry.broadcast(ActorEvent::EndPlay)
	}
}

impl World {
	pub fn resume(&self) {
		self.tick.resume();
	}

	pub fn pause(&self) {
		self.tick.pause();
	}
}
