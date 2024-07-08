use crate::ast::ast::AstNode;
use crate::ast::ast::Expr;

use lazy_static::lazy_static;
use std::sync::Mutex;
use std::rc::Rc;
use std::cell::RefCell;

lazy_static! {
    static ref GLOB_NAME: Mutex<String> = Mutex::new(String::new());
}

fn get_char(str: &str) -> char {
    let c = str.chars().nth(0);
    match c {
        Some(c) => c,
        None => '0',
    }
}

impl AstNode {
    pub fn name_inner_nodes(&mut self) -> &mut Self {
        /// A -> B ... -> AA -> AZ ... -> BA ...
        fn increment_name(name: &mut String) {
            let mut chars: Vec<char> = name.chars().collect();
            let mut carry = true;

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
        }

        fn iter_tree(node: &mut Rc<RefCell<AstNode>>)
        {
            // Drops the first borrow or returns early
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                return
            }
            let node_cell = node.clone();
            let mut data = GLOB_NAME.lock().unwrap();
            node_cell.borrow_mut().name = data.clone();
            increment_name(&mut data);
            drop(data);
            if let Some(mut left) = node_cell.clone().borrow().left.clone() {
                iter_tree(&mut left);
            }
            if let Some(mut right) = node_cell.clone().borrow().right.clone() {
                iter_tree(&mut right);
            }
        }

        let mut data = GLOB_NAME.lock().unwrap();
        *data = String::from("A");
        drop(data);
        iter_tree(&mut Rc::new(RefCell::new(self.clone())));
        self
    }

    pub fn to_cnf(mut self) -> Option<Rc<RefCell<AstNode>>>
    {
        self.name_inner_nodes();
        let root = AstNode::new_and(None, None);
        let tail_opt = AstNode::to_cnf_tseytin(AstNode::to_negation_normal_form(self), Some(root.clone()));

        if tail_opt.is_none() {
            println!("\x1b[31mTail option is none\x1b[0m");
            return None;
        }
        // Swap the value of tail with tail.left as our recursive function
        // will always end with an And with no right child
        let tail = tail_opt.unwrap();
        tail.swap(&root.borrow_mut().left.take().as_mut().unwrap());

        println!("CNF DONE");
        if root.borrow().left.is_none() || root.borrow().right.is_none() {
            println!("\x1b[31mRoot has a None child\x1b[0m");
            None
        } else {
            Some(root)
        }
    }

    fn to_cnf_tseytin(nnf_node_option: Option<Rc<RefCell<AstNode>>>, cnf_tail: Option<Rc<RefCell<AstNode>>>
    ) -> Option<Rc<RefCell<AstNode>>>
    {
        println!("CNF once more");
        let new_cnf_tail: Option<Rc<RefCell<AstNode>>> = cnf_subtree(&nnf_node_option, cnf_tail.clone());
        if new_cnf_tail.is_none() {
            println!("CNF FAILED");
            return cnf_tail;
        }
        let nnf_node = nnf_node_option.unwrap();
        let nnf_node = nnf_node.borrow();
        return match nnf_node.data
        {
            Expr::Lit(_) => {
                println!("CNF LIT");
                new_cnf_tail
            },
            Expr::Not() => AstNode::to_cnf_tseytin(nnf_node.left.clone(), new_cnf_tail),
            Expr::And() | Expr::Or() => {
                let new_cnf_tail = AstNode::to_cnf_tseytin(nnf_node.left.clone(), new_cnf_tail);
                AstNode::to_cnf_tseytin(nnf_node.right.clone(), new_cnf_tail)
            },
        };
        fn cnf_subtree(nnf_node_option: &Option<Rc<RefCell<AstNode>>>, cnf_tail: Option<Rc<RefCell<AstNode>>>,
        ) -> Option<Rc<RefCell<AstNode>>>
        {
            return match nnf_node_option {
                None => None,
                Some(nnf_node) => match nnf_node.borrow().data {
                    Expr::Not() => merge_trees(cnf_tail.unwrap(), tseytin_not(nnf_node)),
                    Expr::And() => merge_trees(cnf_tail.unwrap(), tseytin_and(nnf_node)),
                    Expr::Or() => merge_trees(cnf_tail.unwrap(), tseytin_or(nnf_node)),
                    Expr::Lit(_) => cnf_tail,
                },
            };

            fn tseytin_not(nnf_node: &Rc<RefCell<AstNode>>
            ) -> Rc<RefCell<AstNode>>
            {
               AstNode::new_and(
                    Some(AstNode::new_or(
                        Some(AstNode::new_literal(get_char(&nnf_node.borrow().name))),
                        Some(AstNode::new_literal(get_char(
                            &nnf_node.borrow().left.as_ref().unwrap().borrow().name,
                        ))),
                    )),
                    Some(AstNode::new_or(
                        Some(AstNode::new_not(Some(AstNode::new_literal(get_char(
                            &nnf_node.borrow().name,
                        ))))),
                        Some(AstNode::new_not(Some(AstNode::new_literal(get_char(
                            &nnf_node.borrow().left.as_ref().unwrap().borrow().name,
                        ))))),
                    )),
                )
            }

            fn tseytin_and(nnf_node: &Rc<RefCell<AstNode>>
            ) -> Rc<RefCell<AstNode>>
            {
                AstNode::new_and(
                    Some(AstNode::new_or(
                        Some(AstNode::new_not(Some(AstNode::new_literal(get_char(
                            &nnf_node.borrow().name,
                        ))))),
                        Some(AstNode::new_literal(get_char(
                            &nnf_node.borrow().left.as_ref().unwrap().borrow().name,
                        ))),
                    )),
                    Some(AstNode::new_and(
                        Some(AstNode::new_or(
                            Some(AstNode::new_not(Some(AstNode::new_literal(get_char(
                                &nnf_node.borrow().name,
                            ))))),
                            Some(AstNode::new_literal(get_char(
                                &nnf_node.borrow().right.as_ref().unwrap().borrow().name,
                            ))),
                        )),
                        Some(AstNode::new_or(
                            Some(AstNode::new_not(Some(AstNode::new_literal(get_char(
                                &nnf_node.borrow().left.as_ref().unwrap().borrow().name,
                            ))))),
                            Some(AstNode::new_or(
                                Some(AstNode::new_not(Some(AstNode::new_literal(get_char(
                                    &nnf_node.borrow().right.as_ref().unwrap().borrow().name,
                                ))))),
                                Some(AstNode::new_literal(get_char(
                                    &nnf_node.borrow().name,
                                ))),
                            )),
                        )),
                    )),
                )
            }

            fn tseytin_or(nnf_node: &Rc<RefCell<AstNode>>
            ) -> Rc<RefCell<AstNode>>
            {
                AstNode::new_and(
                    Some(AstNode::new_or(
                        Some(AstNode::new_not(Some(AstNode::new_literal(get_char(
                            &nnf_node.borrow().name,
                        ))))),
                        Some(AstNode::new_or(
                            Some(AstNode::new_literal(get_char(
                                &nnf_node.borrow().left.as_ref().unwrap().borrow().name,
                            ))),
                            Some(AstNode::new_literal(get_char(
                                &nnf_node.borrow().right.as_ref().unwrap().borrow().name,
                            ))),
                        )),
                    )),
                    Some(AstNode::new_and(
                        Some(AstNode::new_or(
                            Some(AstNode::new_not(Some(AstNode::new_literal(get_char(
                                &nnf_node.borrow().left.as_ref().unwrap().borrow().name,
                            ))))),
                            Some(AstNode::new_literal(get_char(
                                &nnf_node.borrow().name,
                            ))),
                        )),
                        Some(AstNode::new_or(
                            Some(AstNode::new_not(Some(AstNode::new_literal(get_char(
                                &nnf_node.borrow().right.as_ref().unwrap().borrow().name,
                            ))))),
                            Some(AstNode::new_literal(get_char(
                                &nnf_node.borrow().name,
                            ))),
                        )),
                    )),
                )
            }

            fn merge_trees(parent_tree_tail: Rc<RefCell<AstNode>>, subtree: Rc<RefCell<AstNode>>,
            ) -> Option<Rc<RefCell<AstNode>>>
            {
                if parent_tree_tail.borrow().left.is_none() {
                    parent_tree_tail.borrow_mut().left = Some(subtree);
                    return Some(parent_tree_tail);
                }
                else {
                    parent_tree_tail.borrow_mut().right = Some(AstNode::new_and(None, None));
                    merge_trees(parent_tree_tail.borrow_mut().right.clone().unwrap(), subtree)
                }
            }
        }
    }
}
