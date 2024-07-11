use crate::ast::node::*;
use crate::ast::node::Op;

use std::collections::HashMap;

use std::rc::Rc;
use std::cell::RefCell;

impl Node
{
	pub fn set_vals_rec(&mut self, map: &mut HashMap<String, Rc<RefCell<bool>>>, size: Rc<RefCell<u32>>) {
		match self.operator {
			Op::Lit(ref name) => {
				if !map.contains_key(name) {
					map.insert(name.clone(), Rc::new(RefCell::new(false)));
					self.value = Some(map[name].clone());
					*size.borrow_mut() += 1;
				}
				self.value = Some(map[name].clone());
			},
			Op::Not => {
				self.left.as_mut().unwrap().borrow_mut().set_vals_rec(map, size.clone());
			},
			_ => {
				self.left.as_mut().unwrap().borrow_mut().set_vals_rec(map, size.clone());
				self.right.as_mut().unwrap().borrow_mut().set_vals_rec(map, size.clone());
			},
		}
	}

	/// Will store any literals in the hashmap, and return the hashmap
	/// The map will hold pointers that are duplicated in the trees for associated literals
	/// By changing values in the map, the values in the trees will also change
	pub fn set_vals(&mut self) -> (HashMap<String, Rc<RefCell<bool>>>, u32)
	{
		let mut map = HashMap::new();
		let size = Rc::new(RefCell::new(0));
		self.set_vals_rec(&mut map, size.clone());
		let size = *size.borrow();
		(map, size)
	}

	// Purpose, update the variables such that after 2^size iterations any state will be reached
	pub fn var_update(map: &mut HashMap<String, Rc<RefCell<bool>>>, state: u32) -> u32 {
		for (i, (_, val)) in map.iter().clone().enumerate() {
			// println!("State: {}: {}", i, state & (1 << i) != 0);
			*val.borrow_mut() = state & (1 << i) != 0;
		}
		state + 1
	}
}