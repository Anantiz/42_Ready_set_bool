#[derive(Debug, Clone)]
pub enum Expr {
    Var(char),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

impl Expr {
    // Distribute negations using De Morgan's laws

	pub fn negation_normal_form(self) -> Expr {
		match self {
			Expr::Not(inner) => match *inner {
				Expr::And(left, right) => Expr::Or(
					Box::new(Expr::Not(left).negation_normal_form()),
					Box::new(Expr::Not(right).negation_normal_form()),
				),
				Expr::Or(left, right) => Expr::And(
					Box::new(Expr::Not(left).negation_normal_form()),
					Box::new(Expr::Not(right).negation_normal_form()),
				),
				other => Expr::Not(Box::new(other.negation_normal_form())),
			},
			Expr::And(left, right) => Expr::And(
				Box::new(left.negation_normal_form()),
				Box::new(right.negation_normal_form()),
			),
			Expr::Or(left, right) => Expr::Or(
				Box::new(left.negation_normal_form()),
				Box::new(right.negation_normal_form()),
			),
			other => other,
		}
	}

    // Convert the AST back into polish notation
    pub fn to_polish_notation(&self) -> String {
        match self {
            Expr::Var(c) => c.to_string(),
            Expr::Not(expr) => format!("{}!", expr.to_polish_notation()),
            Expr::And(left, right) => format!(
                "{}{}&",
                left.to_polish_notation(),
                right.to_polish_notation()
            ),
            Expr::Or(left, right) => format!(
                "{}{}|",
                left.to_polish_notation(),
                right.to_polish_notation()
            ),
        }
    }
}

/*
Input example: AB&CD&|
Exprected tree:
     |
   /   \
  &     &
 / \   / \
A   B C   D
*/
pub fn parse_polish_expression(input: &str) -> Expr
{
    let mut stack: Vec<Expr> = Vec::new();

    for ch in input.chars() {
        match ch {
            'A'..='Z' => stack.push(Expr::Var(ch)), // Push variables onto the stack
            '!' => {
                let expr = stack.pop().expect("Expected an operand for '!'"); // Pop one operand for NOT
                stack.push(Expr::Not(Box::new(expr)));
            }
            '&' => {
                let right = stack.pop().expect("Expected a right operand for '&'"); // Pop two operands for AND
                let left = stack.pop().expect("Expected a left operand for '&'");
                stack.push(Expr::And(Box::new(left), Box::new(right)));
            }
            '|' => {
                let right = stack.pop().expect("Expected a right operand for '|'"); // Pop two operands for OR
                let left = stack.pop().expect("Expected a left operand for '|'");
                stack.push(Expr::Or(Box::new(left), Box::new(right)));
            }
            _ => panic!("Invalid character in input"), // Panic on invalid input
        }
    }

    // There should be exactly one element on the stack, which is the root of the AST
    stack.pop().expect("Invalid expression, stack should contain exactly one element")
}