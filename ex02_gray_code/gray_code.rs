fn gray_code(n : u32) -> u32
{
	n ^ (n >> 1)
}

fn main()
{
	let mut grays : [u32; 16] = [0; 16];

	for n in 0..16
	{
		grays[n] = gray_code(n as u32);
	}
	println!("Dec | Binary | Gray");
	for n in 0..16
	{
		println!("{:3} | {:05b} | {:05b}", n, n, grays[n]);
	}
}