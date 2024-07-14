mod set;
type Set = std::collections::HashSet<u32>;


fn main()
{
	let args : Vec<String> = std::env::args().collect();
    if args.len() < 2 {
		println!("Usage: {} <input>", args[0]);
        return;
    }
	let mut set = Set::new();
    for arg in args.iter().skip(1) {
		let val = arg.parse::<u32>();
		if val.is_err() {
			println!("Error: Failed to parse argument '{}'", arg);
			return;
		}
		set.insert(val.unwrap());
	}

	// set::set::set_insert_vals(&mut set, vec![1, 2, 3]);
	// set::set::set_insert_vals(&mut set, vec![69, 420, 666, 69420, 30011933]);

	let powerset = set::set::set_get_powerset(&set);
	// let powerset = set::set::sort_power_set_by_size(&powerset);
	set::set::print_powerset(&powerset);
}
