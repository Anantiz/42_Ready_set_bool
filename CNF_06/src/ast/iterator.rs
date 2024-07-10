use crate::ast::node::*;

use std::rc::Rc;
use std::cell::RefCell;

pub struct TreeIterator {
	stack: Vec<Rc<RefCell<Node>>>,
    current: Option<Rc<RefCell<Node>>>
}
impl Node
{
	pub fn new_iterator(&self) -> TreeIterator {
	}
}

impl TreeIterator
{
	pub fn next_node(&mut self) -> Option<Rc<RefCell<Node>>>
	{

	}
}
