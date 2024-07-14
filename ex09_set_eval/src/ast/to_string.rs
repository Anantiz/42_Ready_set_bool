use crate::ast::node::*;

use std::rc::Rc;
use std::cell::RefCell;

use crate::set::Set;
use crate::set::set::set_get_string;


impl Node
{
	pub fn to_rpn(&self) -> String {
		// match & self.operator {
		// 	Op::And => format!("{} {} &[{}]", self.left.clone().unwrap().borrow().to_rpn(), self.right.clone().unwrap().borrow().to_rpn(), self.name.clone()),
		// 	Op::Or => format!("{} {} |[{}]", self.left.clone().unwrap().borrow().to_rpn(), self.right.clone().unwrap().borrow().to_rpn(),self.name.clone()),
		// 	Op::Not => format!("{} ![{}]", self.left.clone().unwrap().borrow().to_rpn(), self.name.clone()),
		// 	Op::Lit(val) => format!("{}", val.clone()),
		// }
			match & self.operator {
			Op::And => format!("{} {} &", self.left.clone().unwrap().borrow().to_rpn(), self.right.clone().unwrap().borrow().to_rpn()),
			Op::Or => format!("{} {} |", self.left.clone().unwrap().borrow().to_rpn(), self.right.clone().unwrap().borrow().to_rpn()),
			Op::Not => format!("{}!", self.left.clone().unwrap().borrow().to_rpn()),
			Op::Lit(val_name) => {
				let value : Option<Rc<RefCell<Set>>> = self.value.clone();
				if value.is_none() {
					return format!("{}", val_name.clone())
				}
				let value = value.unwrap();
				return format!("{}[{}]", val_name.clone(), set_get_string(&value.borrow()));
			}
		}
	}

	pub fn private_to_infix_inner(& self) -> String {
		match & self.operator {
			Op::And => format!("({} & {})", self.left.clone().unwrap().borrow().private_to_infix_inner(), self.right.clone().unwrap().borrow().private_to_infix_inner()),
			Op::Or => format!("({} | {})", self.left.clone().unwrap().borrow().private_to_infix_inner(), self.right.clone().unwrap().borrow().private_to_infix_inner()),
			Op::Not => format!("!{}", self.left.clone().unwrap().borrow().private_to_infix_inner()),
			Op::Lit(val) => format!("{}", val.clone()),
		}
	}

	pub fn to_infix(& self) -> String
	{

		let str = self.private_to_infix_inner();
		if str.len() > 2 && str.as_bytes()[0] == b'(' && str.as_bytes()[str.len() - 1] == b')' {
			return str[1..str.len()-1].to_string();
		}
		str
	}
}

use std::fmt;

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_infix())
    }
}

impl fmt::Display for Node {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.to_infix())
	}
}
