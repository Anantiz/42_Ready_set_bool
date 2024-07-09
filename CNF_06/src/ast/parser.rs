use crate::ast::node::*;
mod constructors as Node;

use std::rc::Rc;
use std::cell::RefCell;


impl Node
{
	pub fn rpn_to_ast(str: &str) -> Result<Option<Rc<RefCell<Node>>>, String> {
        let mut stack: Vec<Rc<RefCell<Node>>> = Vec::new();

        for c in str.chars() {
            match c {
                'A'..='Z' => stack.push(Node::new_lit(c.to_string())),
                '!' => {
                    let child = stack.pop().ok_or_else(|| "Expected target for NOT".to_string())?;
                    stack.push(Node::new_not(Some(child)));
                },
                '&' => {
                    let right = stack.pop().ok_or_else(|| "Expected right-hand operator for &".to_string())?;
                    let left = stack.pop().ok_or_else(|| "Expected left-hand operator for &".to_string())?;
                    stack.push(Node::new_and(Some(left), Some(right)));
                },
                '|' => {
                    let right = stack.pop().ok_or_else(|| "Expected right-hand operator for |".to_string())?;
                    let left = stack.pop().ok_or_else(|| "Expected left-hand operator for |".to_string())?;
                    stack.push(Node::new_or(Some(left), Some(right)));
                },
                '=' => {
                    let right = stack.pop().ok_or_else(|| "Expected right-hand operator for =".to_string())?;
                    let left = stack.pop().ok_or_else(|| "Expected left-hand operator for =".to_string())?;
                    stack.push(Node::new_or(
                        Some(Node::new_and(Some(left.clone()), Some(right.clone()))),
                        Some(Node::new_and(
                            Node::negate_box(Some(left)),
                            Node::negate_box(Some(right)),
                        )),
                    ));
                },
                '^' => {
                    let right = stack.pop().ok_or_else(|| "Expected right-hand operator for ^".to_string())?;
                    let left = stack.pop().ok_or_else(|| "Expected left-hand operator for ^".to_string())?;
                    stack.push(Node::new_or(
                        Some(Node::new_and(Some(left.clone()), Node::negate_box(Some(right.clone())))),
                        Some(Node::new_and(Node::negate_box(Some(left)), Some(right))),
                    ));
                },
                '>' => {
                    let right = stack.pop().ok_or_else(|| "Expected right-hand operator for >".to_string())?;
                    let left = stack.pop().ok_or_else(|| "Expected left-hand operator for >".to_string())?;
                    stack.push(Node::new_or(Node::negate_box(Some(left)), Some(right)));
                },
                ' ' => continue,
                _ => return Err(format!("Invalid character: {}", c)),
            }
        }
        Ok(stack.pop())
    }
}