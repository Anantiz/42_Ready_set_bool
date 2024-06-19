#[inline(always)]
fn add(a: u32, b: u32) -> u32
{
	let mut pow_mask : u32 = a & b;
	pow_mask = pow_mask << 1;
	let digit_mask : u32 = a ^ b;
	if (pow_mask & digit_mask) != 0
	{
		return add(pow_mask, digit_mask);
	}
	else
	{
		return pow_mask | digit_mask;
	}
}

fn mult(mut a : u32, mut b : u32) -> u32
{
	// Swap if a < b to have the smallest loop
	if a < b
	{
		a = a ^ b;
		b = a ^ b;
		a = a ^ b;
	}

	let mut i : u32 = 0;
	let mut ret : u32 = 0;
	while i < b
	{
		ret = add(ret, a);
		i = add(i, 1);
	}
	return ret;
}

fn main()
{
	let a : u32 = 10;
	let b: u32 = 50;
	let ret = mult(a, b);
	println!("{} * {} = {}", a, b, ret);

	let a : u32 = 5;
	let b: u32 = 3;
	let ret = mult(a, b);
	println!("{} * {} = {}", a, b, ret);

	let a : u32 = 10;
	let b: u32 = 1;
	let ret = mult(a, b);
	println!("{} * {} = {}", a, b, ret);

	let a : u32 = 2;
	let b: u32 = 0;
	let ret = mult(a, b);
	println!("{} * {} = {}", a, b, ret);

	let a : u32 = 0;
	let b: u32 = 0;
	let ret = mult(a, b);
	println!("{} * {} = {}", a, b, ret);
}