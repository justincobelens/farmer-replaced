use engine::World;

pub struct UiObject {}

impl UiObject {
	pub fn render(world: &World) {
		let actors = world.get_all_actors();

		let position_1 = actors.first().unwrap().get_transform().location;

		println!("Actor pos: {:?}", position_1);
	}
}
