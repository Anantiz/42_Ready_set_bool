use crate::ast::node::*;
use crate::ast::node::Op;

use std::rc::Rc;
use std::cell::RefCell;

impl Node {

	pub fn negate(&mut self) -> Rc<RefCell<Node>>
	{
		match self.operator {
			Op::Not => self.left.clone().unwrap(),
			Op::Lit(_) => Node::new_not(Some(self.duplicate().to_rc()), self.name.clone()).to_rc(),
			Op::And => Node::new_or(
				Some(self.left.clone().unwrap().borrow_mut().negate()),
				Some(self.right.clone().unwrap().borrow_mut().negate()),
				self.name.clone()).to_rc(),
			Op::Or => Node::new_and(
				Some(self.left.clone().unwrap().borrow_mut().negate()),
				Some(self.right.clone().unwrap().borrow_mut().negate()),
				self.name.clone()).to_rc(),
		}
	}

	/// Will create a new tree in NNF form
	pub fn to_nnf(& self) -> Option<Rc<RefCell<Node>>>
	{
		fn inner(og_node : Rc<RefCell<Node>>)-> Option<Rc<RefCell<Node>>>
		{
			let mut node = og_node.borrow_mut();
			match node.operator {
				Op::Lit(_) => {
					drop(node);
					Some(og_node)
				},
				Op::Or | Op::And => {
					let left = inner(node.left.clone().unwrap());
					let right = inner(node.right.clone().unwrap());
					node.left = left;
					node.right = right;
					drop(node);
					Some(og_node)
				},
				Op::Not => {
					let mut child = node.left.clone().unwrap().borrow_mut().clone();
					match child.operator {
						Op::Lit(_) => {
							drop(node);
							Some(og_node)
						}
						_ => {
							drop(node);
							Some(child.negate())
						}
					}
				},
			}
		}
		// SAFETY: We duplicate the tree to avoid modifying the original tree
		inner(self.duplicate().to_rc())
	}
}