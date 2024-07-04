use crate::ast::ast::AstNode;
use crate::ast::ast::Expr;

impl AstNode
{
	/// Perform a match over the given Expr
    /// Return
    ///  Case:
    ///     > Not: left
    ///     > Lit: Not(left: lit, right: None)
    ///     > Or: And(left: negate(left), right : negate(right))
    ///     > And: Or(left: negate(left), right : negate(right))
    /// That's actually de Morgan's Law I guess
    pub fn negate(node : AstNode) -> Option<Box<AstNode>>
    {
        match node.data
        {
            Expr::Lit(c) => Some(AstNode::new_not(Some(AstNode::new_literal(c)))),
            Expr::Not() => node.left,
            Expr::And() => Some(AstNode::new_or(
                AstNode::negate_box(node.left),
                AstNode::negate_box(node.right))
                ),
            Expr::Or() => Some(AstNode::new_and(
                AstNode::negate_box(node.left),
                AstNode::negate_box(node.right))
            )
        }
    }

    pub fn negate_box(node : Option<Box<AstNode>>) -> Option<Box<AstNode>>
    {
        match node
        {
            None => None,
            Some(node) => AstNode::negate(*node)
        }
    }

    /*
        ** Returns None if the input string is invalid
    */
    pub fn rpn_to_ast(str : &str) -> Result<Option<Box<AstNode>>,String>
    {
        let mut stack : Vec<Box<AstNode>> = Vec::new();

        for c in str.chars()
        {
            match c
            {
                'A'..='Z' => stack.push(AstNode::new_literal(c)),
                '!' =>  {
                    let child = stack.pop().ok_or_else(|| "Expected target for NOT".to_string())?;
                    stack.push(AstNode::new_not(Some(child)));
                },
                '&' => {
                    let right = stack.pop().ok_or_else(|| "Expected right-hand operator for &".to_string())?;
                    let left = stack.pop().ok_or_else(|| "Expected left-hand operator for &".to_string())?;
                    stack.push(AstNode::new_and(Some(left), Some(right)));
                },
                '|' => {
                    let right = stack.pop().ok_or_else(|| "Expected right-hand operator for |".to_string())?;
                    let left = stack.pop().ok_or_else(|| "Expected left-hand operator for |".to_string())?;
                    stack.push(AstNode::new_or(Some(left), Some(right)));
                },
                '=' => { // Implication
                    let right = stack.pop().ok_or_else(|| "Expected right-hand operator for =".to_string())?; // Pop two operands for OR
                    let left = stack.pop().ok_or_else(|| "Expected left-hand operator for =".to_string())?;
                    stack.push(AstNode::new_or(
                            Some(AstNode::new_and(Some(left.clone()), Some(right.clone()))),
                            Some(AstNode::new_and(AstNode::negate_box(Some(left)), AstNode::negate_box(Some(right))))
                        ))
                },
                '^' => { // XOR
                    let right = stack.pop().ok_or_else(|| "Expected right-hand operator for ^".to_string())?; // Pop two operands for OR
                    let left = stack.pop().ok_or_else(|| "Expected left-hand operator for ^".to_string())?;
                    stack.push(AstNode::new_or(
                            Some(AstNode::new_and(Some(left.clone()), AstNode::negate_box(Some(right.clone())))),
                            Some(AstNode::new_and(AstNode::negate_box(Some(left)), Some(right)))
                        ))
                },
                '>' => { // Material conditional, only false for [0 1 => False]
                    let right = stack.pop().ok_or_else(|| "Expected right-hand operator for >".to_string())?; // Pop two operands for OR
                    let left = stack.pop().ok_or_else(|| "Expected left-hand operator for >".to_string())?;
                    stack.push(AstNode::new_or(AstNode::negate_box(Some(left)), Some(right)));
                },
                ' ' => continue,
                _ => return Err(format!("Invalid character: {}", c))
            }
        }
        return Ok(stack.pop());
    }

    pub fn to_rpn(&self) -> String
    {
        match self.data
        {
            Expr::Lit(c) => c.to_string(),
            Expr::Not() => format!("{}!", self.left.as_ref().unwrap().to_rpn()),
            Expr::And() => format!("{}{}&", self.left.as_ref().unwrap().to_rpn(), self.right.as_ref().unwrap().to_rpn()),
            Expr::Or() => format!("{}{}|", self.left.as_ref().unwrap().to_rpn(), self.right.as_ref().unwrap().to_rpn())
        }
    }
}

use std::fmt;

impl fmt::Debug for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_rpn())
    }
}