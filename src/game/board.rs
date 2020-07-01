use crate::game::entry::*;
use crate::game::*;
use crate::game::entry::Entry::*;

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct TicTac {
	pub ents: [Entry; 9],
	open: Moves,
}

impl TicTac {
	pub fn new() -> TicTac {
		TicTac {
			ents: [Entry::E; 9],
			open: (0..9).collect(),
		}
	}
}

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

	fn generate_moves2(&mut self, _ent: Entry) -> &Moves {
		&self.open
	}

	fn generate_moves(&self, _ent: Entry) -> Moves {
		self.ents
			.iter()
			.enumerate()
			.filter(|x| *x.1 == E)
			.map(|x| x.0)
			.collect()
	}

	fn apply_move2(&mut self, e: Entry, m: Move) {
		self.ents[m] = e;
		let i = self.open.iter().position(|x| *x == m).unwrap();
		self.open.remove(i);
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