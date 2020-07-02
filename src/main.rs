extern crate tictac;

use std::io::{stdin, stdout, Write};
use tictac::game::*;
use tictac::game::entry::*;
use tictac::game::entry::Entry::{X, O};
use tictac::ai::{AI, Minimax};
use tictac::ai::monte::{Monte};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
	let mut b = ult::Ult::new();
	let mut winner: Option<Entry> = None;
	let mut turn = X;

	let mut ai1 = Monte::new(50_000, turn, b.generate_moves(turn));
	let mut ai2 = Monte::new(10_000, turn, b.generate_moves(turn));
	// let mut ai2 = Minimax::new();

	while let None = winner {

		let mv = if turn == X {
			// println!("{}", b);
			// let mvs = b.generate_moves(turn).1;
			// get_move(turn, mvs)?
			ai1.get_move(&b) as usize
		} else {
			ai2.get_move(&b) as usize
		};
		
		b.apply_move(turn, mv);
		ai1.apply_move(turn, mv, b.generate_moves(turn));
		ai2.apply_move(turn, mv, b.generate_moves(turn));
		print!("\x1B[2J");
		println!("{}", b);

		winner = b.check_winner();
		turn = turn.flip();
	}

	println!("{}", b);
	Ok(())
}

#[allow(dead_code)]
fn get_move(turn: Entry, mvs: Vec<usize>) -> Result<usize> {
	println!("{:?}'s turn:", turn);
	stdout().flush()?;

	let min = mvs.iter().min().unwrap();
	let max = mvs.iter().max().unwrap();
	let mut g_mv;

	loop {
		let b_mv = if max / 9 == min / 9 {
			max / 9
		} else {
			print!("Select Board:");
			stdout().flush()?;
			get_location().unwrap()
		};

		print!("Select Move:");
		stdout().flush()?;
		let l_mv = get_location().unwrap();
		g_mv = b_mv * 9 + l_mv;

		if mvs.contains(&g_mv) {
			break;
		} else {
			println!("Invalid Move!");
		}
	}

	Ok(g_mv)
}

fn get_location() -> Result<usize> {
	let mut input = String::new();
	stdin().read_line(&mut input)?;
	match input.chars().nth(0) {
		Some('q') => Ok(0),
		Some('w') => Ok(1),
		Some('e') => Ok(2),
		Some('a') => Ok(3),
		Some('s') => Ok(4),
		Some('d') => Ok(5),
		Some('z') => Ok(6),
		Some('x') => Ok(7),
		Some('c') => Ok(8),
		_ => Ok(99),
	}
}
