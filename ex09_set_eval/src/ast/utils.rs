use crate::ast::node::*;
use crate::ast::node::Op;

use std::rc::Rc;
use std::cell::RefCell;

impl Node {

	/// Will make a deep-copy of the node and all of its children
	/// Costly operation, but allow things clone() wouldn;t
	pub fn duplicate(&self) -> Node
	{
		// Might not have parent set corectly
		match & self.operator {
			Op::And => Node::new_and(
				Some(self.left.clone().unwrap().borrow().duplicate().to_rc()),
				Some(self.right.clone().unwrap().borrow().duplicate().to_rc()),
				self.name.clone()
			),
			Op::Or => Node::new_or(
				Some(self.left.clone().unwrap().borrow().duplicate().to_rc()),
				Some(self.right.clone().unwrap().borrow().duplicate().to_rc()),
				self.name.clone()
			),
			Op::Not => Node::new_not(
				Some(self.left.clone().unwrap().borrow().duplicate().to_rc()),
				self.name.clone()
			),
			Op::Lit(_) => Node::new_lit(
				self.name.clone(),
			),
		}
	}

	/// Will create a duplicate tree that will be the negation of the original tree
	pub fn negate_subtree(& self) -> Rc<RefCell<Node>> {
		fn inner(node : Rc<RefCell<Node>>)-> Rc<RefCell<Node>> {
			// SAFETY: It is safe to unwrap the childrens because
			// we are certain that no Tree exists without children (Aside from leaves that are literals)
			let node = node.borrow();
			match node.operator {
				Op::And => Node::new_or(
					Some(inner(node.left.clone().unwrap())),
					Some(inner(node.right.clone().unwrap())),
					node.name.clone()
				).to_rc(),
				Op::Or => Node::new_and(
					Some(inner(node.left.clone().unwrap())),
					Some(inner(node.right.clone().unwrap())),
					node.name.clone()
				).to_rc(),
				Op::Not => node.left.clone().unwrap(),
				Op::Lit(_) => Node::new_not(
					Some(node.duplicate().to_rc()),
					node.name.clone()
				).to_rc(),
			}
		}
		inner(self.duplicate().to_rc())
	}

	/// Will set the 'parent' field of the childrens to point to this node
	pub fn assign_parents_to_children(& self)
	{
		match & self.operator {
			Op::And | Op::Or => {
				if self.left.is_some() {
					self.left.clone().unwrap().borrow_mut().parent = Some(self.as_rc());
				}
				if self.right.is_some() {
					self.right.clone().unwrap().borrow_mut().parent = Some(self.as_rc());
				}
			},
			Op::Not => {
				if self.left.is_some() {
					self.left.clone().unwrap().borrow_mut().parent = Some(self.as_rc());
				}
			},
			Op::Lit(_) => {},
		}
	}
}