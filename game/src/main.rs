use engine::utility::functions::new_active_world;
use game::{Example2Actor, ExampleActor};

#[tokio::main]
async fn main() {
	let world = new_active_world();

	world.spawn_actor(ExampleActor::new()).await;
	println!("count: {}", world.get_actor_count().await);

	world.spawn_actor(Example2Actor::new()).await;
	println!("count: {}", world.get_actor_count().await);

	let actor = world.get_actor_of_class::<ExampleActor>().await.unwrap();
	let actor2 = world.get_actor_of_class::<Example2Actor>().await.unwrap();

	actor.example_method();
	actor2.another_example_method();

	actor.health.set(24);

	actor2.health.set(99);

	actor.health.update(|h| *h += 1);

	let id = actor.get_id();
	println!("actor id: {:?}", id);

	world.shutdown();
	let id = actor.get_id();
	println!("actor id: {:?}", id);
}
