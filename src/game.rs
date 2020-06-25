pub mod board;
pub mod entry;
pub mod ult;

pub type Move = usize;

pub struct Moves<T>(pub T, pub Vec<Move>);

pub trait Player : Copy + PartialEq + std::fmt::Display + Clone {
	fn to_int(&self) -> i32;
	fn flip(&self) -> Self;
}

pub trait Game<T: Player> : Clone {
  fn generate_moves(&self, player: T) -> Moves<T>;
  fn apply_move(&mut self, player: T, m: Move);
  fn check_winner(&self) -> Option<T>;
}