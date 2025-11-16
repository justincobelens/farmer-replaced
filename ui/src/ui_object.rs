use engine::World;

pub struct UiObject {}

impl UiObject {
	pub fn render(world: &World) {
		let actors = world.get_all_actors();

		for actor in actors {
			let transform = actor.get_transform();
			let loc = transform.location;

			let x = loc.x;
			let y = loc.y;

			println!("Actor pos: {:?}:{:?}", x, y);
		}
	}
}
