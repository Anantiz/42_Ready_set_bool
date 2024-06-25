mod eval;

// Pretty explanatory
fn ft_error_exit(msg : &str)
{
	println!("{}", msg);
	panic!();
}

// For dual operations
fn ft_exec_stack(stack : &mut Vec<bool>, op : fn(bool, bool) -> bool)
{
	if stack.len() < 2
	{
		ft_error_exit("Not enough operands in stack");
	}

	// Pop B first because it was pushed last
	let b = stack.pop().unwrap();
	let a = stack.pop().unwrap();

	stack.push(op(a, b));
}

// For unary operations
fn ft_exec_stack_mono(stack : &mut Vec<bool>, op : fn(bool) -> bool)
{
	if stack.len() < 1
	{
		ft_error_exit("Not enough operands in stack");
	}

	let a = stack.pop().unwrap();

	stack.push(op(a));
}

/*
Reverse polish Notation
for handling a boolean expression

// Set: 1 0 ! & | ^ > =
// func: negation, conjuction, disjunction, exclusive_disjunction, material_condition, logical_equivalence
*/
pub fn rpn_evaluate(line : &str) -> bool
{
	let mut stack : Vec<bool> = Vec::new();

	for c in line.chars()
	{
		match c
		{
			' ' => continue,
			'0' => stack.push(false),
			'1' => stack.push(true),
			'!' => ft_exec_stack_mono(&mut stack, eval::negation),
			'&' => ft_exec_stack(&mut stack, eval::conjuction),
			'|' => ft_exec_stack(&mut stack, eval::disjunction),
			'^' => ft_exec_stack(&mut stack, eval::exclusive_disjunction),
			'>' => ft_exec_stack(&mut stack, eval::material_condition),
			'=' => ft_exec_stack(&mut stack, eval::logical_equivalence),
			_ => ft_error_exit("Invalid character in input"),
		}
	}
	return stack.pop().unwrap();
}
