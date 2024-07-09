use std::rc::Rc;
use std::cell::RefCell;

impl Node {
	fn new_and(left: Option<Rc<RefCell<Node>>>, right: Option<Rc<RefCell<Node>>>, parent: Option<Rc<RefCell<Node>>>, name: String) -> Self {
		let mut node = Node::new(name, Operator::And);
		node.left = left;
		node.right = right;
		node.parent = parent;
		node
	}

	fn new_or(left: Option<Rc<RefCell<Node>>>, right: Option<Rc<RefCell<Node>>>, parent: Option<Rc<RefCell<Node>>>, name: String) -> Self {
		let mut node = Node::new(name, Operator::Or);
		node.left = left;
		node.right = right;
		node.parent = parent;
		node
	}

	fn new_not(left: Option<Rc<RefCell<Node>>>, parent: Option<Rc<RefCell<Node>>>, name: String) -> Self {
		let mut node = Node::new(name, Operator::Not);
		node.left = left;
		node.parent = parent;
		node
	}

	fn new_lit(parent: Option<Rc<RefCell<Node>>>, name: String) -> Self {
		Node::new(name, Operator::Lit(name));
		node.parent = parent;
		node
	}
}