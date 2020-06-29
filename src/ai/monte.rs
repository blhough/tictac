use crate::game::{Move, Game, Player};
use crate::entry::{Entry};
use crate::entry::Entry::*;
use crate::ai::{AI};

use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::f64::consts;

type ID = usize;

pub struct Monte {
	curr_id: ID,
	root: ID,
	nodes: HashMap<ID, Node>,
}

#[derive(Debug)]
struct Node {
	mv: Move,
	wins: i32,
	moves: Vec<Move>,
	nodes: Vec<ID>,
	player: Entry,
	visits: i32,
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

	fn random_child(&self) -> ID {
		let mut rng = rand::thread_rng();
		let ind = rng.gen_range(0, self.nodes.len());
		self.nodes[ind]
	}
}

impl<G: Game<Entry>> AI<G> for Monte {
	fn get_move(&mut self, g: &G) -> Move {

		for _ in 0..10_000 {
			let mut g2 = g.clone();
			let path = self.traverse(&mut g2);
			// println!("{:?}", &path);
			let res = self.playout(g2, *path.last().unwrap());
			self.backpropagate(path, res);
		}

		// println!("{:#?}", self.root.borrow());
		// for n in self.root.borrow().nodes.iter() {
		// }
		let root = self.nodes.get(&self.root).unwrap();
		let mut max = 0;
		let mut mv = 0;
		println!("{:?}\n", &root);

		for n in root.nodes.iter() {
			let nn = self.nodes.get(&n).unwrap();
			println!("{:?}", &nn);
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
	pub fn new(player: Entry, moves: Vec<Move>) -> Monte {
		let node = Node::new(0, player.flip(), moves);
		let mut nodes = HashMap::new();
		nodes.insert(0, node);
		Monte{
			curr_id: 0,
			root: 0,
			nodes,
		}
	}

	pub fn apply_move(&mut self, e: Entry, m: Move, moves: Vec<Move>) {
		let root = self.nodes.get(&self.root).unwrap();


		let child = root.nodes.iter().find(|&x| self.nodes.get(&x).unwrap().mv == m).unwrap();

		// let child = if let Some(c) = root.nodes.iter().find(|&x| self.nodes.get(&x).unwrap().mv == m) {
		// 	*c
		// } else {
		// 	let id = self.next_id();
		// 	let n = Node::new(m, e, moves);
		// 	self.nodes.insert(id, n);
		// 	id
		// };
		self.root = *child;
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

			// println!("{:#?}", nn);
			n = self.best_child(n);
		}

		if let Some(nn) = self.nodes.get(&n) {
			// println!("{:?}", nn.moves);
			// Explore a new move
			if nn.moves.len() > 0 {
				// println!("here2");
				let id = self.next_id();
				let nn = self.nodes.get_mut(&n).unwrap();
				let mv = nn.moves.pop().unwrap();
				let player = nn.player.flip();
				g.apply_move(player, mv);

				let mut mvs = g.generate_moves(player).1;
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
		let mut rng = rand::thread_rng();
		let mut p = nn.player.flip();
		// println!("{}", g);

		while w.is_none() {
			let mvs = g.generate_moves(p).1;
			
			if mvs.len() == 0 {
				w = Some(E);
				break;
			}

			let ind = rng.gen_range(0, mvs.len());
			g.apply_move(p, mvs[ind]);
			w = g.check_winner();
			p = p.flip();

			// println!("{}", g);
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

// # main function for the Monte Carlo Tree Search 
// def monte_carlo_tree_search(root): 
	
// 	while resources_left(time, computational power): 
// 		leaf = traverse(root) 
// 		simulation_result = rollout(leaf) 
// 		backpropagate(leaf, simulation_result) 
		
// 	return best_child(root) 

// # function for node traversal 
// def traverse(node): 
// 	while fully_expanded(node): 
// 		node = best_uct(node) 
		
// 	# in case no children are present / node is terminal 
// 	return pick_univisted(node.children) or node 

// # function for the result of the simulation 
// def rollout(node): 
// 	while non_terminal(node): 
// 		node = rollout_policy(node) 
// 	return result(node) 

// # function for randomly selecting a child node 
// def rollout_policy(node): 
// 	return pick_random(node.children) 

// # function for backpropagation 
// def backpropagate(node, result): 
// 	if is_root(node) return
// 	node.stats = update_stats(node, result) 
// 	backpropagate(node.parent) 

// # function for selecting the best child 
// # node with highest number of visits 
// def best_child(node): 
// 	pick child with highest number of visits 
