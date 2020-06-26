use crate::game::{Move, Game, Player};
use crate::entry::{Entry};
use crate::entry::Entry::*;

fn eval<G: Game<Entry>>(b: &G, d: i32) -> i32 {
	match b.check_winner() {
		Some(X) => 100 + d,
		Some(O) => -100 -d,
		_ => 0,
	}
}

pub fn get_move<G: Game<Entry>>(b: &G) -> Move {
	get_move_r(b, 1, O, -1000, 1000).1
}

fn get_move_r<G: Game<Entry>>(s: &G, d: i32, e: Entry, mut a: i32, b: i32) -> (i32, Move) {
	if d == 0 {
		(e.to_int() * eval(s, d), 0)
	} else {
		let mvs = s.generate_moves(e);
		println!("{:?}", mvs.1);

		if mvs.1.len() == 0 || s.check_winner().is_some() {
			(e.to_int() * eval(s, d), 0)
		} else {
			let mut value = -1000000;
			let mut best_mv = 0;

			for &mv in &mvs.1 {
				let mut new_s = s.clone();
				new_s.apply_move(e, mv);
				let next_val = -get_move_r(&new_s, d-1, e.flip(), -b, -a).0;

				if next_val > value {
					value = next_val;
					best_mv = mv;
				}

				if next_val > a {
					a = next_val;
				}

				if a >= b { break; }
			}

			(value, best_mv)
		}
	}
}
