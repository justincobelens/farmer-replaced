use std::{
	collections::HashMap,
	sync::atomic::{AtomicU64, Ordering},
	sync::{Arc, OnceLock, RwLock},
};

use crate::{World, actor::ActorId, tick::runtime::TickRuntime};

static INSTANCE: OnceLock<Arc<GameInstance>> = OnceLock::new();

pub fn instance() -> Arc<GameInstance> {
	INSTANCE.get_or_init(|| Arc::new(GameInstance::new())).clone()
}

pub type WorldId = u64;

pub struct GameInstance {
	next_world_id: AtomicU64,
	next_actor_id: AtomicU64,
	worlds: RwLock<HashMap<WorldId, Arc<World>>>,
	active: RwLock<Option<WorldId>>,
	tick: Arc<TickRuntime>,
}

impl GameInstance {
	fn new() -> Self {
		let tick = TickRuntime::variable(1.0);
		tick.clone().start_thread();

		Self {
			next_world_id: AtomicU64::new(1),
			next_actor_id: AtomicU64::new(1),
			worlds: RwLock::new(HashMap::new()),
			active: RwLock::new(None),
			tick,
		}
	}

	pub fn next_actor_id(&self) -> ActorId {
		ActorId(self.next_actor_id.fetch_add(1, Ordering::Relaxed))
	}

	pub fn create_world(&self) -> WorldId {
		let id = self.next_world_id.fetch_add(1, Ordering::Relaxed);
		let world = World::new();

		self.tick.subscribe(world.clone());
		self.worlds.write().expect("worlds poisoned").insert(id, world);
		// initialize active if not set
		let mut active = self.active.write().expect("active poisoned");
		if active.is_none() {
			*active = Some(id);
		}
		id
	}

	pub fn set_active_world(&self, id: WorldId) -> bool {
		if !self.worlds.read().expect("worlds poisoned").contains_key(&id) {
			return false;
		}
		*self.active.write().expect("active poisoned") = Some(id);
		true
	}

	pub fn active_world(&self) -> Option<Arc<World>> {
		let id = *self.active.read().expect("active poisoned");
		let id = id?;
		self.worlds.read().expect("worlds poisoned").get(&id).cloned()
	}

	pub fn get_world(&self, id: WorldId) -> Option<Arc<World>> {
		self.worlds.read().expect("worlds poisoned").get(&id).cloned()
	}

	pub fn resume_tick(&self) {
		self.tick.resume();
	}

	pub fn pause_tick(&self) {
		self.tick.pause();
	}

	/// Engine wide unique actor IDs
	pub fn alloc_actor_id(&self) -> ActorId {
		let n = self.next_actor_id.fetch_add(1, Ordering::Relaxed);
		ActorId(n)
	}
}
