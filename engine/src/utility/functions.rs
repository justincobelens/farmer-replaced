use std::sync::Arc;

use crate::game_instance::{WorldId, instance};
use crate::world::World;

/// Create a new world inside the single GameInstance. Returns its id.
pub fn new_world() -> WorldId {
	instance().create_world()
}

/// Make a specific world the active one. Returns false if id doesn't exist.
pub fn set_active_world(id: WorldId) -> bool {
	instance().set_active_world(id)
}

/// Get the currently active World (panics if none exists).
pub fn get_world() -> Arc<World> {
	try_get_world().expect("No active world. Call new_world() and set_active_world(...) first.")
}

/// Get the currently active World (Option form).
pub fn try_get_world() -> Option<Arc<World>> {
	instance().active_world()
}

/// Create a new world inside the single GameInstance. Activates it, and returns it
pub fn new_active_world() -> Arc<World> {
	set_active_world(new_world());
	get_world()
}
