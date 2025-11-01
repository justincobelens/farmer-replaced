#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ActorId(pub u64);

impl ActorId {
	pub const INVALID: ActorId = ActorId(0);
	pub fn is_valid(&self) -> bool {
		self.0 != 0
	}
}
