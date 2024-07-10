use crate::ast::node::*;

use std::rc::Rc;
use std::cell::RefCell;


impl Node {
	pub fn new_and(left: Option<Rc<RefCell<Node>>>, right: Option<Rc<RefCell<Node>>>,
		name: String) -> Self {
		let mut node = Node::new(name, Op::And);
		node.left = left;
		node.right = right;
		node.parent = None;
		node
	}

	pub fn new_or(left: Option<Rc<RefCell<Node>>>, right: Option<Rc<RefCell<Node>>>,
		name: String) -> Self {
		let mut node = Node::new(name, Op::Or);
		node.left = left;
		node.right = right;
		node.parent = None;
		node
	}

	pub fn new_not(left: Option<Rc<RefCell<Node>>>, name: String) -> Self {
		let mut node = Node::new(name, Op::Not);
		node.left = left;
		node.parent = None;
		node
	}

	pub fn new_lit(name: String) -> Self {
		let mut node = Node::new(name, Op::Lit(name.clone()));
		node.parent = None;
		node
	}

	pub fn to_rc(self) -> Rc<RefCell<Node>> {
		Rc::new(RefCell::new(self))
	}
}