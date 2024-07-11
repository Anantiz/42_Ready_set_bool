pub fn display_first_line(vars : &Vec<(bool, bool)>)
{
	for v in vars.iter().enumerate()
	{
		if v.1.0
		{
			print!("| {} ", (v.0 as u8 + 'A' as u8) as char);
		}
	}
	println!("| = |");
}

pub fn display_truth(vars : &Vec<(bool, bool)>, result : bool)
{
	for v in vars.iter()
	{
		if v.0
		{
			print!("| {} ", if v.1 { "1" } else { "0" });
		}
	}
	println!("| {} |", if result { "1" } else { "0" });
}

