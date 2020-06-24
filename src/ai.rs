pub mod board;

use board::{Board, Move};
use board::entry::{Entry};
use board::entry::Entry::*;

fn eval(b: &Board, d: i32) -> i32 {
	match b.check_winner() {
		Some(X) => 100 + d,
		Some(O) => -100 -d,
		_ => 0,
	}
}

pub fn get_move(b: &Board) -> Move {
	get_move_r(&b, 9, O).1
}

fn get_move_r(b: &Board, d: i32, e: Entry) -> (i32, Move) {
	if d == 0 {
		(e.to_int() * eval(b, d), 0)
	} else {
		let mvs = b.generate_moves(e);

		if mvs.1.len() == 0 || b.check_winner().is_some() {
			(e.to_int() * eval(b, d), 0)
		} else {
			let mut value = -1000000;
			let mut best_mv = 0;
			let mut vs = [0; 9];

			for &mv in &mvs.1 {
				let b2 = b.apply_move(e, mv);
				let next_val = -get_move_r(&b2, d-1, e.flip()).0;

				if d == 9 {
					vs[mv] = next_val;
				}

				if next_val > value {
					value = next_val;
					best_mv = mv;
				}
			}

			if d == 9 { 
				// println!("{:?}", mvs.1);
				// println!("{:?}", &vs[0..3]);
				// println!("{:?}", &vs[3..6]);
				// println!("{:?}", &vs[6..9]);
			}
			(value, best_mv)
		}
	}
}
