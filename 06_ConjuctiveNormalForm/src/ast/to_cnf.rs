/*
NEGATIONS
    Expression:
        ¬(a)
        |
        x(b)

    CNF:
        (¬a ∨ ¬b) ∧ (b ∨ a).

CONJUCTIONS
    Expression:
        ∧(a)
       / \
    x(b)  y(c)

    CNF:
        (¬a ∨ b) ∧ (¬a ∨ c) ∧ (¬b ∨ ¬c ∨ a)

DISJUNCTION:
    Expression:
        ∨(a)
       / \
    x(b)  y(c)

    CNF:
        (¬a ∨ b ∨ c) ∧ (¬b ∨ a) ∧ (¬c ∨ a)



*/

use crate::ast::ast::AstNode;
use crate::ast::ast::Expr;

impl AstNode
{
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
        return AstNode::to_cnf_tseytin(AstNode::to_negation_normal_form(self))
    }

    fn to_cnf_tseytin(mut nnf_node_option : Option<Box<AstNode>>) -> Option<Box<AstNode>>
    {
        match nnf_node_option
        {
            None => None,
            Some(nnf_node) => match nnf_node.as_ref().data
            {
                Expr::Not() => preppend_tree_as_and(tseytin_not(nnf_node)),
                Expr::And() => preppend_tree_as_and(tseytin_and(nnf_node)),
                Expr::Or() => preppend_tree_as_and(tseytin_or(nnf_node)),
                Expr::Lit(_) => None,
            },
        }
    }
}