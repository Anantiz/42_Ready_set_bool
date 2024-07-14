mod set;
mod ast;

use set::*;
use std::rc::Rc;
use std::cell::RefCell;

fn main()
{
	let expr = String::from("AB^C|");


	let mut set_a = Set::new();
	set::set::set_insert_vals(&mut set_a, vec![1, 2, 3, 69]);
	let mut set_b = Set::new();
	set::set::set_insert_vals(&mut set_b, vec![69, 420]);
	let mut set_c = Set::new();
	set::set::set_insert_vals(&mut set_c, vec![666, 777, 888, 999]);
	let sets = vec![Rc::new(RefCell::new(set_a)), Rc::new(RefCell::new(set_b)), Rc::new(RefCell::new(set_c))];



	let tree = ast::node::Node::parse(&expr);
	if tree.is_err() {
		println!("{}", tree.err().unwrap());
		return;
	}
	let tree = tree.unwrap();
	if tree.is_none() {
		println!("Error: Could not parse expression");
		return;
	}
	println!("Evaluating:");
	set::eval::set_evaluate(tree.unwrap(), &sets);
}