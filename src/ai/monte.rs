
use crate::game::{Move, Game, Player};
use crate::entry::{Entry};
use crate::entry::Entry::*;
use crate::ai::{AI};

use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

type Nodes = Vec<Rc<RefCell<Node>>>;

pub struct Monte {
	root: Rc<RefCell<Node>>,
}

#[derive(Debug)]
struct Node {
	pub mv: Move,
	player: Entry,
	pub moves: Vec<Move>,
	pub nodes: Nodes,
	visits: i32,
	wins: i32,
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

	fn random_child(&self) -> Rc<RefCell<Node>> {
		let mut rng = rand::thread_rng();
		let ind = rng.gen_range(0, self.nodes.len());
		self.nodes[ind].clone()
	}
}

impl<G: Game<Entry>> AI<G> for Monte {
	fn get_move(&mut self, g: &G) -> Move {
		let mut g = g.clone();
		for _ in 0..100 {
			let path = self.traverse(&mut g);
			let res = self.playout(g.clone(), path.last().unwrap().clone());
			self.backpropagate(path, res);
		}

		// println!("{:#?}", self.root.borrow());
		// for n in self.root.borrow().nodes.iter() {
		// }
		self.root.borrow().nodes[0].borrow().mv
	}
}

impl Monte {
	pub fn new(player: Entry, moves: Vec<Move>) -> Monte {
		let node = Node::new(0, player, moves);
		Monte{ root: Rc::new(RefCell::new(node)) }
	}

	pub fn apply_move(&mut self, e: Entry, m: Move, moves: Vec<Move>) {
		let child = if let Some(c) = self.root.borrow().nodes.iter().find(|&x| x.borrow().mv == m) {
			c.clone()
		} else {
			Rc::new(RefCell::new(Node::new(m, e, moves)))
		};
		self.root = child;
	}

	fn traverse<G: Game<Entry>>(&self, g: &mut G) -> Nodes {
		let mut nodes = Vec::new();
		let mut n = self.root.clone();
		
		loop {
			nodes.push(n.clone());

			let count = n.borrow().moves.len();
			if count > 0 { break; } 
			let count = n.borrow().nodes.len();
			if count == 0 { break; } 

			let nn = n.borrow().random_child();
			{
				let b = nn.borrow();
				g.apply_move(b.player, b.mv);
			}
			n = nn;
		}

		let rc_n = nodes.pop().unwrap();
		{
			let mut b = rc_n.borrow_mut();

			if b.moves.len() == 0 {
				nodes.push(rc_n.clone());
				return nodes;
			}

			let mv = b.moves.pop().unwrap();

			g.apply_move(b.player, b.mv);

			let next_p = b.player.flip();
			let new_n = Node::new(mv, next_p, g.generate_moves(next_p).1);

			b.nodes.push(Rc::new(RefCell::new(new_n)));
		}
		nodes.push(rc_n);
		nodes
	}

	fn playout<G: Game<Entry>>(&self, mut g: G, n: Rc<RefCell<Node>>) -> Entry {
		let b = n.borrow();
		let mut w = None;
		let p = b.player.flip();
		let mut rng = rand::thread_rng();

		while w.is_none() {
			let mvs = g.generate_moves(p).1;

			if mvs.len() == 0 {
				w = Some(E);
				break;
			}

			let ind = rng.gen_range(0, mvs.len());
			g.apply_move(p, mvs[ind]);
			w = g.check_winner();
		}

		w.unwrap()
	}

	fn backpropagate(&self, nodes: Nodes, res: Entry) {
		for n in nodes {
			let mut b = n.borrow_mut();
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
