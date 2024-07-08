use crate::ast::ast::AstNode;
use crate::ast::ast::Expr;
use std::rc::Rc;
use std::cell::RefCell;

impl AstNode {
    /// Distribute NOT's operator such that it is only ever for literals
    pub fn to_negation_normal_form(mut self) -> Option<Rc<RefCell<AstNode>>> {
        match self.data {
            Expr::Lit(_) => Some(Rc::new(RefCell::new(self))),
            Expr::Not() => {
                let left = self.left.take().unwrap();
                let left_data = left.borrow().data.clone();
                if let Expr::Lit(_) = left_data {
                    return Some(Rc::new(RefCell::new(self)));
                } else {
                    return AstNode::to_negation_normal_form(AstNode::negate_box(Some(left)).unwrap().borrow_mut().clone());
                }
            },
            _ => {
                let left = self.left.take().unwrap();
                let right = self.right.take().unwrap();
                self.left = AstNode::to_negation_normal_form(left.borrow_mut().clone());
                self.right = AstNode::to_negation_normal_form(right.borrow_mut().clone());
                Some(Rc::new(RefCell::new(self)))
            }
        }
    }
}
