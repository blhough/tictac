use Entry::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Entry {
	E,
	X,
	O,
}

impl std::fmt::Display for Entry {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		match self {
			E => write!(f, "☐"),
			X => write!(f, "X"),
			O => write!(f, "O"),
		}
	}
}