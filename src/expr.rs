use crate::error_handler::ViskumError;
use crate::token::Token;
use crate::token::Literal;

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}
pub struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}
pub struct GroupingExpr {
    expression: Box<Expr>,
}
pub struct LiteralExpr {
    value: Literal,
}
pub struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}
pub trait ExprVisitor<T> {
    fn visit_binary_expr(&self, expr: &BinaryExpr);
    fn visit_grouping_expr(&self, expr: &GroupingExpr);
    fn visit_literal_expr(&self, expr: &LiteralExpr);
    fn visit_unary_expr(&self, expr: &UnaryExpr);
}

impl BinaryExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) {
        visitor.visit_binary_expr(self)
    }
}

impl GroupingExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) {
        visitor.visit_grouping_expr(self)
    }
}

impl LiteralExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) {
        visitor.visit_literal_expr(self)
    }
}

impl UnaryExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) {
        visitor.visit_unary_expr(self)
    }
}

