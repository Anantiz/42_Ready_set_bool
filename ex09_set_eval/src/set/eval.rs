use crate::ast::node::Node as Node;
use crate::ast::node::Op as Op;

use super::set::Set as Set;
use super::set::set_get_string as set_get_string;
use std::rc::Rc;
use std::cell::RefCell;


fn set_and(set_a: &Set, set_b: &Set) -> Set
{
	let mut set = Set::new();
	for val in set_a.iter() {
		if set_b.contains(val) {
			set.insert(*val);
		}
	}
	set
}

fn set_or(set_a: &Set, set_b: &Set) -> Set
{
	let mut set = Set::new();
	for val in set_a.iter() {
		set.insert(*val);
	}
	for val in set_b.iter() {
		set.insert(*val);
	}
	set
}

/// Whatever that means
fn set_not(set_a: &Set, universe: &Set) -> Set
{
	let mut set = Set::new();
	for val in universe.iter() {
		if !set_a.contains(val) {
			set.insert(*val);
		}
	}
	set
}

fn set_evaluate_rec(tree: &Rc<RefCell<Node>>, universe: &Set) -> Set
{
	let node = tree.borrow();
	match node.operator {
		Op::Lit(_) => {
			let value = node.value.clone();
			if value.is_none() {
				return Set::new();
			}
			return value.unwrap().borrow().clone();
		},
		Op::And => {
			let left = set_evaluate_rec(&node.left.clone().unwrap(), universe);
			let right = set_evaluate_rec(&node.right.clone().unwrap(), universe);
			return set_and(&left, &right);
		},
		Op::Or => {
			let left = set_evaluate_rec(&node.left.clone().unwrap(), universe);
			let right = set_evaluate_rec(&node.right.clone().unwrap(), universe);
			return set_or(&left, &right);
		},
		Op::Not => {
			let left = set_evaluate_rec(&node.left.clone().unwrap(), universe);
			return set_not(&left, universe);
		},
	}
}

fn init_universe(sets: &Vec<Rc<RefCell<Set>>>) -> Set
{
	let mut universe = Set::new();
	for set in sets.iter() {
		for val in set.borrow().iter() {
			universe.insert(*val);
		}
	}
	universe
}

pub fn set_evaluate(tree: Rc<RefCell<Node>>, sets: &Vec<Rc<RefCell<Set>>>) -> Set
{
	// It will be simpler to implement the set evaluation
	// if we convert the tree to NNF form (No need to handle negated operators)
	let nnf = tree.borrow().to_nnf();
	let tree = nnf.unwrap();
	let tree_values_map = tree.borrow_mut().set_vals(&sets);
	if tree_values_map.is_err() {
		println!("{}", tree_values_map.err().unwrap());
		return Set::new();
	}
	// let (tree_values_map, size) = tree_values_map.unwrap();
	println!("{}", tree.borrow().to_rpn());

	// At this point with have an AST for expression and the tree_values_map
	// All lits in the tree are linked to their respective sets
	let universe : Set = init_universe(&sets);
	let result = set_evaluate_rec(&tree, &universe);
	println!("Result: {}", set_get_string(&result));
	result
}