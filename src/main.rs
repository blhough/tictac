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
	println!("{}", b);

	while let None = winner {
		let mv = if turn == X {
			get_move(turn)?
		} else {
			ai::get_move(&b) as usize
		};

		b.apply_move(turn, mv);
		
		println!("{}", b);

		winner = b.check_winner();
		println!("{:?}", winner);

		turn = if let X = turn { O } else { X };
	}

	Ok(())
}

fn get_move(turn: Entry) -> Result<usize> {
	print!("{:?}'s turn: ", turn);
	stdout().flush()?;

	let mut input = String::new();
	stdin().read_line(&mut input)?;

	let m = input[0..2].trim().parse::<usize>()?;
	Ok(m)
}
