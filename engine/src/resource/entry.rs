use std::{any::Any, sync::Arc};

use crate::resource::resource::ResourceLike;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ResourceId(pub u64);

impl ResourceId {
	pub const INVALID: ResourceId = ResourceId(0);
	pub fn is_valid(&self) -> bool {
		self.0 != 0
	}
}

pub struct ResourceEntry {
	pub id: ResourceId,
	pub actor: Arc<dyn ResourceLike>,
	pub erased: Arc<dyn Any + Send + Sync>,
}

impl ResourceEntry {
	pub fn downcast<A: ResourceLike>(&self) -> Option<Arc<A>> {
		Arc::downcast::<A>(self.erased.clone()).ok()
	}
}
