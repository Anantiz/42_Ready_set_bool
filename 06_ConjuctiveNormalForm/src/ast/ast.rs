use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Copy)]
pub enum Expr {
    Lit(char),
    Not(),
    And(),
    Or(),
}

#[derive(Clone)]
pub struct AstNode {
    pub data: Expr,
    pub left: Option<Rc<RefCell<AstNode>>>,
    pub right: Option<Rc<RefCell<AstNode>>>,
    pub name: String,
}
