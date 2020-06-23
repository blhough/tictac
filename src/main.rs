mod ai;

use std::io::{stdin, stdout, Write};
use ai::*;
use board::*;
use entry::*;
use entry::Entry::{X, O};
use rand::Rng;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
	let mut b = Board::new();
	let mut winner: Option<Entry> = None;
	let mut turn = O;
	// let mut rng = rand::thread_rng();

	while let None = winner {
		let mv = if turn == X {
			get_move(turn)?
		} else {
			ai::get_move(&b) as usize
			// let mvs = b.generate_moves(O).1;
			// let rnd = rng.gen_range(0, mvs.len());
			// mvs[rnd]
		};

		b.ents[mv] = turn;
		
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

	let m = input[0..1].parse::<usize>()?;
	Ok(m)
}
