type Set = std::collections::HashSet<u32>;

pub fn set_insert_vals(set: &mut Set, vals: Vec<u32>) {
	for val in vals {
		set.insert(val);
	}
}

pub fn set_get_powerset(set : &Set) -> Vec<Set> {
	let mut powerset = Vec::new();
	let set_vec = set.iter().collect::<Vec<&u32>>();
	let n = set_vec.len();
	let mut i = 0;
	while i < (1 << n) {
		let mut subset = Set::new();
		let mut j = 0;
		while j < n {
			// Works cuz the bin of the index will
			// exclude themselves just like we want the set too
			if i & (1 << j) != 0 {
				subset.insert(*set_vec[j]);
			}
			j += 1;
		}
		powerset.push(subset);
		i += 1;
	}
	powerset
}

pub fn print_set(set: &Set) {
	let mut n = set.len();
	print!("{{");
	for val in set.iter() {
		if n == 1 {
			print!("{}", val);
			break;
		}
		n -= 1;
		print!("{}, ", val);
	}
	println!("}}");
}

pub fn set_get_string(set: &Set) -> String {
	let mut n = set.len();
	let mut string = String::new();
	string.push_str("{");
	for val in set.iter() {
		if n == 1 {
			string.push_str(&val.to_string());
			break;
		}
		n -= 1;
		string.push_str(&format!("{}, ", val));
	}
	string.push_str("}");
	string
}

pub fn sort_power_set_by_size(powerset: &Vec<Set>) -> Vec<Set> {
	let mut powerset = powerset.clone();
	powerset.sort_by(|a, b| a.len().cmp(&b.len()));
	powerset
}

pub fn print_powerset(powerset: &Vec<Set>) {
	println!("Powerset {{");
	for set in powerset.iter() {
		print!("    ");
		print_set(set);
	}
	println!("}}");
}