use std::any::Any;

pub trait ResourceLike: Send + Sync + 'static {
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T> ResourceLike for T
where
	T: Any + Send + Sync + 'static,
{
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}
}

// Marker trait for resources
//
// Explanation to furture self:
// This trait is used to mark types as resources.
// If only ResourceLike is used, then any type that is Send + Sync + 'static
// would be considered a resource, which is not the intended behavior.
pub trait Resource: ResourceLike {}
