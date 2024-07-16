/// Ref: https://en.wikipedia.org/wiki/Z-order_curve
/// Ref: https://en.wikipedia.org/wiki/Moser%E2%80%93de_Bruijn_sequence
fn generate_moser_de_bruijn_sequence(n: usize) -> Vec<u32> {
    let mut sequence = Vec::with_capacity(n);

    for i in 0..n {
        let mut value = 0;
        let mut mask = i;
        let mut power_of_4 = 1;

        while mask > 0 {
            if mask & 1 == 1 {
                value += power_of_4;
            }
            power_of_4 *= 4;
            mask >>= 1;
        }

        sequence.push(value);
    }
    sequence
}

fn shift_sequence(seq :&Vec<u32>, n: u32) -> Vec<u32> {
	let mut new_seq = Vec::with_capacity(seq.len());
	for s in seq {
		new_seq.push(s << n);
	}
	new_seq
}

fn get_z_curve_index(x: u32, y: u32) -> u32 {

	let mut x_value = 0;
	let mut mask = x;
	let mut power_of_4 = 1;
	while mask > 0 {
		if mask & 1 == 1 {
			x_value += power_of_4;
		}
		power_of_4 *= 4;
		mask >>= 1;
	}

	let mut y_value = 0;
	mask = y;
	power_of_4 = 1;
	while mask > 0 {
		if mask & 1 == 1 {
			y_value += power_of_4;
		}
		power_of_4 *= 4;
		mask >>= 1;
	}
	return x_value | (y_value << 1);
}

fn get_z_curve_array(size: usize) -> Vec<Vec<u32>> {
	let base_row = generate_moser_de_bruijn_sequence(size);
	let base_column = shift_sequence(&base_row, 1);

	let mut z_curve: Vec<Vec<u32>> = Vec::with_capacity(size);
	for i in 0..size {
		let mut line = Vec::with_capacity(size);
		for j in 0..size {
			line.push(base_row[j] | base_column[i]);
		}
		z_curve.push(line);
	}
	z_curve
}

fn main()
{

	println!("Moser-de-Bruijn sequence in a 2D grid:\n");
	const SQUARE_SIZE: usize = 8;
	let curve = get_z_curve_array(SQUARE_SIZE);
	for i in 0..SQUARE_SIZE {
		print!("\t");
		for j in 0..SQUARE_SIZE {
			print!("{:02} ", curve[i][j]);
		}
		println!();
	}


	/*
	3:4 -> 37
	4:4 -> 48
	3:5 -> 39
	*/
	println!("\nDirect Z-curve index calculations:\n");
	println!("\tIndex of (3,4) is {}", get_z_curve_index(3, 4));
	println!("\tIndex of (4,4) is {}", get_z_curve_index(4, 4));
	println!("\tIndex of (3,5) is {}", get_z_curve_index(3, 5));
	println!("\tIndex of (2640,1080) is {}", get_z_curve_index(2640, 1080));
	println!("\n");
}