mod set;
type Set = std::collections::HashSet<u32>;


fn main()
{
	let args : Vec<String> = std::env::args().collect();
    if args.len() < 2 {
		println!("Usage: {} <input>", args[0]);
        return;
    }

	// Add program arguments to set
	let mut set = Set::new();
    for arg in args.iter().skip(1) {
		let val = arg.parse::<u32>();
		if val.is_err() {
			println!("Error: Failed to parse argument '{}'", arg);
			return;
		}
		set.insert(val.unwrap());
	}
	let powerset = set::set::set_get_powerset(&set);
	set::set::print_powerset(&powerset);
}
