use std::rc::Rc;
use std::cell::RefCell;

use crate::set::Set;


#[derive(Clone)]
pub enum Op {
	And,
	Or,
	Not,
	Lit(String),
}

#[derive(Clone)]
pub struct Node {
	pub name: String,
	pub operator: Op,
	pub left: Option<Rc<RefCell<Node>>>,
	pub right: Option<Rc<RefCell<Node>>>,
	pub parent: Option<Rc<RefCell<Node>>>,
	pub value : Option<Rc<RefCell<Set>>>,
}

impl Node {
	pub fn new(name: String, operator: Op) -> Self {
		Node {
			name,
			operator,
			left: None,
			right: None,
			parent: None,
			value: None,
		}
	}
}
