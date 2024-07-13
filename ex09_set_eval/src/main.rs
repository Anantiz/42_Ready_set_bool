mod set;
type Set = std::collections::HashSet<u32>;


fn main()
{
	let mut set = Set::new();
	// set::set::set_insert_vals(&mut set, vec![69, 420, 666, 69420, 30011933]);
	set::set::set_insert_vals(&mut set, vec![1, 2, 3]);

	let powerset = set::set::set_get_powerset(&set);
	let powerset = set::set::sort_power_set_by_size(&powerset);
	set::set::print_powerset(&powerset);
}
