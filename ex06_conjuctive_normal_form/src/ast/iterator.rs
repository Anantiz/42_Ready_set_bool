use crate::ast::node::*;

use std::rc::Rc;
use std::cell::RefCell;

pub struct TreeIterator {
	stack: Vec<Rc<RefCell<Node>>>,
}
impl Node
{
	pub fn new_iterator(root : Rc<RefCell<Node>>) -> TreeIterator {
		fn inner(node : Rc<RefCell<Node>>, stack: &mut Vec<Rc<RefCell<Node>>>) {
			stack.push(node.clone());
			if node.borrow().right.is_some() {
				inner(node.borrow().clone().right.unwrap(), stack);
			}
			if node.borrow().left.is_some() {
				inner(node.borrow().clone().left.unwrap(), stack);
			}
		}
		let mut it = TreeIterator { stack: Vec::new() };
		inner(root, &mut it.stack);
		it

}
}

impl TreeIterator
{
	pub fn next_node(&mut self) -> Option<Rc<RefCell<Node>>> {
		self.stack.pop()
	}
}
