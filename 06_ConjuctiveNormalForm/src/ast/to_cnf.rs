use crate::ast::ast::AstNode;
use crate::ast::ast::Expr;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GLOB_NAME: Mutex<String> = Mutex::new(String::new());
}

fn get_char(str : &str) -> char
{
    let c = str.chars().nth(0);
    match c
    {
        Some(c) => c,
        None => '0'
    }
}

impl AstNode
{
    pub fn name_inner_nodes(&mut self) -> &mut Self
    {
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

        fn iter_tree(node: &mut AstNode) {
            if node.left.is_some() // right cannot be if left is not
            {
                let mut data = GLOB_NAME.lock().unwrap();
                node.name = String::from(&*data);
                increment_name(&mut *data);
                if let Some(ref mut left) = node.left {
                    iter_tree(left);
                }
                if let Some(ref mut right) = node.right {
                    iter_tree(right);
                }
            }
        }

        let mut data = GLOB_NAME.lock().unwrap();
        *data = String::from("A");
        drop(data);
        iter_tree(self);
        self
    }

    /// A Conversion using the Tseytin Transformation
    /// Note to future self: What is the Tseytin Transformation?
    /// 	It's a method to keep a compact representation of a formula
    /// 	In CNF form, a CNF formula that keeps logical equivalence will
    /// 	scale exponentially with the number of variables.
    /// 	However, for CNF to work we do not need to keep the logical equivalence
    /// 	it just has to be equisatisfiable (Meaning that if one has a solution the other has too)
    pub fn to_cnf(mut self) -> Option<Box<AstNode>>
    {
        /* Steps:

        1. Convert to Negation Normal Form
            (So you only have Literals, Conjonctions and Disjunctions,
            it removes all other fancy operators)
        2. Create new intermediary variables for each internal node (non-leaves)
        3. For each of these variables, append each of their clauses to the tree as a new conjunction
            (Haha so easy)
        */
        self.name_inner_nodes(); // I want to tell this is safe code, not unsafe, how to enforce the compiler to accept this
        let cnf_new_tree = Some(AstNode::new_and(None, None)); // Holder
        AstNode::to_cnf_tseytin(AstNode::to_negation_normal_form(self), &cnf_new_tree);
        return (cnf_new_tree.unwrap()).left
    }

    fn to_cnf_tseytin(nnf_node_option : Option<Box<AstNode>>, cnf_new_tree : &Option<Box<AstNode>>)
    {
        fn cnf_subtree(mut cnf_tail : &Option<Box<AstNode>>) -> Option<Box<AstNode>>
        {
            /*
            NEGATIONS
                Expression:
                    ¬(a)
                    |
                    x(b)
                CNF:
                    (¬a ∨ ¬b) ∧ (b ∨ a).
            */
            fn tseytin_not(nnf_node : &Box<AstNode>) -> Box<AstNode>
            {
                AstNode::new_and(
                    Some(AstNode::new_or(
                        Some(AstNode::new_literal(get_char(&nnf_node.name))),
                        Some(AstNode::new_literal(get_char(&nnf_node.left.as_ref().unwrap().name)))
                    )),
                    Some(AstNode::new_or(
                        Some(AstNode::new_not(Some(AstNode::new_literal(get_char(&nnf_node.name))))),
                        Some(AstNode::new_not(Some(AstNode::new_literal(get_char(&nnf_node.left.as_ref().unwrap().name)))))
                    )),
                )
            }

            /*
            CONJUCTIONS
                Expression:
                    ∧(a)
                / \
                x(b)  y(c)

                CNF:
                    (¬a ∨ b) ∧ (¬a ∨ c) ∧ (¬b ∨ ¬c ∨ a)
            */
            fn tseytin_and(nnf_node : &Box<AstNode>) -> Box<AstNode>
            {
                AstNode::new_and(
                    Some(AstNode::new_or(
                        Some(AstNode::new_not(Some(AstNode::new_literal(get_char(&nnf_node.name))))),
                        Some(AstNode::new_literal(get_char(&nnf_node.left.as_ref().unwrap().name)))
                    )),
                    Some(AstNode::new_and(
                        Some(AstNode::new_or(
                            Some(AstNode::new_not(Some(AstNode::new_literal(get_char(&nnf_node.name))))),
                            Some(AstNode::new_literal(get_char(&nnf_node.right.as_ref().unwrap().name)))
                        )),
                        Some(AstNode::new_or(
                            Some(AstNode::new_not(Some(AstNode::new_literal(get_char(&nnf_node.left.as_ref().unwrap().name))))),
                            Some(AstNode::new_or(
                                Some(AstNode::new_not(Some(AstNode::new_literal(get_char(&nnf_node.right.as_ref().unwrap().name))))),
                                Some(AstNode::new_literal(get_char(&nnf_node.name)))
                            ))
                        ))
                    ))
                )
            }

            /*
            DISJUNCTION:
                Expression:
                    ∨(a)
                / \
                x(b)  y(c)

                CNF:
                    (¬a ∨ b ∨ c) ∧ (¬b ∨ a) ∧ (¬c ∨ a)
            */
            fn tseytin_or(nnf_node : &Box<AstNode>) -> Box<AstNode>
            {
                AstNode::new_and(
                    Some(AstNode::new_or(
                        Some(AstNode::new_not(Some(AstNode::new_literal(get_char(&nnf_node.name))))),
                        Some(AstNode::new_or(
                            Some(AstNode::new_literal(get_char(&nnf_node.left.as_ref().unwrap().name))),
                            Some(AstNode::new_literal(get_char(&nnf_node.right.as_ref().unwrap().name)))
                        ))
                    )),
                    Some(AstNode::new_and(
                        Some(AstNode::new_or(
                            Some(AstNode::new_not(Some(AstNode::new_literal(get_char(&nnf_node.left.as_ref().unwrap().name))))),
                            Some(AstNode::new_literal(get_char(&nnf_node.name)))
                        )),
                        Some(AstNode::new_or(
                            Some(AstNode::new_not(Some(AstNode::new_literal(get_char(&nnf_node.right.as_ref().unwrap().name))))),
                            Some(AstNode::new_literal(get_char(&nnf_node.name)))
                        ))
                    ))
                )
            }

            fn add_and_node_with_subtree_to_right_of_an_and_node(tail : &Option<Box<AstNode>>, subtree : Box<AstNode>) -> Option<Box<AstNode>>
            {
                match tail
                {
                    None => Some(subtree),
                    Some(tail) => {
                        tail.right = Some(subtree);
                        Some(tail)
                    }
                }
            }
            /*
                For each node, get it's associated CNF subtree
                and add it to the CNF tree
            */

            match nnf_node_option
            {
                None => None,
                Some(nnf_node) => match nnf_node.as_ref().data
                {
                    Expr::Not() => add_and_node_with_subtree_to_right_of_an_and_node(tail, tseytin_not(&nnf_node)),
                    Expr::And() => add_and_node_with_subtree_to_right_of_an_and_node(tail, tseytin_and(&nnf_node)),
                    Expr::Or() => add_and_node_with_subtree_to_right_of_an_and_node(tail, tseytin_or(&nnf_node)),
                    Expr::Lit(_) => None,
                },
            }
        }

        let tail : Option<Box<AstNode>> = cnf_subtree(cnf_new_tree);
        if nnf_node_option.is_some()
        {
            AstNode::to_cnf_tseytin(nnf_node_option.unwrap().left, &tail);
            AstNode::to_cnf_tseytin(nnf_node_option.unwrap().right,&tail);
        }
    }
}

