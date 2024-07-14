use crate::ast::node::*;
use crate::ast::node::Op;

use crate::set::Set;
use std::collections::HashMap;

use std::rc::Rc;
use std::cell::RefCell;

impl Node
{
	pub fn set_vals_rec(&mut self, map: &mut HashMap<String, Rc<RefCell<Set>>>,
		size: Rc<RefCell<u32>>, sets: &Vec<Rc<RefCell<Set>>>) -> Result<(), String>
	{
		match self.operator {
			Op::Lit(ref name) => {
				if !map.contains_key(name) {
					let set = sets.get(*size.borrow() as usize);
					if set.is_none() {
						return Err(format!("Error: Could not find a set to bind with literal '{}'", name));
					}
					let set = set.unwrap();
					map.insert(name.clone(), set.clone());
					self.value = Some(map[name].clone());
					*size.borrow_mut() += 1;
				}
				self.value = Some(map[name].clone());
				return Ok(());
			},
			Op::Not => {
				let ret = self.left.as_mut().unwrap().borrow_mut().set_vals_rec(map, size.clone(), sets);
				if  ret.is_err() {
					return Err(ret.err().unwrap());
				}
				return Ok(());
			},
			_ => {
				let ret = self.left.as_mut().unwrap().borrow_mut().set_vals_rec(map, size.clone(), sets);
				if ret.is_err() {
					return Err(ret.err().unwrap());
				}
				let ret = self.right.as_mut().unwrap().borrow_mut().set_vals_rec(map, size.clone(), sets);
				if ret.is_err() {
					return Err(ret.err().unwrap());
				}
				return Ok(());
			},
		}
	}

	/// Will store any literals in the hashmap, and return the hashmap
	/// The map will hold pointers that are duplicated in the trees for associated literals
	/// By changing values in the map, the values in the trees will also change
	pub fn set_vals(&mut self, sets: &Vec<Rc<RefCell<Set>>>) -> Result<(HashMap<String, Rc<RefCell<Set>>>, u32), String>
	{
		let mut map = HashMap::new();
		let size = Rc::new(RefCell::new(0));
		let ret = self.set_vals_rec(&mut map, size.clone(), sets);
		if ret.is_err() {
			return Err(ret.err().unwrap());
		}
		let size = *size.borrow();
		Ok((map, size))
	}
}