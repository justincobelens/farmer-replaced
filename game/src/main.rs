use engine::{instance, math::Transform, utility::functions::new_active_world};
use game::{Example2Actor, ExampleActor};

fn main() {
	instance().resume_tick();

	let world = new_active_world();

	world.spawn_actor(ExampleActor::new(Transform::default()));
	world.spawn_actor(Example2Actor::new(Transform::default()));

	instance().run_forever();

	world.shutdown();
}
