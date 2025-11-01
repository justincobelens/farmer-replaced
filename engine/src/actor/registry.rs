use std::{
	any::Any,
	sync::{Arc, RwLock},
};

use crate::actor::Actor;

pub struct ActorEntry {
	pub erased: Arc<dyn Any + Send + Sync>,
	pub actor: Arc<dyn Actor>,
}

pub struct ActorRegistry {
	actors: RwLock<Vec<ActorEntry>>,
}

impl ActorRegistry {
	pub fn new() -> Self {
		Self {
			actors: RwLock::new(Vec::new()),
		}
	}

	pub fn add_actor<A: Actor>(&self, actor: Arc<A>) {
		let erased: Arc<dyn Any + Send + Sync> = actor.clone();
		let dyn_actor: Arc<dyn Actor> = actor;
		let mut guard = self.actors.write().unwrap();
		guard.push(ActorEntry {
			erased,
			actor: dyn_actor,
		});
	}

	pub fn len(&self) -> usize {
		self.actors.read().unwrap().len()
	}

	/// Get the first actor of a class `A` as `Arc<A>`.
	pub fn get_actor_of_class<A: Actor>(&self) -> Option<Arc<A>> {
		let guard = self.actors.read().unwrap();
		for entry in guard.iter() {
			if let Ok(a) = Arc::downcast::<A>(entry.erased.clone()) {
				return Some(a);
			}
		}
		None
	}

	/// Get all actors of a class.
	pub fn get_all_of_class<A: Actor>(&self) -> Vec<Arc<A>> {
		let guard = self.actors.read().unwrap();
		let mut out = Vec::new();
		for entry in guard.iter() {
			if let Ok(a) = Arc::downcast::<A>(entry.erased.clone()) {
				out.push(a);
			}
		}
		out
	}

	pub fn broadcast_tick(&self, dt: f64) {
		let guard = self.actors.read().unwrap();
		for entry in guard.iter() {
			entry.actor.on_tick(dt as f32);
		}
	}

	pub fn broadcast_end_play(&self) {
		let guard = self.actors.read().unwrap();
		for entry in guard.iter() {
			entry.actor.on_end_play();
		}
	}
}
