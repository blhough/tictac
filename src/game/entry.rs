extern crate colour;

use crate::game::Player;
use Entry::*;
use colour::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Entry {
	E,
	X,
	O,
}

impl std::fmt::Display for Entry {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		match self {
			E => print!("☐"),
			X => cyan!("X"),
			O => red!("O"),
		};
		Ok(())
	}
}

impl Player for Entry {
	fn to_int(&self) -> i32 {
		match self {
			E => 0,
			X => 1,
			O => -1,
		}
	}

	fn flip(&self) -> Entry {
		match self {
			E => E,
			X => O,
			O => X,
		}
	}
}