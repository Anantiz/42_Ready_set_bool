use crate::ast::node::*;

use std::rc::Rc;
use std::cell::RefCell;

pub struct TreeIterator {
	iterator_root: Rc<RefCell<Node>>,
	current_node: Option<Rc<RefCell<Node>>>,
}
impl Node
{
	pub fn new_iterator(&self) -> TreeIterator {
		TreeIterator {
			iterator_root: self.duplicate().to_rc(),
			current_node: Some(self.duplicate().to_rc()),
		}
	}
}

impl TreeIterator
{
	pub fn next_node(&mut self) -> Option<Rc<RefCell<Node>>>
	{
		if self.current_node.is_none() {
			return None;
		}
		if self.current_node.as_ref().unwrap().borrow().left.is_some() {
			return self.current_node.as_ref().unwrap().borrow().left.clone();
		}
		if self.current_node.as_ref().unwrap().borrow().right.is_some() {
			return self.current_node.as_ref().unwrap().borrow().right.clone();
		}
		else {
			let mut current_node = self.current_node.as_ref().unwrap().borrow().clone();
			loop {
				if current_node.parent.is_none() {
					return None;
				}

				let parent_right = current_node.parent.clone().unwrap().borrow().right.clone();
				if parent_right.is_some()
				{
					if parent_right.clone().unwrap().borrow().name == current_node.name {
						current_node = current_node.parent.clone().unwrap().borrow().clone();
						if current_node.name == self.iterator_root.borrow().name {
							self.current_node = None;
							return None;
						} else {
							println!("Current name is {}, root name is {}", current_node.name, self.iterator_root.borrow().name);
						}
					}
					else {
						let ret = current_node.parent.clone().unwrap().borrow().right.clone();
						self.current_node = ret.clone();
						return ret;
					}
				}
				else {
					current_node = current_node.parent.clone().unwrap().borrow().clone();
				}
			}
		}
	}
}
