use engine::{App, World, commands::Commands, math::Transform};
use game::{Example2Actor, ExampleActor};
use ui::ui_object::UiObject;

fn setup(commands: Commands) {
	commands.spawn(ExampleActor::new(Transform::default()));
	commands.spawn(Example2Actor::new(Transform::default()));
}

fn extract(main_world: &World, render_world: &mut Arc<World>) {
	// this func should sync the main world with the render world
	// this is just filler code
	if let Some(actor) = main_world.get_actor_of_class::<Example2Actor>() {
		let health = actor.health.get();
		println!("Actor health: {:?}", health);
	}
}

fn render(world: &World) {
	UiObject::render(world);
}

fn main() {
	App::new()
		.set_setup(setup)
		.set_extract(extract)
		.set_render(render)
		.run();
}
