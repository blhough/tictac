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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
	Unset,
	Open,
	Closed,
}

impl Entry {
	pub fn print(&self, c: Color, w: Option<Entry>) {
		match c {
			Color::Unset => match self {
				E => prnt!("☐ "),
				X => cyan!("X "),
				O => red!("O "),
			},
			Color::Open => match self {
				E => yellow!("☐ "),
				X => cyan!("X "),
				O => red!("O "),
			},
			Color::Closed => match w {
				Some(X) => match self {
					E => cyan!("☐ "),
					X => cyan!("X "),
					O => cyan!("O "),
				},
				Some(O) => match self {
					E => red!("☐ "),
					X => red!("X "),
					O => red!("O "),
				},
				_ => self.print(Color::Unset, None),
			},
		}
	}
}

impl std::fmt::Display for Entry {
	fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
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