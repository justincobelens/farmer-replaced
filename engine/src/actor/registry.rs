use std::{any::Any, sync::Arc};
use tokio::sync::RwLock;

use crate::actor::{Actor, ActorEvent};

pub struct ActorEntry {
	pub actor_arc: Arc<dyn Any + Send + Sync>,
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

	pub async fn add_actor_entry<A: Actor>(&self, actor_arc: Arc<A>) {
		let mut guard = self.actors.write().await;
		let erased: Arc<dyn Any + Send + Sync> = actor_arc;

		guard.push(ActorEntry { actor_arc: erased });
	}

	pub async fn len(&self) -> usize {
		self.actors.read().await.len()
	}

	/// Get the first actor of a class `A` as `Arc<A>`.
	pub async fn get_actor_of_class<A: Actor>(&self) -> Option<Arc<A>> {
		let guard = self.actors.read().await;
		for entry in guard.iter() {
			let any_arc = entry.actor_arc.clone();
			if let Ok(t_arc) = Arc::downcast::<A>(any_arc) {
				return Some(t_arc);
			}
		}
		None
	}

	/// Get all actors of a class.
	pub async fn get_all_of_class<A: Actor>(&self) -> Vec<Arc<A>> {
		let guard = self.actors.read().await;
		let mut out = Vec::new();
		for entry in guard.iter() {
			let any_arc = entry.actor_arc.clone();
			if let Ok(t_arc) = Arc::downcast::<A>(any_arc) {
				out.push(t_arc);
			}
		}
		out
	}

	pub fn broadcast(&self, event: ActorEvent) {
		println!("Received event: {:?}", event);
	}
}
