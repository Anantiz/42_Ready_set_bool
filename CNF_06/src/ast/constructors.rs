use crate::ast::node::*;

use std::rc::Rc;
use std::cell::RefCell;


impl Node {
	pub fn new_and(left: Option<Rc<RefCell<Node>>>, right: Option<Rc<RefCell<Node>>>,
		parent: Option<Rc<RefCell<Node>>>, name: String) -> Self {
		let mut node = Node::new(name, Op::And);
		node.left = left;
		node.right = right;
		node.parent = parent;
		node
	}

	pub fn new_or(left: Option<Rc<RefCell<Node>>>, right: Option<Rc<RefCell<Node>>>,
		parent: Option<Rc<RefCell<Node>>>, name: String) -> Self {
		let mut node = Node::new(name, Op::Or);
		node.left = left;
		node.right = right;
		node.parent = parent;
		node
	}

	pub fn new_not(left: Option<Rc<RefCell<Node>>>, parent: Option<Rc<RefCell<Node>>>,
		name: String) -> Self {
		let mut node = Node::new(name, Op::Not);
		node.left = left;
		node.parent = parent;
		node
	}

	pub fn new_lit(parent: Option<Rc<RefCell<Node>>>, name: String, value : String) -> Self {
		let mut node = Node::new(name, Op::Lit(value));
		node.parent = parent;
		node
	}

	pub fn to_rc(self) -> Rc<RefCell<Node>> {
		Rc::new(RefCell::new(self))
	}
}