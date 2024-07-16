fn get_moser_de_brujin_index(x: u16, y: u16) -> u64 {

	let mut x_value: u64 = 0;
	let mut power_of_4: u64 = 1;
	let mut mask = x;
	while mask > 0 {
		if mask & 1 == 1 {
			x_value += power_of_4;
		}
		power_of_4 *= 4;
		mask >>= 1;
	}

	let mut y_value: u64 = 0;
	power_of_4 = 1;
	mask = y;
	while mask > 0 {
		if mask & 1 == 1 {
			y_value += power_of_4;
		}
		power_of_4 *= 4;
		mask >>= 1;
	}
	return x_value + (y_value << 1);
}

fn print_table_moser_brujin_binary() {
	for y in 0..8 {
		for x in 0..8 {
			let index = get_moser_de_brujin_index(x, y);
			print!("{:06b}  ", index);
		}
		println!();
	}
	println!();

	for y in 0..8 {
		for x in 0..8 {
			let index = get_moser_de_brujin_index(x, y);
			print!("{:02} ", index);
		}
		println!();
	}
	println!();
}

/// Actually the assigment asks for something much more trivial
fn map(x: u16, y: u16) -> f64 {
	const MAX: f64 = (2u64.pow(32) - 1) as f64;
	get_moser_de_brujin_index(x, y) as f64 / MAX
}

fn dumbfuck_approach(n: u64) -> u16 {
	fn get_moser_de_brujin_sequence_inner(x: u16) -> u64 {
		let mut x_value: u64 = 0;
		let mut power_of_4: u64 = 1;
		let mut mask = x;
		while mask > 0 {
			if mask & 1 == 1 {
				x_value += power_of_4;
			}
			power_of_4 *= 4;
			mask >>= 1;
		}
		x_value
	}
	let mut rank: u16 = 0;
	loop {
		let val = get_moser_de_brujin_sequence_inner(rank);
		if val == n{
			return rank;
		}
		rank += 1;
	}
}

fn reverse_map(n: f64) -> (u16, u16) {
	const MAX: f64 = (2u64.pow(32) - 1) as f64;

	let mask_x: u64 = 0b10101010101010101010101010101010101;
	let mask_y: u64 = mask_x << 1;
	let n: u64 = (n * MAX) as u64;

	// Meaning :
	//	The value of the nth rank in the moser_de_brujin sequence
	//	such that sequence[n] = x, ditto for y shifted by 1
	let x_sequence_value: u64 = n & mask_x;
	let y_sequence_value: u64 = n & mask_y;

	// Now we need to efficiently compute the rank based on the value
	// Logarithm won't work because the sequence has additions of terms
	// which result in non-powers-of-4
	// PAIN
	let x = dumbfuck_approach(x_sequence_value);
	let y = dumbfuck_approach(y_sequence_value >> 1);
	(x , y)
}

fn main()
{
	let test_cases :  [(u16, u16); 6] = [
        (0, 0),
        (2, 1),
        (5, 5),
        (1, 6),
        (510, 512),
        (65535, 65535),
    ];
	print_table_moser_brujin_binary();

    for &(x, y) in &test_cases {
        let map = map(x, y);
		let (rx, ry) = reverse_map(map);
		println!("OG: [{:05}, {:05}]\tMapped: {:0.8}\tRevert: [{:05}, {:05}] ", x, y, map, rx, ry);
    }
}

/*
fn reverse_map(n: f64) -> (u16, u16) {
    const MAX: f64 = (2u64.pow(32) - 1) as f64;

    let mask_x: u64 = 0b01010101010101010101010101010101;
    let mask_y: u64 = mask_x << 1;

    let n: u64 = (n * MAX) as u64;

    fn extract_value(n: u64, mask: u64) -> u16 {
        let mut value = 0;
        let mut bit_position = 0;
        let mut shift = 0;
        while mask >> bit_position > 0 {
            if n & (1 << bit_position) != 0 {
                value |= 1 << shift;
            }
            bit_position += 2;
            shift += 1;
        }
        value
    }

    let x = extract_value(n, mask_x);
    let y = extract_value(n, mask_y >> 1);

    (x, y)
}


*/