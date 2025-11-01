use std::sync::{
	Arc,
	atomic::{AtomicBool, Ordering},
};

/// Simple toggle that can be safeley copied and shared between threads
#[derive(Debug, Clone)]
pub struct Toggle {
	state: Arc<AtomicBool>,
}

impl Toggle {
	pub fn init_on() -> Self {
		Self {
			state: Arc::new(AtomicBool::new(true)),
		}
	}

	pub fn init_off() -> Self {
		Self {
			state: Arc::new(AtomicBool::new(false)),
		}
	}

	pub fn is_on(&self) -> bool {
		self.state.load(Ordering::SeqCst)
	}

	pub fn is_off(&self) -> bool {
		!self.state.load(Ordering::SeqCst)
	}

	pub fn turn_on(&self) {
		self.state.store(true, Ordering::SeqCst);
	}

	pub fn turn_off(&self) {
		self.state.store(false, Ordering::SeqCst);
	}

	pub fn turn_on_until_dropped(&self) -> ToggleGuard {
		self.turn_on();
		ToggleGuard::new(self.state.clone(), false)
	}
	pub fn turn_off_until_dropped(&self) -> ToggleGuard {
		self.turn_off();
		ToggleGuard::new(self.state.clone(), true)
	}
}

/// Context manager that guaranees toggle returns
/// to its orginal state even if the current codeblock fails
pub struct ToggleGuard {
	state: Arc<AtomicBool>,
	return_state: bool,
}

impl ToggleGuard {
	pub fn new(state: Arc<AtomicBool>, return_state: bool) -> Self {
		Self { return_state, state }
	}

	fn undo(&self) {
		self.state.store(self.return_state, Ordering::SeqCst);
	}
}

impl Drop for ToggleGuard {
	fn drop(&mut self) {
		self.undo();
	}
}
