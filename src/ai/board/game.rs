pub type Move = usize;

pub struct Moves<T>(pub T, pub Vec<Move>);

pub trait Game<T> : Clone {
  fn generate_moves(&self, player: T) -> Moves<T>;
  fn apply_move(&mut self, player: T, m: Move);
  fn check_winner(&self) -> Option<T>;
}