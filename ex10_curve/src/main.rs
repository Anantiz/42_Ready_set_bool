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

/// Actually the assigment asks for something much more trivial
fn map(x: u16, y: u16) -> f64 {
	const MAX: f64 = (2u64.pow(32) - 1) as f64;
	get_moser_de_brujin_index(x, y) as f64 / MAX
}

fn main()
{
	let test_cases :  [(u16, u16); 5] = [
        (0, 0),
        (32767, 32767),
        (65535, 32767),
        (32768, 32768),
        (65535, 65535),
    ];

    for &(x, y) in &test_cases {
        println!("Mapping ({:05}, {:05}) to {:0.10}", x, y, map(x, y));
    }
}