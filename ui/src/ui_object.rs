use engine::World;

pub struct UiObject {}

impl UiObject {
	pub fn render(world: &World) {
		let count = world.get_actor_count();
		println!("UI OBJECT called render with count: {:?}", count);
	}
}
