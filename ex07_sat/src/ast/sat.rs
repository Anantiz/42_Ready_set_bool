use crate::ast::node::*;
use crate::ast::node::Op;

impl Node
{
	/// It's called dumbfuck because it's a dumbfuck implementation
	/// I don't wanna spend 2 weeks making a good one
	pub fn dumbfuck_sat(&mut self) -> Result<bool, String> {
		const MAX_SIZE : u32 = 26;
		let (mut map, map_size) = self.set_vals();
		if map_size > MAX_SIZE {
			return Err(format!("The input has to many literals, for the sake of life-expectancy we won't compute it, please refrain from input larger than {} unique literals\n\tLiterals count: {}", MAX_SIZE, map_size));
		}

		let mut counter : u128 = 0;
		let mut state : u32 = 1;
		let max : u128 = 2u128.pow(map_size);
		loop {
			if self.dumbfuck_sat_inner() {
				return Ok(true)
			}
			counter += 1;
			if counter == max {
				return Ok(false)
			}
			state = Node::var_update(&mut map, state); // Will change the literals state
		}
	}

	pub fn dumbfuck_sat_inner(&self) -> bool {
		match &self.operator {
			Op::And => {
				self.dumbfuck_sat_and()
			},
			Op::Or => {
				self.dumbfuck_sat_or()
			},
			Op::Not => {
				self.dumbfuck_sat_not()
			},
			Op::Lit(_) => {
				self.dumbfuck_sat_lit()
			},
		}

	}

	fn dumbfuck_sat_and(&self) -> bool {
		self.clone().left.unwrap().borrow().dumbfuck_sat_inner() && self.clone().right.unwrap().borrow().dumbfuck_sat_inner()
	}

	fn dumbfuck_sat_or(&self) -> bool {
		self.clone().left.unwrap().borrow().dumbfuck_sat_inner() || self.clone().right.unwrap().borrow().dumbfuck_sat_inner()
	}

	fn dumbfuck_sat_not(&self) -> bool {
		!(self.clone().left.unwrap().borrow().dumbfuck_sat_inner())
	}

	fn dumbfuck_sat_lit(&self) -> bool {
		return  *self.clone().value.unwrap().borrow();
	}
}