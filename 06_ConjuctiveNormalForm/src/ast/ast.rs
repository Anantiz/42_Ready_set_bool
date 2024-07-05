#[derive(Clone, Copy)]
pub enum Expr
{
    Lit(char),
    Not(),
    And(),
    Or(),
}

#[derive(Clone)]
pub struct AstNode
{
    pub data : Expr,
    pub left : Option<Box<AstNode>>,
    pub right : Option<Box<AstNode>>,
    pub name : String
}