use crate::entry::*;
use crate::game::*;
use crate::entry::Entry::*;
use crate::game::board::TicTac;

#[derive(Debug, Clone)]
pub struct Ult {
	pub brds: Vec<TicTac>,
	wins: Vec<Option<Entry>>,
	last_move: Option<Move>,
}

impl Ult {
	pub fn new() -> Ult {
		Ult {
			last_move: None,
			brds: vec![TicTac::new(); 9],
			wins: vec![None; 9],
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
			let mut local_moves = self.brds[last_m % 9].generate_moves(ent).1;

			// If there is more than one move available then
			// can't send the player to the previous board.
			if local_moves.len() > 0 {
				if let Some(pos) = local_moves.iter().position(|&x| x == last_m / 9) {
					local_moves.remove(pos);
				}
			} else { // If there are no moves in the local board, the player can play anywhere.
				let mvs = self.brds.iter()
					.enumerate()
					.flat_map(|x| x.1.generate_moves(ent).1.iter().map(|&y| y + x.0 % 9 * 9).collect::<Vec<_>>())
					.collect::<Vec<_>>();
				return Moves(ent, mvs);
			}

			let global_moves = local_moves.iter().map(|x| x + last_m % 9 * 9).collect::<Vec<_>>();

			Moves(ent, global_moves)
		}	else {
			Moves(ent, (0..=81).collect())
		}
	}

	fn apply_move(&mut self, e: Entry, m: Move) {
		self.last_move = Some(m);
		self.brds[m / 9].ents[m % 9] = e;
		if self.wins[m / 9].is_none() {
			self.wins[m / 9] = self.brds[m / 9].check_winner();
		}
	}
}

impl std::fmt::Display for Ult {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		let mvs = self.generate_moves(X);

		for i in 0..3 {
			for j in 0..3 {
				for k in 0..3 {
					for l in 0..3 {
						let b_ind = i * 3 + k;
						let e_ind = j * 3 + l;
						let b = &self.brds[b_ind];
						if mvs.1.contains(&(b_ind * 9 + e_ind)) {
							b.ents[e_ind].print(Color::Open, None);
						} else if let Some(w) = self.wins[b_ind] {
							b.ents[e_ind].print(Color::Closed, Some(w));
						} else {
							b.ents[e_ind].print(Color::Unset, None);
						}
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