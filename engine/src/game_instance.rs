use std::{
	collections::HashMap,
	sync::{
		Arc, OnceLock,
		atomic::{AtomicBool, AtomicU64, Ordering},
	},
	{thread, time::Duration},
};

use parking_lot::RwLock;

use crate::{World, actor::ActorId, app::ExtractFn, tick::runtime::TickRuntime};

static INSTANCE: OnceLock<Arc<GameInstance>> = OnceLock::new();

pub(crate) fn instance() -> Arc<GameInstance> {
	INSTANCE
		.get_or_init(|| Arc::new(GameInstance::new()))
		.clone()
}

pub type WorldId = u64;
type RendertFn = Box<dyn Fn(&World) + Send + Sync>;

pub struct GameInstance {
	next_world_id: AtomicU64,
	next_actor_id: AtomicU64,
	worlds: RwLock<HashMap<WorldId, Arc<World>>>,
	active: RwLock<Option<WorldId>>,
	tick: Arc<TickRuntime>,
	running: AtomicBool,
	extract: RwLock<Option<ExtractFn>>,
	render: RwLock<Option<RendertFn>>,
}

impl GameInstance {
	fn new() -> Self {
		let tick = TickRuntime::variable(10.0);
		tick.clone().start_thread();

		Self {
			next_world_id: AtomicU64::new(1),
			next_actor_id: AtomicU64::new(1),
			worlds: RwLock::new(HashMap::new()),
			active: RwLock::new(None),
			tick,
			running: AtomicBool::new(true),
			extract: RwLock::new(None),
			render: RwLock::new(None),
		}
	}

	pub fn next_actor_id(&self) -> ActorId {
		ActorId(self.next_actor_id.fetch_add(1, Ordering::Relaxed))
	}

	pub fn create_world(&self) -> WorldId {
		let id = self.next_world_id.fetch_add(1, Ordering::Relaxed);
		let world = World::new();

		self.tick.subscribe(world.clone());
		self.worlds.write().insert(id, world);
		// initialize active if not set
		let mut active = self.active.write();
		if active.is_none() {
			*active = Some(id);
		}
		id
	}

	pub fn set_active_world(&self, id: WorldId) -> bool {
		if !self.worlds.read().contains_key(&id) {
			return false;
		}
		*self.active.write() = Some(id);

		true
	}

	pub fn active_world(&self) -> Option<Arc<World>> {
		let id = *self.active.read();
		let id = id?;
		self.worlds.read().get(&id).cloned()
	}

	pub fn update(&self) {
		let main_world = self.active_world().unwrap();

		let mut render_world = main_world.as_ref().clone();

		if let Some(extract_fn) = &*self.extract.read() {
			extract_fn(main_world.as_ref(), &mut render_world);
		}

		if let Some(render) = &*self.render.read() {
			render(&render_world);
		}

		// todo ui commands should be here
	}

	pub(crate) fn start_tick(&self) {
		self.tick.resume();
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

	// Request the main loop to exit
	pub fn request_exit(&self) {
		self.running.store(false, Ordering::SeqCst);
	}

	/// Block the current thread until `request_exit` is called.
	pub fn run_forever(&self) {
		// Ensure start in a running state
		self.running.store(true, Ordering::SeqCst);

		loop {
			if !self.running.load(Ordering::SeqCst) {
				break;
			}
			thread::sleep(Duration::from_millis(16));
		}
	}

	pub fn set_extract(&self, system: ExtractFn) {
		*self.extract.write() = Some(system);
	}

	pub fn set_render(&self, system: RendertFn) {
		*self.render.write() = Some(system);
	}
}
