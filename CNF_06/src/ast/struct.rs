use std::rc::Rc;
use std::cell::RefCell;

enum Operator {
	And,
	Or,
	Not,
	Lit(String),
}

struct Node {
	name: String,
	operator: Operator,
	left: Option<Rc<RefCell<Node>>>,
	right: Option<Rc<RefCell<Node>>>,
	parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
	fn new(name: String, operator: Operator) -> Self {
		Node {
			name,
			operator,
			left: None,
			right: None,
			parent: None,
		}
	}
}
