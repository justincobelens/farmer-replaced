use engine::{instance, math::Transform, utility::functions::new_active_world};
use game::{Example2Actor, ExampleActor};

#[tokio::main]
async fn main() {
	let world = new_active_world();
	instance().resume_tick();

	world.spawn_actor(ExampleActor::new(Transform::default()));
	println!("count: {}", world.get_actor_count());

	world.spawn_actor(Example2Actor::new(Transform::default()));

	println!("count: {}", world.get_actor_count());

	let actor = world.get_actor_of_class::<ExampleActor>().unwrap();
	let actor2 = world.get_actor_of_class::<Example2Actor>().unwrap();

	actor.example_method();
	actor2.another_example_method();

	actor.health.set(24);

	actor2.health.set(99);

	let id = actor.get_id();
	println!("actor id: {:?}", id);

	world.shutdown();
	let id = actor.get_id();
	println!("actor id: {:?}", id);
}
