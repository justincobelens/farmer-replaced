#[derive(Debug, Clone)]
pub enum ActorEvent {
	BeginPlay,
	Tick(f64),
	EndPlay,
}
