use std::sync::Arc;

use crate::{
	World,
	game_instance::{GameInstance, instance},
};

type SystemFn = Box<dyn FnMut()>;

type WorldFn = SystemFn;
type RendertFn = Box<dyn Fn(&World) + Send + Sync>;
type ExtractFn = Box<dyn Fn(&World) + Send + Sync>;

pub struct App {
	game_instance: Arc<GameInstance>,
	world: Option<WorldFn>,
	extract: Option<ExtractFn>,
	render: Option<RendertFn>,
}

impl App {
	pub fn new() -> App {
		let game_instance = instance();
		App {
			game_instance,
			world: None,
			extract: None,
			render: None,
		}
	}

	pub fn add_world<F>(&mut self, system: F) -> &mut Self
	where
		F: FnMut() + 'static,
	{
		let _ = self.world.insert(Box::new(system));
		self
	}

	pub fn set_extract<F>(&mut self, system: F) -> &mut Self
	where
		F: Fn(&World) + Send + Sync + 'static,
	{
		self.extract = Some(Box::new(system));
		self
	}

	pub fn set_render<F>(&mut self, system: F) -> &mut Self
	where
		F: Fn(&World) + Send + Sync + 'static,
	{
		self.render = Some(Box::new(system));
		self
	}

	pub fn run(&mut self) {
		self.world.as_mut().expect("No world set")(); // panic if no world present

		if let Some(extract_fn) = self.extract.take() {
			self.game_instance.set_extract(extract_fn);
		}

		if let Some(render_fn) = self.render.take() {
			self.game_instance.set_render(render_fn);
		}

		self.game_instance.start_tick();
		self.game_instance.run_forever();
	}
}
