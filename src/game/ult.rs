use crate::entry::*;
use crate::game::*;
use crate::entry::Entry::*;
use crate::game::board::TicTac;

#[derive(Debug, Clone)]
pub struct Ult {
	pub brds: Vec<TicTac>,
	last_move: Option<Move>,
}

impl Ult {
	pub fn new() -> Ult {
		Ult {
			last_move: None,
			brds: vec![TicTac::new(); 9]
		}
	}
}

impl Game<Entry> for Ult {
	fn check_winner(&self) -> Option<Entry> {
		// [
		// 	check_row(self.ents[0], self.ents[1], self.ents[2]),
		// 	check_row(self.ents[3], self.ents[4], self.ents[5]),
		// 	check_row(self.ents[6], self.ents[7], self.ents[8]),

		// 	check_row(self.ents[0], self.ents[3], self.ents[6]),
		// 	check_row(self.ents[1], self.ents[4], self.ents[7]),
		// 	check_row(self.ents[2], self.ents[5], self.ents[8]),

		// 	check_row(self.ents[0], self.ents[4], self.ents[8]),
		// 	check_row(self.ents[6], self.ents[4], self.ents[2]),
		// ].iter().flatten().map(|&e| e).next()
		None
	}

	fn generate_moves(&self, ent: Entry) -> Moves<Entry> {
		if let Some(last_m) = self.last_move {
			let mut local_moves = self.brds[last_m / 9].generate_moves(ent).1;

			// Can't send the player to the previous board.
			if let Some(pos) = local_moves.iter().position(|&x| x == last_m % 9) {
				local_moves.remove(pos);
			}

			// If there are no moves in the local board, the player can play anywhere.
			if local_moves.len() == 0 {

			}

			let global_moves = local_moves.iter().map(|x| x + last_m % 9 * 9).collect::<Vec<_>>();

			Moves(ent, global_moves)
		}	else {
			self.brds[0].generate_moves(ent)
		}
	}

	fn apply_move(&mut self, e: Entry, m: Move) {
		self.last_move = Some(m);
		self.brds[m / 9].ents[m % 9] = e;
	}
}

fn check_row(a: Entry, b: Entry, c: Entry) -> Option<Entry> {
	match (a, b, c) {
		(X, X, X) => Some(X),
		(O, O, O) => Some(O),
		_ => None,
	}
}

impl std::fmt::Display for Ult {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		for i in 0..3 {
			for j in 0..3 {
				for k in 0..3 {
					for l in 0..3 {
						write!(f, "{} ", self.brds[i*3+k].ents[j*3+l])?;
					}
					write!(f, " ")?;
				}
				writeln!(f, "")?;
			}
			writeln!(f, "")?;
		}

		Ok(())
	}
}