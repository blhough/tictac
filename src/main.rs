mod ai;
mod game;

use std::io::{stdin, stdout, Write};
use board::*;
use entry::*;
use game::*;
use ult::*;
use entry::Entry::{X, O};
use rand::Rng;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
	let mut b = Ult::new();
	let mut winner: Option<Entry> = None;
	let mut turn = X;
	// let mut rng = rand::thread_rng();

	// let mvs = b.generate_moves(O).1;
	// let rnd = rng.gen_range(0, mvs.len());
	// b.ents[mvs[rnd]] = O;

	while let None = winner {

		let mv = if turn == X {
			println!("{}", b);
			let mvs = b.generate_moves(turn).1;
			get_move(turn, mvs)?
		} else {
			ai::get_move(&b) as usize
		};

		b.apply_move(turn, mv);
		

		winner = b.check_winner();
		println!("{:?}", winner);

		turn = if let X = turn { O } else { X };
	}

	Ok(())
}

fn get_move(turn: Entry, mvs: Vec<usize>) -> Result<usize> {
	println!("{:?}'s turn:", turn);
	stdout().flush()?;

	let min = mvs.iter().min().unwrap();
	let max = mvs.iter().max().unwrap();

	let b_mv = if max - min < 9 {
		max / 9
	} else {
		print!("Select Board:");
		stdout().flush()?;
		get_location().unwrap()
	};

	print!("Select Move:");
	stdout().flush()?;
	let l_mv = get_location().unwrap();

	Ok(b_mv * 9 + l_mv)
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
		_ => Ok(8),
	}
}
