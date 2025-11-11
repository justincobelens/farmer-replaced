use engine::{App, World, math::Transform, utility::functions::new_active_world};
use game::{Example2Actor, ExampleActor};

fn world() {
	let world = new_active_world();

	world.spawn_actor(ExampleActor::new(Transform::default()));
	world.spawn_actor(Example2Actor::new(Transform::default()));
}

fn extract(world: &World) {
	if let Some(actor) = world.get_actor_of_class::<Example2Actor>() {
		let health = actor.health.get();
		println!("Actor health: {:?}", health);

		if health >= 99 {
			world.spawn_actor(Example2Actor::new(Transform::default()));
		}

		if health <= 95 {
			world.spawn_actor(ExampleActor::new(Transform::default()));
		}
	}
}

fn render(world: &World) {
	let count = world.get_actor_count();
	println!("called render with count: {:?}", count);
}

fn main() {
	App::new()
		.add_world(world)
		.set_extract(extract)
		.set_render(render)
		.run();
}
