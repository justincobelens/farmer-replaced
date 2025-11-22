use crate::resource::entry::ResourceEntry;
use parking_lot::RwLock;

pub struct ResourceRegistry {
	/// All resources stored as entries
	_entries: RwLock<Vec<ResourceEntry>>,
	// /// ResourceId, or index, in entries
	// index_by_id: RwLock<HashMap<ResourceId, usize>>,
}

impl ResourceRegistry {
	pub fn new() -> Self {
		Self {
			_entries: RwLock::new(Vec::new()),
			// index_by_id: RwLock::new(HashMap::new()),
		}
	}
}
