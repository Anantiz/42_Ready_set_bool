use crate::ast::node::*;

impl Node
{
	fn to_rpn(&self) -> String {
		match & self.operator {
			Op::And => format!("{}{}&", self.left.clone().unwrap().borrow().to_rpn(), self.right.clone().unwrap().borrow().to_rpn()),
			Op::Or => format!("{}{}|", self.left.clone().unwrap().borrow().to_rpn(), self.right.clone().unwrap().borrow().to_rpn()),
			Op::Not => format!("{}!", self.left.clone().unwrap().borrow().to_rpn()),
			Op::Lit(val) => format!("{} ", val.clone()),
		}
	}
}

use std::fmt;

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_rpn())
    }
}
