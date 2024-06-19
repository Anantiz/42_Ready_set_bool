#[inline(always)]
fn add(a: u32, b: u32) -> u32
{

	// Multi-Steps:
	// 	1.Shift the mask to the left if we have a carry
	// 	 let mut pow_mask : u32 = a & b;
	// 	 pow_mask = pow_mask << 1;
	// 	2.Add the remaining non-carry digits
	// 	 let digit_mask : u32 = a ^ b;
	//  3.Combine the two masks
		//  return pow_mask | digit_mask;
	// One liner
	 return ((a & b) << 1) | (a ^ b);
}

fn main()
{
	let a : u32 = 10;
	let b: u32 = 50;

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