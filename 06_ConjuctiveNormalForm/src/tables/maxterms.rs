mod rpn;

/*
	Returns an expression where the literals are replaced by their values
*/
fn create_expression(literals : &Vec<(bool, bool)>, og_expression : &str) -> String
{
	let mut expression = String::new();
	for c in og_expression.chars()
	{
		match c
		{
			' ' => continue,
			'A'..='Z' => expression.push(if literals[c as usize - 'A' as usize].1 { '1' } else { '0' }),
			_ => expression.push(c),
		}
	}
	return expression;
}

/*
	For each variables in the expression (First bool in tuple)
	> Set it's state for the given counter (Row of the truth table)
*/
fn set_var_states(mut counter : u32, literals : &mut Vec<(bool, bool)>)
{
	for v in literals.iter_mut()
	{
		if v.0 //If Variable is in the expression
		{
			v.1 = (counter & 0b1) == 0b1; // Set the value of the variable to the last bit of counter
			counter >>= 1;
		}
	}
}

/*
	Will count how much iterations are needed to get all the possible literals configurations
*/
fn max_iter(literals : &Vec<(bool, bool)>) -> u32
{
	// Count them
	let mut var_count = 0;
	for v in literals.iter()
	{
		if v.0
		{
			var_count += 1;
		}
	}
	2u32.pow(var_count as u32)
}

/*
Returns a vector of all variable configurations that result in a maxterm
*/
pub fn get_maxterms(line : &str) -> Vec<Vec<(bool, bool)>>
{
	// Literals range from A to Z
	let mut literals : Vec<(bool, bool)> = vec![(false, false); 26];
	for c in line.chars()
	{
		match c
		{
			' ' => continue,
			'A'..='Z' => literals[c as usize - 'A' as usize].0 = true,
			_ => (),
		}
	}

	let mut maxterms : Vec<Vec<(bool, bool)>> = Vec::new();

	let stop_counter : u32 = max_iter(&literals);
	let mut counter : u32 = 0;

	while counter != stop_counter
	{
		// Will set the boolean values of the literals in the expression
		set_var_states(counter, &mut literals);

		// Rewrite the expression with values instead of literals
		let expression = create_expression(&literals, line);
		let expression_result = rpn::rpn_evaluate(&expression);

		// Get all the maxterms
		if !expression_result
		{
			maxterms.push(literals.clone());
		}
		counter += 1;
	}
	return maxterms;
}