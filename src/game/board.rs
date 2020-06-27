use crate::entry::*;
use crate::game::*;
use crate::entry::Entry::*;

#[derive(Debug, Clone)]
pub struct TicTac {
	pub ents: [Entry; 9],
}

impl TicTac {
	pub fn new() -> TicTac {
		TicTac { ents: [Entry::E; 9] }
	}
}

// impl<'a, T, U: Player> Game<U> for &'a T where T: Game<U> {}
// impl<'a, T, U: Player> Game<U> for &'a mut T where T: Game<U> {}

impl Game<Entry> for TicTac {
	fn check_winner(&self) -> Option<Entry> {
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

	fn generate_moves(&self, ent: Entry) -> Moves<Entry> {
		let moves = self.ents
			.iter()
			.enumerate()
			.filter(|x| *x.1 == E)
			.map(|x| x.0)
			.collect::<Vec<_>>();
		Moves(ent, moves)
	}

	fn apply_move(&mut self, e: Entry, m: Move) {
		self.ents[m] = e;
	}

	fn eval(&self, d: i32) -> i32 {
		match self.check_winner() {
			Some(X) => 100 + d,
			Some(O) => -100 -d,
			_ => 0,
		}
	}
}

fn check_row(a: Entry, b: Entry, c: Entry) -> Option<Entry> {
	match (a, b, c) {
		(X, X, X) => Some(X),
		(O, O, O) => Some(O),
		_ => None,
	}
}

impl std::fmt::Display for TicTac {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		writeln!(f, "{} {} {}", self.ents[0], self.ents[1], self.ents[2])?;
		writeln!(f, "{} {} {}", self.ents[3], self.ents[4], self.ents[5])?;
		writeln!(f, "{} {} {}", self.ents[6], self.ents[7], self.ents[8])
	}
}