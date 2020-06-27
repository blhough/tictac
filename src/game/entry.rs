extern crate colored;

use colored::{Colorize};
use crate::game::Player;
use Entry::*;

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
	Last,
}

impl Entry {
	pub fn print(&self, c: Color, w: Option<Entry>) {
		match c {
			Color::Unset => match self {
				E => print!("☐ "),
				X => print!("{}", "✕ ".cyan()),
				O => print!("{}", "⭘ ".red()),
			},
			Color::Open => match self {
				E => print!("{}", "☐ ".yellow()),
				X => print!("{}", "✕ ".cyan()),
				O => print!("{}", "⭘ ".red()),
			},
			Color::Closed => match w {
				Some(X) => match self {
					E => print!("{}", "☐ ".cyan()),
					_ => print!("{}", "◼ ".cyan()),
				},
				Some(O) => match self {
					E => print!("{}", "☐ ".red()),
					_ => print!("{}", "◼ ".red()),
				},
				_ => self.print(Color::Unset, None),
			},
			Color::Last => match self {
				E => print!("☐ "),
				X => print!("{}", "✕ ".black().on_cyan()),
				O => print!("{}", "⭘ ".black().on_red()),
			},
		}
	}
}

impl std::fmt::Display for Entry {
	fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		match self {
			E => print!("☐"),
			X => print!("X"),
			O => print!("O"),
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