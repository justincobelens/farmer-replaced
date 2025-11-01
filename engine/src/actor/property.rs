use parking_lot::RwLock;

pub struct Property<T>(RwLock<T>);

impl<T> Property<T> {
	pub fn new(value: T) -> Self {
		Self(RwLock::new(value))
	}

	/// Replace the value
	pub fn set(&self, value: T) {
		*self.0.write() = value;
	}

	/// Get a cloned value
	pub fn get(&self) -> T
	where
		T: Clone,
	{
		self.0.read().clone()
	}

	/// Read-only function access without cloning
	pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
		f(&*self.0.read())
	}

	/// In-place mutation
	pub fn update(&self, f: impl FnOnce(&mut T)) {
		f(&mut *self.0.write());
	}
}
