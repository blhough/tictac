use crate::game::{Move, Game, Player};
use crate::game::entry::{Entry};
use crate::game::entry::Entry::*;
use crate::ai::{AI};

use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::f64::consts;

type ID = usize;

pub struct Monte {
	iter: usize,
	curr_id: ID,
	root: ID,
	nodes: HashMap<ID, Node>,
}

// #[derive(Debug)]
struct Node {
	wins: i32,
	visits: i32,
	mv: Move,
	player: Entry,
	moves: Vec<Move>,
	nodes: Vec<ID>,
}

impl Node {
	fn new(mv: Move, player: Entry, moves: Vec<Move>) -> Node {
		Node {
			mv,
			player,
			moves,
			nodes: Vec::new(),
			visits: 0,
			wins: 0,
		}
	}

	#[allow(dead_code)]
	fn random_child(&self) -> ID {
		let mut rng = rand::thread_rng();
		let ind = rng.gen_range(0, self.nodes.len());
		self.nodes[ind]
	}
}

impl<G: Game<Entry>> AI<G> for Monte {
	fn get_move(&mut self, g: &G) -> Move {

		for _ in 0..self.iter {
			let mut g2 = g.clone();
			let path = self.traverse(&mut g2);
			let res = self.playout(g2, *path.last().unwrap());
			self.backpropagate(path, res);
		}

		let root = self.nodes.get(&self.root).unwrap();
		let mut max = 0;
		let mut mv = 0;
		// println!("{:?}\n", &root);

		for n in root.nodes.iter() {
			let nn = self.nodes.get(&n).unwrap();
			// println!("{:?}", &nn);
			let score = nn.visits;
			if score >= max {
				max = score;
				mv = nn.mv;
			}
		}
		mv
	}
}

impl Monte {
	pub fn new(iter: usize, player: Entry, moves: Vec<Move>) -> Monte {
		let node = Node::new(0, player.flip(), moves);
		let mut nodes = HashMap::new();
		nodes.insert(0, node);
		Monte{
			iter,
			curr_id: 0,
			root: 0,
			nodes,
		}
	}

	pub fn apply_move(&mut self, e: Entry, m: Move, moves: Vec<Move>) {
		let curr_root_id = self.root;
		let curr_root = self.nodes.get(&curr_root_id).unwrap();

		if let Some(next_root_ind) = curr_root.nodes.iter().position(|&x| self.nodes.get(&x).unwrap().mv == m) {
			let curr_root = self.nodes.get_mut(&self.root).unwrap();
			self.root = curr_root.nodes.remove(next_root_ind);
			self.delete_rec(curr_root_id);
		} else {
			let id = self.next_id();
			let n = Node::new(m, e, moves);
			self.nodes.insert(id, n);
			self.root = id;
		};
	}

	fn delete_rec(&mut self, n: ID) {
		let node = self.nodes.remove(&n).unwrap();
		for &c in node.nodes.iter() {
			self.delete_rec(c);
		}
	}

	fn next_id(&mut self) -> ID {
		self.curr_id += 1;
		self.curr_id
	}

	fn best_child(&self, n: ID) -> ID {
		let p = self.nodes.get(&n).unwrap();
		let mut max = 0.0;
		let mut best = 0;

		for n in p.nodes.iter() {
			let nn = self.nodes.get(&n).unwrap();
			let score = nn.wins as f64/ nn.visits as f64 + consts::SQRT_2 * (p.visits as f64).ln() / nn.visits as f64;
			if score >= max {
				max = score;
				best = *n;
			}
		}
		best
	}

	fn traverse<G: Game<Entry>>(&mut self, g: &mut G) -> Vec<ID> {
		let mut nodes = Vec::new();
		let mut n = self.root;
		
		while let Some(nn) = self.nodes.get(&n) {
			nodes.push(n);
			// If there there are unexplored moves or the node is terminal
			if nn.moves.len() > 0 { break; }
			if nn.nodes.len() == 0 { break; }

			if n != self.root {
				g.apply_move(nn.player, nn.mv);
			}

			n = self.best_child(n);
		}

		if let Some(nn) = self.nodes.get(&n) {
			// Explore a new move
			if nn.moves.len() > 0 {
				let id = self.next_id();
				let nn = self.nodes.get_mut(&n).unwrap();
				let mv = nn.moves.pop().unwrap();
				let player = nn.player.flip();
				g.apply_move(player, mv);

				let mut mvs = g.generate_moves(player);
				mvs.shuffle(&mut rand::thread_rng());
				nodes.push(id);
				nn.nodes.push(id);
				let child = Node::new(mv, player, mvs);
				self.nodes.insert(id, child);
			}
		}

		nodes
	}

	fn playout<G: Game<Entry>>(&self, mut g: G, n: ID) -> Entry {
		let nn = self.nodes.get(&n).unwrap();
		let mut w = g.check_winner();
		let mut p = nn.player.flip();

		while w.is_none() {
			if let Some(mv) = g.generate_random_move(p) {
				g.apply_move(p, mv);
				w = g.check_winner();
				p = p.flip();
			} else {
				return E;
			}
		}

		w.unwrap()
	}

	fn backpropagate(&mut self, nodes: Vec<ID>, res: Entry) {
		for n in nodes {
			let mut b = self.nodes.get_mut(&n).unwrap();
			b.visits += 2;
			if b.player == res {
				b.wins += 2;
			} else if res == E {
				b.wins += 1;
			}
		}
	}
}
