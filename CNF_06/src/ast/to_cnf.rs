use crate::ast::node::*;
use crate::ast::node::Op;

use std::rc::Rc;
use std::cell::RefCell;

impl Node {

	/// Will create a new tree in CNF form
	/// Algorithm Reference: https://profs.info.uaic.ro/stefan.ciobaca/logic-2018-2019/notes7.pdf
	///   > Detailed explanation with proofs and examples (that even uses an AST awesome!)
	pub fn to_cnf(& self) -> Option<Rc<RefCell<Node>>>
	{
		let nnf = self.to_nnf();
		if nnf.is_none() {
			return None;
		}
		nnf

		// TO DO:
		// Use tseytin transformation to convert the tree to CNF
	}

}