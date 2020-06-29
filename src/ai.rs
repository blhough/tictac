use crate::game::{Move, Game};
use crate::entry::{Entry};

pub mod minimax;
pub mod monte;

pub trait AI<G: Game<Entry>> {
  fn get_move(&mut self, g: &G) -> Move;
}

pub struct Minimax;
