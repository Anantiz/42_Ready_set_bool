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
		println!("NNF form: {}", nnf.clone().unwrap().borrow().to_rpn());

		let mut cnf_root : Option<Rc<RefCell<Node>>> = None;
		let mut tail : Option<Rc<RefCell<Node>>> = None;
		let mut iterator = nnf.unwrap().borrow().new_iterator();

		loop {
			let it = iterator.next_node();
			if it.is_none() {
				println!("End of cnf reached");
				break;
			}
			let node = it.unwrap().borrow().clone();
			let tseytin_node = match & node.operator {
				Op::And => {
					node.tseytin_transform_and()
				},
				Op::Or => {
					node.tseytin_transform_or()
				},
				Op::Not => {
					node.tseytin_transform_not()
				},
				Op::Lit(_) => {
					node.duplicate()
				},
			};
			if cnf_root.is_none() {
				cnf_root = Some(tseytin_node.as_rc());
				tail = Some(tseytin_node.as_rc());
			} else{
				tail = Some(Node::merge_as_conjuction(&mut tail.unwrap(), tseytin_node));
			}
		}
		cnf_root
	}

	fn tseytin_transform_not(& self) -> Node {
		println!("Transforming NOT node: {}", self.name);
		/*
		*	When the node is a Negation:
		*		As (NOT [left]-> ?) where
		*			> The not node is a
		*			> the left child is b
		*	We can transform it to:
		*		(¬a ∨ ¬b) ∧ (b ∨ a)
		*	Since we introduce new literals, we will use
		*	the node names for their Lit(String) values
		*/
		let a = Node::new_lit(self.name.clone());
		let b = Node::new_lit(self.left.clone().unwrap().borrow().name.clone());
		let not_a = Node::new_not(Some(a.duplicate().to_rc()), a.name.clone());
		let not_b = Node::new_not(Some(b.duplicate().to_rc()), b.name.clone());

		let clause_1 = Node::new_or(Some(not_a.duplicate().to_rc()), Some(not_b.duplicate().to_rc()), Node::increment_name());
		let clause_2 = Node::new_or(Some(b.duplicate().to_rc()), Some(a.duplicate().to_rc()), Node::increment_name());
		Node::assign_parents_to_children(&clause_1);
		Node::assign_parents_to_children(&clause_2);
		let conjucted = Node::new_and(Some(clause_1.to_rc()), Some(clause_2.to_rc()), Node::increment_name()
		);
		Node::assign_parents_to_children(&conjucted);
		conjucted
	}

	/// Create a new tree without affecting the original tree
	fn tseytin_transform_and(& self) -> Node {
		println!("Transforming AND node: {}", self.name);
		/*
		*	When the node is a Conjuction:
		*		As (? ∧ ?) where
		*			> The and node is a
		*			> the left child is b
		*			> the right child is c
		*	We can transform it to:
		*		(¬a ∨ b) ∧ (¬a ∨ c) ∧ (¬b ∨ ¬c ∨ a)
		*/
		let a = Node::new_lit(self.name.clone());
		let b = Node::new_lit(self.left.clone().unwrap().borrow().name.clone());
		let c = Node::new_lit(self.right.clone().unwrap().borrow().name.clone());
		let not_a = Node::new_not(Some(a.duplicate().to_rc()), a.name.clone());
		let not_b = Node::new_not(Some(b.duplicate().to_rc()), b.name.clone());
		let not_c = Node::new_not(Some(c.duplicate().to_rc()), c.name.clone());

		let clause_1 = Node::new_or(Some(not_a.duplicate().to_rc()), Some(b.duplicate().to_rc()), Node::increment_name());
		let clause_2 = Node::new_or(Some(not_a.duplicate().to_rc()), Some(c.duplicate().to_rc()), Node::increment_name());
		let clause_3 = Node::new_or(
			Some(not_b.duplicate().to_rc()),
			Some(Node::new_or(Some(not_c.duplicate().to_rc()), Some(a.duplicate().to_rc()), Node::increment_name()).to_rc()),
			Node::increment_name()
		);
		Node::assign_parents_to_children(&clause_1);
		Node::assign_parents_to_children(&clause_2);
		Node::assign_parents_to_children(&clause_3);

		let conjucted = Node::new_and(
			Some(clause_1.to_rc()),
			Some(Node::new_and(Some(clause_2.to_rc()), Some(clause_3.to_rc()), Node::increment_name()).to_rc()),
			Node::increment_name()
		);
		Node::assign_parents_to_children(&conjucted);
		conjucted
	}

	fn tseytin_transform_or(& self) -> Node {
		println!("Transforming OR node: {}", self.name);
		/*
		*	When the node is a Disjunction:
		*		As (? ∨ ?) where
		*			> The or node is a
		*			> the left child is b
		*			> the right child is c
		*	We can transform it to:
		*		(¬a ∨ b ∨ c) ∧ (¬b ∨ a) ∧ (¬c ∨ a)
		*/
		let a = Node::new_lit(self.name.clone());
		let b = Node::new_lit(self.left.clone().unwrap().borrow().name.clone());
		let c = Node::new_lit(self.right.clone().unwrap().borrow().name.clone());
		let not_a = Node::new_not(Some(a.duplicate().to_rc()), a.name.clone());
		let not_b = Node::new_not(Some(b.duplicate().to_rc()), b.name.clone());
		let not_c = Node::new_not(Some(c.duplicate().to_rc()), c.name.clone());

		let clause_1 = Node::new_or(
			Some(not_a.duplicate().to_rc()),
			Some(Node::new_or(Some(b.duplicate().to_rc()), Some(c.duplicate().to_rc()), Node::increment_name()).to_rc()),
			Node::increment_name()
		);
		let clause_2 = Node::new_or(Some(not_b.duplicate().to_rc()), Some(a.duplicate().to_rc()), Node::increment_name());
		let clause_3 = Node::new_or(Some(not_c.duplicate().to_rc()), Some(a.duplicate().to_rc()), Node::increment_name());

		Node::assign_parents_to_children(&clause_1);
		Node::assign_parents_to_children(&clause_2);
		Node::assign_parents_to_children(&clause_3);

		let conjucted = Node::new_and(
			Some(clause_1.to_rc()),
			Some(Node::new_and(Some(clause_2.to_rc()), Some(clause_3.to_rc()), Node::increment_name()).to_rc()),
			Node::increment_name()
		);
		Node::assign_parents_to_children(&conjucted);
		conjucted
	}

	/// Used to name fresh literals
	fn increment_name() -> String {
		static mut COUNT : u32 = 0;
		let mut name = String::from("t_");
		unsafe {
			name.push_str(&COUNT.to_string());
			COUNT += 1;
		}
		name
	}

}