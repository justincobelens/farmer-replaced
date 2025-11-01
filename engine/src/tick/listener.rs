pub trait TickListener: Send + Sync + 'static {
	fn on_tick(&self, dt: f64);
}
