use crate::engine::actor_registry::ActorId;

#[derive(Debug)]
pub struct Grid {
	tiles: Vec<Option<ActorId>>,
	rows: usize,
	cols: usize,
}

impl Grid {
	pub(super) fn new(rows: usize, cols: usize) -> Grid {
		Grid {
			tiles: vec![None; rows * cols],
			rows,
			cols,
		}
	}

	#[inline]
	fn idx(&self, row: usize, col: usize) -> usize {
		row * self.cols + col
	}

	pub fn size(&self) -> (usize, usize) {
		(self.rows, self.cols)
	}

	pub fn get(&self, row: usize, col: usize) -> Option<ActorId> {
		self.tiles[self.idx(row, col)]
	}

	pub fn is_empty(&self, row: usize, col: usize) -> bool {
		self.get(row, col).is_none()
	}

	pub fn clear(&mut self, row: usize, col: usize) -> Option<ActorId> {
		let i = self.idx(row, col);
		self.tiles[i].take()
	}
}
