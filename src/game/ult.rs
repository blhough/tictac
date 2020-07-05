use crate::game::entry::*;
use crate::game::*;
use crate::game::entry::Entry::*;
use crate::game::board::TicTac;

use rand::Rng;
use std::collections::HashSet;
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub struct Ult {
	pub brds: Vec<TicTac>,
	wins: Vec<Option<Entry>>,
	last_move: Option<Move>,
	open: [HashSet<Move>; 9],
}

impl Ult {
	pub fn new() -> Ult {
		Ult {
			last_move: None,
			brds: vec![TicTac::new(); 9],
			wins: vec![None; 9],
			open: [
				(00..09).collect(),
				(09..18).collect(),
				(18..27).collect(),
				(27..36).collect(),
				(36..45).collect(),
				(45..54).collect(),
				(54..63).collect(),
				(63..72).collect(),
				(72..81).collect()],
		}
	}
}

fn check_row(a: Option<Entry>, b: Option<Entry>, c: Option<Entry>) -> Option<Entry> {
	match (a, b, c) {
		(Some(X), Some(X), Some(X)) => Some(X),
		(Some(O), Some(O), Some(O)) => Some(O),
		_ => None,
	}
}

impl Game<Entry> for Ult {
	fn check_winner(&self) -> Option<Entry> {
		[
			check_row(self.wins[0], self.wins[1], self.wins[2]),
			check_row(self.wins[3], self.wins[4], self.wins[5]),
			check_row(self.wins[6], self.wins[7], self.wins[8]),

			check_row(self.wins[0], self.wins[3], self.wins[6]),
			check_row(self.wins[1], self.wins[4], self.wins[7]),
			check_row(self.wins[2], self.wins[5], self.wins[8]),

			check_row(self.wins[0], self.wins[4], self.wins[8]),
			check_row(self.wins[6], self.wins[4], self.wins[2]),
		].iter().flatten().map(|&e| e).next()
	}

	fn generate_moves(&self, _ent: Entry) -> Moves {
		if let Some(last_m) = self.last_move {
			let mvs = &self.open[last_m % 9];

			match mvs.len() {
				// If there are no moves in the local board, the player can play anywhere.
				0 => self.open.iter().flatten().cloned().collect(),
				// If there is just one move, the player must play there.
				1 => mvs.iter().cloned().collect(),
				// If there is more than one move available then
				// can't send the player to the previous board.
				_ => mvs.iter().cloned().filter(|&x| x % 9 != last_m / 9).collect(),
			}
		} else {
			(0..81).collect()
		}
	}

	fn generate_random_move(&self, _player: Entry) -> Option<Move> {
		if let Some(last_m) = self.last_move {
			let mvs = &self.open[last_m % 9];

			match mvs.len() {
				// If there are no moves in the local board, the player can play anywhere.
				0 => {
					let set = self.open.choose_weighted(
							&mut rand::thread_rng(),
							|x| x.len())
						.ok()?;
					let r = rand::thread_rng().gen_range(0, set.len());
					set.iter().nth(r).map(|&x| x)
				},
				// If there is just one move, the player must play there.
				1 => mvs.iter().cloned().nth(0),
				// If there is more than one move available then
				// can't send the player to the previous board.
				_ => {
					let mut mvs = mvs.clone();
					mvs.remove(&last_m);
					let r = rand::thread_rng().gen_range(0, mvs.len());
					mvs.iter().nth(r).map(|&x| x)
				},
			}
		} else {
			Some(rand::thread_rng().gen_range(0, 81))
		}

		// let mvs = self.generate_moves(player);
		// match mvs.len() {
		// 	0 => None,
		// 	_ => {
		// 		let ind = rand::thread_rng().gen_range(0, mvs.len());
		// 		Some(mvs[ind])
		// 	}
		// }
	}

	fn apply_move(&mut self, e: Entry, m: Move) {
		self.last_move = Some(m);
		self.brds[m / 9].apply_move(e, m % 9);
		self.open[m / 9].remove(&m);

		if self.wins[m / 9].is_none() {
			self.wins[m / 9] = self.brds[m / 9].check_winner();
		}
	}

	fn eval(&self, depth: i32) -> i32 {
		match self.check_winner() {
			Some(X) =>  1000 + depth * 5,
			Some(O) => -1000 - depth * 5,
			_ => {
				let i = self.wins.iter().flat_map(|&w| w).collect::<Vec<_>>();
				let x = i.iter().filter(|&w| *w == X).count() as i32 * (depth + 1) *  10;
				let y = i.iter().filter(|&w| *w == O).count() as i32 * (depth + 1) * -10;
				x + y
			},
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
						let mv = b_ind * 9 + e_ind;
						if mvs.contains(&mv) {
							b.ents[e_ind].print(Color::Open, None);
						} else if self.last_move.is_some() && self.last_move.unwrap() == mv {
							b.ents[e_ind].print(Color::Last, None);
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
