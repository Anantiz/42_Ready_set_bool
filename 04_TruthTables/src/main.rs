mod rpn;
mod display;

fn update_var_states(mut counter : u32, vars : &mut Vec<(bool, bool)>)
{
	for v in vars.iter_mut()
	{
		if v.0 //If Variable is in the expression
		{
			v.1 = (counter & 0b1) == 0b1; // Set the value of the variable to the last bit of counter
			counter >>= 1;
		}
	}
}


fn iter_count(vars : &Vec<(bool, bool)>) -> u32
{
	// Count them
	let mut var_count = 0;
	for v in vars.iter()
	{
		if v.0
		{
			var_count += 1;
		}
	}
	2u32.pow(var_count as u32)
}

fn write_expression(vars : &Vec<(bool, bool)>, line : &str) -> String
{
	let mut expression = String::new();
	for c in line.chars()
	{
		match c
		{
			' ' => continue,
			'A'..='Z' => expression.push(if vars[c as usize - 'A' as usize].1 { '1' } else { '0' }),
			_ => expression.push(c),
		}
	}
	return expression;
}

// Write a truth table for the input expression
// In e.i: A B &
fn print_truth_table(line : &str)
{
	println!("Truth Table for: {}", line);

	// Variables can go from A to Z, 26 variables
	// Make a 26 long vector of bools and set them if they are in the expression
	// First part of tuple is whether the variable is in the expression
	// Second part of tuple is the value of the variable at a give itteration
	let mut vars : Vec<(bool, bool)> = vec![(false, false); 26];

	for c in line.chars()
	{
		match c
		{
			' ' => continue,
			'A'..='Z' => vars[c as usize - 'A' as usize].0 = true,
			_ => (),
		}
	}
	display::display_first_line(&vars);

	let iter_count : u32 = iter_count(&vars);
	let mut counter : u32 = 0;

	// Now loop while the var_state & mask is not zero
	while counter != iter_count
	{
		// println!("Counter: {}\tVar State: {:8b}", counter, var_state);
		// Set vars states
		update_var_states(counter, &mut vars);

		// Dynamically rewrite the expression with expanded variables
		let expression = write_expression(&vars, line);

		// Finally Evaluate and display the line
		display::display_truth(&vars, rpn::rpn_evaluate(&expression));

		// Get next state
		counter += 1;
	}
}


fn main()
{
	let line = "ABCD&|&";
	print_truth_table(&line);

}

