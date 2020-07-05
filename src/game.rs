pub mod board;
pub mod entry;
pub mod ult;

pub type Move = usize;

pub type Moves = Vec<Move>;

pub trait Player : Copy + PartialEq + std::fmt::Display + Clone {
	fn to_int(&self) -> i32;
	fn flip(&self) -> Self;
}

pub trait Game<T: Player> : Clone + std::fmt::Display {
  fn generate_moves(&self, player: T) -> Moves;
  fn generate_random_move(&self, player: T) -> Option<Move>;
  fn apply_move(&mut self, player: T, m: Move);
  fn check_winner(&self) -> Option<T>;
  fn eval(&self, depth: i32) -> i32;
}