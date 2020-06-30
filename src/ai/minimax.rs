use crate::game::{Move, Game, Player};
use crate::entry::{Entry};
use crate::entry::Entry::*;
use crate::ai::{AI, Minimax};

impl Minimax {
	pub fn new() -> Minimax {
		Minimax{}
	}
}

impl<G: Game<Entry>> AI<G> for Minimax {
	fn get_move(&mut self, b: &G) -> Move {
		get_move_r(b, 11, O, -10000, 10000).1
	}
}

fn get_move_r<G: Game<Entry>>(s: &G, d: i32, e: Entry, mut a: i32, b: i32) -> (i32, Move) {
	if d == 0 {
		(e.to_int() * s.eval(d), 0)
	} else {
		let mvs = s.generate_moves(e);

		if mvs.len() == 0 || s.check_winner().is_some() {
			(e.to_int() * s.eval(d), 0)
		} else {
			let mut value = -1000000;
			let mut best_mv = 0;
			let mut vs = Vec::new();

			for &mv in &mvs {
				let mut new_s = s.clone();
				new_s.apply_move(e, mv);
				let next_val = -get_move_r(&new_s, d-1, e.flip(), -b, -a).0;

				if d == 11 {
					vs.push(next_val);
				}

				if next_val > value {
					value = next_val;
					best_mv = mv;
				}

				if next_val > a {
					a = next_val;
				}

				if a >= b { break; }
			}

			if d == 11 {
				println!("{:?}", mvs);
				println!("{:?}", vs);
			}

			(value, best_mv)
		}
	}
}
