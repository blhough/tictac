use Entry::*;

#[derive(Debug, Copy, Clone)]
pub enum Entry {
	E,
	X,
	O,
}

pub struct Board {
	pub ents: [Entry; 9],
}

impl Board {
	pub fn new() -> Board {
		Board { ents: [Entry::E; 9] }
	}

	pub fn check_winner(&self) -> Option<Entry> {
		[
			check_row(self.ents[0], self.ents[1], self.ents[2]),
			check_row(self.ents[3], self.ents[4], self.ents[5]),
			check_row(self.ents[6], self.ents[7], self.ents[8]),

			check_row(self.ents[0], self.ents[3], self.ents[6]),
			check_row(self.ents[1], self.ents[4], self.ents[7]),
			check_row(self.ents[2], self.ents[5], self.ents[8]),

			check_row(self.ents[0], self.ents[4], self.ents[8]),
			check_row(self.ents[6], self.ents[4], self.ents[2]),
		].iter().flatten().map(|&e| e).next()
	}
}

fn check_row(a: Entry, b: Entry, c: Entry) -> Option<Entry> {
	match (a, b, c) {
		(X, X, X) => Some(X),
		(O, O, O) => Some(O),
		_ => None,
	}
}

impl std::fmt::Display for Board {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		writeln!(f, "{:?} {:?} {:?}", self.ents[0], self.ents[1], self.ents[2])?;
		writeln!(f, "{:?} {:?} {:?}", self.ents[3], self.ents[4], self.ents[5])?;
		writeln!(f, "{:?} {:?} {:?}", self.ents[6], self.ents[7], self.ents[8])
	}
}