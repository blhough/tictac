mod tic;

use std::io::{stdin, stdout, Write};
use tic::{Board, Entry};
use tic::Entry::{X, O};
use rand::Rng;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
	let mut b = Board::new();
	let mut winner: Option<Entry> = None;
	let mut turn = X;
	let mut rng = rand::thread_rng();

	while let None = winner {
		let mv = if turn == X {
			get_move(turn)?
		} else {
			let mvs = b.generate_moves(O).1;
			let rnd = rng.gen_range(0, mvs.len());
			mvs[rnd]
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

// function negamax(node, depth, color) is
//     if depth = 0 or node is a terminal node then
//         return color × the heuristic value of node
//     value := −∞
//     for each child of node do
//         value := max(value, −negamax(child, depth − 1, −color))
//     return value