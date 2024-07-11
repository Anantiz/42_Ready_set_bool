use crate::ast::node::*;
use crate::ast::node::Op;

impl Node
{
	/// TTTF
	/// Prints the tree as only NAND gates
	pub fn to_nand_paradise(& self) -> String {
		match &self.operator {
			Op::Lit(val) => format!("{}", val.clone()),
			Op::Not => {
				format!("NAND({}, {})",
				self.left.clone().unwrap().borrow().to_nand_paradise(),
				self.left.clone().unwrap().borrow().to_nand_paradise())
			},
			Op::And => {
				format!("NAND(NAND({}, {}), NAND({}, {})",
				self.left.clone().unwrap().borrow().to_nand_paradise(),
				self.left.clone().unwrap().borrow().to_nand_paradise(),
				self.right.clone().unwrap().borrow().to_nand_paradise(),
				self.right.clone().unwrap().borrow().to_nand_paradise())
			},
			Op::Or => {
				format!("NAND(NAND(NAND({}, {}), NAND({}, {})), NAND(NAND({}, {}), NAND({}, {}))",
				self.left.clone().unwrap().borrow().to_nand_paradise(),
				self.left.clone().unwrap().borrow().to_nand_paradise(),
				self.right.clone().unwrap().borrow().to_nand_paradise(),
				self.right.clone().unwrap().borrow().to_nand_paradise(),
				self.left.clone().unwrap().borrow().to_nand_paradise(),
				self.left.clone().unwrap().borrow().to_nand_paradise(),
				self.right.clone().unwrap().borrow().to_nand_paradise(),
				self.right.clone().unwrap().borrow().to_nand_paradise())
			}
		}
	}
}
