mod set;
mod ast;
type Set = std::collections::HashSet<u32>;


fn main()
{
	let mut set_a = Set::new();
	set::set::set_insert_vals(&mut set_a, vec![1, 2, 3]);

	let mut set_b = Set::new();
	set::set::set_insert_vals(&mut set_b, vec![2, 3, 4]);

	let sets = vec![set_a, set_b];


	let expr = String::from("AB&");
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

	{
		let mut tree_values_map = tree.unwrap().borrow_mut().set_vals(&sets);
		if tree_values_map.is_err() {
			println!("{}", tree_values_map.err().unwrap());
			return;
		}
		// let (tree_values_map, size) = tree_values_map.unwrap();
		println!("{}", tree.to_rpn());
	}
}
