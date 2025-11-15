use std::sync::Arc;

use crate::{
	World,
	commands::Commands,
	game_instance::{GameInstance, instance},
	utility::functions::new_active_world,
};

pub type RendertFn = Box<dyn Fn(&World) + Send + Sync>;
pub type ExtractFn = Box<dyn Fn(&World, &mut World) + Send + Sync>;
pub type CommandsFn = Box<dyn Fn(Commands) + Send + Sync>;

pub struct App {
	game_instance: Arc<GameInstance>,
	setup: Option<CommandsFn>,
	extract: Option<ExtractFn>,
	render: Option<RendertFn>,
}

impl App {
	pub fn new() -> App {
		let game_instance = instance();
		App {
			game_instance,
			setup: None,
			extract: None,

			render: None,
		}
	}

	pub fn set_setup<F>(&mut self, system: F) -> &mut Self
	where
		F: Fn(Commands) + Send + Sync + 'static,
	{
		let _ = self.setup.insert(Box::new(system));
		self
	}

	pub fn set_extract<F>(&mut self, system: F) -> &mut Self
	where
		F: Fn(&World, &mut World) + Send + Sync + 'static,
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
		if let Some(setup) = &self.setup {
			let world = new_active_world();
			let commands = Commands::new(world.clone());
			setup(commands);
		}

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
