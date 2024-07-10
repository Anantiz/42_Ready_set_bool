use crate::ast::node::*;

use std::rc::Rc;
use std::cell::RefCell;

impl Node
{
	pub fn parse(str: &str) -> Result<Option<Rc<RefCell<Node>>>, String> {

        // Prevent stack overflow, '100' is very conservative, but this is a toy project
        // we are not there to handle boolean expressions with millions of operations and variables
        const MAX_RPN_LENGTH : usize = 100;
        if str.is_empty() {
            return Ok(None);
        }
        if str.len() > MAX_RPN_LENGTH {
            return Err(format!("Input string is too long. Max length is {}, this is to prevent stack overflows", MAX_RPN_LENGTH));
        }

        let mut stack: Vec<Rc<RefCell<Node>>> = Vec::new();
        let mut name = "".to_string();
        for c in str.chars() {
            name = increment_name(name);
            match c {
                'A'..='Z' => stack.push(Node::new_lit(None, c.to_string(), c.to_string()).to_rc()),
                '!' => {
                    let child = stack.pop().ok_or_else(|| "Expected target for NOT".to_string())?;
                    let node = Node::new_not(Some(child.clone()), None, String::from(name.clone())).to_rc();
                    child.borrow_mut().parent = Some(node.clone());
                    stack.push(node);
                },
                '&' => {
                    let right = stack.pop().ok_or_else(|| "Expected right-hand operator for &".to_string())?;
                    let left = stack.pop().ok_or_else(|| "Expected left-hand operator for &".to_string())?;
                    let node = Node::new_and(Some(right.clone()), Some(left.clone()), None, String::from(name.clone())).to_rc();
                    right.borrow_mut().parent = Some(node.clone());
                    left.borrow_mut().parent = Some(node.clone());
                    stack.push(node);
                },
                '|' => {
                    let right = stack.pop().ok_or_else(|| "Expected right-hand operator for |".to_string())?;
                    let left = stack.pop().ok_or_else(|| "Expected left-hand operator for |".to_string())?;
                    let node = Node::new_or(Some(right.clone()), Some(left.clone()), None, String::from(name.clone())).to_rc();
                    right.borrow_mut().parent = Some(node.clone());
                    left.borrow_mut().parent = Some(node.clone());
                    stack.push(node);
                },
                '=' => {
                    let right_a = stack.pop().ok_or_else(|| "Expected right-hand operator for =".to_string())?;
                    let left_a = stack.pop().ok_or_else(|| "Expected left-hand operator for =".to_string())?;

                    let right_b = right_a.borrow().duplicate().to_rc();
                    let left_b = left_a.borrow().duplicate().to_rc();

                    let node_and_left = Node::new_and(
                        Some(left_a),
                        Some(right_a),
                        None, String::from(name.clone())).to_rc();

                    let node_and_right = Node::new_and(
                        Some(Node::negate_subtree(&left_b.borrow())),
                        Some(Node::negate_subtree(&right_b.borrow())),
                        None, String::from(name.clone())).to_rc();

                    node_and_left.borrow().right.clone().unwrap().borrow_mut().parent = Some(node_and_left.clone());
                    node_and_left.borrow().left.clone().unwrap().borrow_mut().parent = Some(node_and_left.clone());
                    node_and_right.borrow().right.clone().unwrap().borrow_mut().parent = Some(node_and_right.clone());
                    node_and_right.borrow().left.clone().unwrap().borrow_mut().parent = Some(node_and_right.clone());

                    let node = Node::new_or(Some(node_and_left.clone()), Some(node_and_right.clone()), None, String::from(name.clone())).to_rc();
                    node_and_left.borrow_mut().parent = Some(node.clone());
                    node_and_right.borrow_mut().parent = Some(node.clone());
                    stack.push(node);
                },
                '^' => {
                    let right_a = stack.pop().ok_or_else(|| "Expected right-hand operator for ^".to_string())?;
                    let left_a = stack.pop().ok_or_else(|| "Expected left-hand operator for ^".to_string())?;

                    let right_b = right_a.borrow().duplicate().to_rc();
                    let left_b = left_a.borrow().duplicate().to_rc();

                    let node_and_left = Node::new_and(
                        Some(left_a),
                        Some(Node::negate_subtree(&right_a.borrow())),
                        None, String::from(name.clone())).to_rc();

                    let node_and_right = Node::new_and(
                        Some(Node::negate_subtree(&left_b.borrow())),
                        Some(right_b),
                        None, String::from(name.clone())).to_rc();

                    node_and_left.borrow().right.clone().unwrap().borrow_mut().parent = Some(node_and_left.clone());
                    node_and_left.borrow().left.clone().unwrap().borrow_mut().parent = Some(node_and_left.clone());
                    node_and_right.borrow().right.clone().unwrap().borrow_mut().parent = Some(node_and_right.clone());
                    node_and_right.borrow().left.clone().unwrap().borrow_mut().parent = Some(node_and_right.clone());

                    let node = Node::new_or(Some(node_and_left.clone()), Some(node_and_right.clone()), None, String::from(name.clone())).to_rc();
                    node_and_left.borrow_mut().parent = Some(node.clone());
                    node_and_right.borrow_mut().parent = Some(node.clone());
                    stack.push(node);
                },
                '>' => {
                    let right = stack.pop().ok_or_else(|| "Expected right-hand operator for >".to_string())?;
                    let left = stack.pop().ok_or_else(|| "Expected left-hand operator for >".to_string())?;
                    // stack.push(Node::new_or(Node::negate_box(Some(left)), Some(right)));
                    let node = Node::new_or(
                        Some(Node::negate_subtree(&left.borrow())),
                        Some(right),
                        None, String::from(name.clone())).to_rc();
                    node.borrow().left.clone().unwrap().borrow_mut().parent = Some(node.clone());
                    node.borrow().right.clone().unwrap().borrow_mut().parent = Some(node.clone());
                    stack.push(node);
                },
                ' ' => continue,
                _ => return Err(format!("Invalid character: {}", c)),
            }
        }
        return Ok(stack.pop());

        fn increment_name(mut name: String) -> String{
            let mut chars: Vec<char> = name.chars().collect();
            let mut carry = true;
            println!("Incrementing name '{}'", name);

            for i in (0..chars.len()).rev() {
                if carry {
                    if chars[i] == 'Z' {
                        chars[i] = 'A';
                    } else {
                        chars[i] = (chars[i] as u8 + 1) as char;
                        carry = false;
                    }
                }
            }
            if carry {
                chars.insert(0, 'A');
            }
            name.clear();
            name.extend(chars);
            name
        }
    }
}
