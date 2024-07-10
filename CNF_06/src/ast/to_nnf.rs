use crate::ast::node::*;
use crate::ast::node::Op;

use std::rc::Rc;
use std::cell::RefCell;

impl Node {

	/// Will create a new tree in CNF form
	pub fn to_nnf(& self) -> Option<Rc<RefCell<Node>>>
	{
		fn inner(og_node : Rc<RefCell<Node>>)-> Option<Rc<RefCell<Node>>>
		{
			let node = og_node.borrow();
			match node.operator {
				Op::And => {
					let left = inner(node.left.clone().unwrap());
					let right = inner(node.right.clone().unwrap());
					match (left, right) {
						(Some(left), Some(right)) => Some(Node::new_and(Some(left), Some(right), node.name.clone()).to_rc()),
						_ => None,
					}
				},
				Op::Or => {
					let left = inner(node.left.clone().unwrap());
					let right = inner(node.right.clone().unwrap());
					match (left, right) {
						(Some(left), Some(right)) => Some(Node::new_or(Some(left), Some(right), node.name.clone()).to_rc()),
						_ => None,
					}
				},
				Op::Not => {
					let left = inner(node.left.clone().unwrap());
					match left {
						Some(left) => Some(Node::new_not(Some(left), node.name.clone()).to_rc()),
						_ => None,
					}
				},
				Op::Lit(_) => {
					drop(node); // Drop the borrow
					Some(og_node)
				}
			}
		}
		println!("NNF Done");
		// SAFETY: We duplicate the tree to avoid modifying the original tree
		inner(self.duplicate().to_rc())
	}
}