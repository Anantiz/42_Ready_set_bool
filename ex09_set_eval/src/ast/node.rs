use std::rc::Rc;
use std::cell::RefCell;

type Set = std::collections::HashSet<u32>;

#[derive(Clone)]
pub enum Op {
	And,
	Or,
	Not,
	Lit(String),
}

#[derive(Clone)]
pub struct Node<'a> {
	pub name: String,
	pub operator: Op,
	pub left: Option<Rc<RefCell<Node>>>,
	pub right: Option<Rc<RefCell<Node>>>,
	pub parent: Option<Rc<RefCell<Node>>>,
	pub value : Option<&'aSet>, // read only ref, so no Rc<RefCell<Set>>
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
