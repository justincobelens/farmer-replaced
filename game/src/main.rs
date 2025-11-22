use engine::{App, World, commands::Commands, math::Transform, resource::Resource};
use engine_macros::Resource;
use ui::render::render_pipeline;

use game::{Example2Actor, ExampleActor};

#[derive(Resource)]
pub struct GameState;

fn main() {
	App::new()
		.set_setup(setup)
		.set_extract(extract)
		.set_render(render)
		.run();
}

fn setup(commands: Commands) {
	let actor_1 = ExampleActor::new(Transform::default());
	let actor_2 = Example2Actor::new(Transform::default());

	commands.spawn(actor_1);
	commands.spawn(actor_2);
}

fn extract(_main_world: &World, _render_world: &mut World) {
	// this func should sync the main world with the render world
	// this is just filler code
}

fn render(world: &World) {
	render_pipeline(world);
}
