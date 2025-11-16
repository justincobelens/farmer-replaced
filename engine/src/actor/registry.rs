use std::{any::Any, collections::HashMap, sync::Arc};

use parking_lot::RwLock;

use crate::actor::{Actor, ActorId, entry::ActorEntry};

pub struct ActorRegistry {
	/// All actors stored as entries
	entries: RwLock<Vec<ActorEntry>>,

	/// ActorId, or index, in entries
	index_by_id: RwLock<HashMap<ActorId, usize>>,
}

impl ActorRegistry {
	pub fn new() -> Self {
		Self {
			entries: RwLock::new(Vec::new()),
			index_by_id: RwLock::new(HashMap::new()),
		}
	}

	/// Add an actor. The actor must already have a valid id.
	pub fn add_actor<A: Actor>(&self, actor: Arc<A>) {
		let id = actor.get_id();
		debug_assert!(id.is_valid(), "adding actor with an invalid id");

		let entry = ActorEntry {
			id,
			actor: actor.clone() as Arc<dyn Actor>,
			erased: actor as Arc<dyn Any + Send + Sync>,
		};

		let mut entries = self.entries.write();
		let index = entries.len();
		entries.push(entry);
		self.index_by_id.write().insert(id, index);
	}

	/// Remove an actor by id. Returns the actor if found.
	pub fn remove_actor(&self, id: ActorId) -> Option<Arc<dyn Actor>> {
		let mut entries = self.entries.write();
		let mut index_by_id = self.index_by_id.write();

		let index = index_by_id.remove(&id)?;
		let removed = entries.swap_remove(index);

		// If swapped into index, fix its mapping
		if let Some(swapped) = entries.get(index) {
			index_by_id.insert(swapped.id, index);
		}

		Some(removed.actor)
	}

	/// Total number of actors.
	pub fn len(&self) -> usize {
		self.entries.read().len()
	}

	/// Get the first actor of a class `A`.
	/// Linear scan over entries
	pub fn get_actor_of_class<A: Actor>(&self) -> Option<Arc<A>> {
		let entries = self.entries.read();
		for entry in entries.iter() {
			if let Some(a) = entry.downcast::<A>() {
				return Some(a);
			}
		}
		None
	}

	/// Get all actors of a class `A`.
	/// Linear scan over entries
	pub fn get_all_of_class<A: Actor>(&self) -> Vec<Arc<A>> {
		let entries = self.entries.read();
		let mut out = Vec::new();
		for entry in entries.iter() {
			if let Some(a) = entry.downcast::<A>() {
				out.push(a);
			}
		}
		out
	}

	/// Call `on_tick` on all actors
	pub fn broadcast_tick(&self, dt_seconds: f64) {
		let actors = self.get_actor_snapshot();

		for actor in actors {
			actor.on_tick(dt_seconds);
		}
	}

	/// Call `on_end_play` on all actors
	pub fn broadcast_end_play(&self) {
		let actors = self.get_actor_snapshot();

		for actor in actors {
			actor.on_end_play();
		}
	}

	pub fn get_actor_snapshot(&self) -> Vec<Arc<dyn Actor>> {
		let entries = self.entries.read();
		entries.iter().map(|e| e.actor.clone()).collect()
	}
}
