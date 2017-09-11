

use std::fmt::{Display, Result, Formatter};

#[derive(Debug, Clone)]
pub struct Position {
	line: usize,
	pub col: usize,
	file: String
}


impl Position {
	pub fn new(file: String) -> Position {
		Position {
			line: 1,
			col: 1,
			file: file
		}
	}

	pub fn next_line(&mut self) {
		self.line += 1;
		self.col = 1;
	}
}

impl Display for Position {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "\"{}\":{}:{}", self.file, self.line, self.col)
	}
}

