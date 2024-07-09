use std::rc::Rc;
use std::cell::RefCell;

pub enum Op {
	And,
	Or,
	Not,
	Lit(String),
}

pub struct Node {
	pub name: String,
	pub operator: Op,
	pub left: Option<Rc<RefCell<Node>>>,
	pub right: Option<Rc<RefCell<Node>>>,
	pub parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
	pub fn new(name: String, operator: Op) -> Self {
		Node {
			name,
			operator,
			left: None,
			right: None,
			parent: None,
		}
	}
}
