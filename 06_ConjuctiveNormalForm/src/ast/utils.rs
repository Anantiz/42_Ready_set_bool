use crate::ast::ast::AstNode;
use crate::ast::ast::Expr;
use std::rc::Rc;
use std::cell::RefCell;

impl AstNode {
    pub fn negate(node: Rc<RefCell<AstNode>>) -> Option<Rc<RefCell<AstNode>>> {
        match node.borrow().data {
            Expr::Lit(c) => Some(AstNode::new_not(Some(AstNode::new_literal(c)))),
            Expr::Not() => node.borrow().left.clone(),
            Expr::And() => Some(AstNode::new_or(
                AstNode::negate_box(node.borrow().left.clone()),
                AstNode::negate_box(node.borrow().right.clone()),
            )),
            Expr::Or() => Some(AstNode::new_and(
                AstNode::negate_box(node.borrow().left.clone()),
                AstNode::negate_box(node.borrow().right.clone()),
            )),
        }
    }

    pub fn negate_box(node: Option<Rc<RefCell<AstNode>>>) -> Option<Rc<RefCell<AstNode>>> {
        match node {
            None => None,
            Some(node) => AstNode::negate(node),
        }
    }

    pub fn rpn_to_ast(str: &str) -> Result<Option<Rc<RefCell<AstNode>>>, String> {
        let mut stack: Vec<Rc<RefCell<AstNode>>> = Vec::new();

        for c in str.chars() {
            match c {
                'A'..='Z' => stack.push(AstNode::new_literal(c)),
                '!' => {
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
                '=' => {
                    let right = stack.pop().ok_or_else(|| "Expected right-hand operator for =".to_string())?;
                    let left = stack.pop().ok_or_else(|| "Expected left-hand operator for =".to_string())?;
                    stack.push(AstNode::new_or(
                        Some(AstNode::new_and(Some(left.clone()), Some(right.clone()))),
                        Some(AstNode::new_and(
                            AstNode::negate_box(Some(left)),
                            AstNode::negate_box(Some(right)),
                        )),
                    ));
                },
                '^' => {
                    let right = stack.pop().ok_or_else(|| "Expected right-hand operator for ^".to_string())?;
                    let left = stack.pop().ok_or_else(|| "Expected left-hand operator for ^".to_string())?;
                    stack.push(AstNode::new_or(
                        Some(AstNode::new_and(Some(left.clone()), AstNode::negate_box(Some(right.clone())))),
                        Some(AstNode::new_and(AstNode::negate_box(Some(left)), Some(right))),
                    ));
                },
                '>' => {
                    let right = stack.pop().ok_or_else(|| "Expected right-hand operator for >".to_string())?;
                    let left = stack.pop().ok_or_else(|| "Expected left-hand operator for >".to_string())?;
                    stack.push(AstNode::new_or(AstNode::negate_box(Some(left)), Some(right)));
                },
                ' ' => continue,
                _ => return Err(format!("Invalid character: {}", c)),
            }
        }
        Ok(stack.pop())
    }

    pub fn to_rpn(&self) -> String {
        match self.data {
            Expr::Lit(c) => c.to_string(),
            Expr::Not() => format!("{} !", self.left.as_ref().unwrap().borrow().to_rpn()),
            Expr::And() => format!("{} {} &", self.left.as_ref().unwrap().borrow().to_rpn(), self.right.as_ref().unwrap().borrow().to_rpn()),
            Expr::Or() => format!("{} {} |", self.left.as_ref().unwrap().borrow().to_rpn(), self.right.as_ref().unwrap().borrow().to_rpn()),
        }
    }
}

use std::fmt;

impl fmt::Debug for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_rpn())
    }
}
