use derive_more::{Display, From};

pub type Result<T> = core::result::Result<T, TickError>;

#[derive(Debug, From, Display)]
pub enum TickError {
	#[display("TickError")]
	TickError,
}
