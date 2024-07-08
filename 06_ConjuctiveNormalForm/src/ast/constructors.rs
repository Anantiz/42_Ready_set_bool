use crate::ast::ast::AstNode;
use crate::ast::ast::Expr;
use std::rc::Rc;
use std::cell::RefCell;

impl AstNode {
    pub fn new_literal(c: char) -> Rc<RefCell<AstNode>> {
        Rc::new(RefCell::new(AstNode {
            data: Expr::Lit(c),
            left: None,
            right: None,
            name: String::new(),
        }))
    }

    pub fn new_not(child: Option<Rc<RefCell<AstNode>>>) -> Rc<RefCell<AstNode>> {
        Rc::new(RefCell::new(AstNode {
            data: Expr::Not(),
            left: child,
            right: None,
            name: String::new(),
        }))
    }

    pub fn new_and(left: Option<Rc<RefCell<AstNode>>>, right: Option<Rc<RefCell<AstNode>>>) -> Rc<RefCell<AstNode>> {
        Rc::new(RefCell::new(AstNode {
            data: Expr::And(),
            left: left,
            right: right,
            name: String::new(),
        }))
    }

    pub fn new_or(left: Option<Rc<RefCell<AstNode>>>, right: Option<Rc<RefCell<AstNode>>>) -> Rc<RefCell<AstNode>> {
        Rc::new(RefCell::new(AstNode {
            data: Expr::Or(),
            left: left,
            right: right,
            name: String::new(),
        }))
    }
}
