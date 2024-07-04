use crate::ast::ast::AstNode;
use crate::ast::ast::Expr;

impl AstNode
{
	/// Distribute NOT's operator such that it is only ever for literals
	pub fn to_negation_normal_form(mut self) -> Option<Box<AstNode>>
	{
		match self.data
		{
			Expr::Lit(_) => Some(Box::new(self)),
			Expr::Not() =>
			{
				if let Expr::Lit(_) = self.left.as_ref().unwrap().data
				{
					return Some(Box::new(self))
				}
				else
				{
					return AstNode::to_negation_normal_form(*(AstNode::negate_box(self.left).unwrap()))
				}
			},
			_ => {
				self.left = AstNode::to_negation_normal_form(*self.left.take().unwrap());
				self.right = AstNode::to_negation_normal_form(*self.right.take().unwrap());
				Some(Box::new(self))
			}
		}
	}
}