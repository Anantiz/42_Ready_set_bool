use crate::ast::ast::AstNode;
use crate::ast::ast::Expr;

impl AstNode
{
    pub fn new_literal(c : char) -> Box<AstNode>
    {
        Box::new(AstNode
        {
            data : Expr::Lit(c),
            left : None,
            right : None
        })
    }

    pub fn new_not(child : Option<Box<AstNode>>) -> Box<AstNode>
    {
        Box::new(AstNode
        {
            data : Expr::Not(),
            left : child,
            right : None
        })
    }

    pub fn new_and(left  : Option<Box<AstNode>>, right : Option<Box<AstNode>>) -> Box<AstNode>
    {
        Box::new(AstNode
        {
            data : Expr::And(),
            left : left,
            right : right
        })
    }

    pub fn new_or(left  : Option<Box<AstNode>>, right : Option<Box<AstNode>>) -> Box<AstNode>
    {
        Box::new(AstNode
        {
            data : Expr::Or(),
            left : left,
            right : right
        })
    }
}