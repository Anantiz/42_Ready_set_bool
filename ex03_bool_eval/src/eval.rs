// ¬ NOT
pub fn negation(a: bool) -> bool
{
	!a
}

// ∧ AND
pub fn conjuction(a : bool, b : bool) -> bool
{
	a & b
}

// ∨ OR
pub fn disjunction(a: bool, b : bool) -> bool
{
	a | b
}

// ⊕ XOR
// 0 0 | 0
// 0 1 | 1
// 1 0 | 1
// 1 1 | 0
pub fn exclusive_disjunction(a: bool, b : bool) -> bool
{
	a ^ b
}

// ⇒ a Implies B (One sided)
// 0 0 | 1
// 0 1 | 1
// 1 0 | 0
// 1 1 | 1
pub fn material_condition(a: bool, b : bool) -> bool
{
	!a & b
}

// ⇔ == (a AND b OR a NOR b)
// 0 0 | 1
// 0 1 | 0
// 1 0 | 0
// 1 1 | 1
pub fn logical_equivalence(a: bool, b : bool) -> bool
{
	(a & b) | (!a & !b)
}
