#[inline(always)]
fn add(a: u32, b: u32) -> u32
{

	// Multi-Steps:
	// 	1.Shift the mask to the left if we have a carry
	let mut pow_mask : u32 = a & b;
	pow_mask = pow_mask << 1;
	// 	2.Add the remaining non-carry digits
	let digit_mask : u32 = a ^ b;
	//  3.Combine the two masks and re-carry if needed
	if (pow_mask & digit_mask) != 0
	{
		return add(pow_mask, digit_mask);
	}
	else
	{
		return pow_mask | digit_mask;
	}
}

fn main()
{
	let a : u32 = 3;
	let b: u32 = 1;

	let ret = add(a, b);
	println!("{} + {} = {}", a, b, ret);

	let a : u32 = 2;
	let b: u32 = 40;

	let ret = add(a, b);
	println!("{} + {} = {}", a, b, ret);

	let a : u32 = 0;
	let b: u32 = 1;

	let ret = add(a, b);
	println!("{} + {} = {}", a, b, ret);

	let a : u32 = 0;
	let b: u32 = 0;

	let ret = add(a, b);
	println!("{} + {} = {}", a, b, ret);
}