mod tic;

use std::io::{stdin, stdout, Write};
use tic::{Board, Entry};
use tic::Entry::{X, O};

fn main() -> std::io::Result<()> {
	let mut b = Board::new();
	let mut winner: Option<Entry> = None;
	let mut turn = X;

	while let None = winner {
		print!("{:?}'s turn: ", turn);
		stdout().flush()?;

		let mut input = String::new();
		stdin().read_line(&mut input)?;

		let m = input[0..1].parse::<usize>().unwrap();

		b.ents[m] = turn;
		
		println!("{}", b);

		winner = b.check_winner();
		println!("{:?}", winner);

		turn = if let X = turn { O } else { X };
	}

	Ok(())
}
