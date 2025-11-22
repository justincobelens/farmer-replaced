use engine::{App, World, commands::Commands, math::Transform};
use game::{Example2Actor, ExampleActor};
use ui::render::render_pipeline;

fn setup(commands: Commands) {
	commands.spawn(ExampleActor::new(Transform::default()));
	commands.spawn(Example2Actor::new(Transform::default()));
}

fn extract(main_world: &World, _render_world: &mut World) {
	// this func should sync the main world with the render world
	// this is just filler code
	if let Some(actor) = main_world.get_actor_of_class::<Example2Actor>() {
		let health = actor.health.get();
		println!("Actor health: {:?}", health);
	}
}

fn render(world: &World) {
	render_pipeline(world);
}

fn main() {
	App::new()
		.set_setup(setup)
		.set_extract(extract)
		.set_render(render)
		.run();
}
