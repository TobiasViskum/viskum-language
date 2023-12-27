use crate::token::Token;
use crate::token::Literal;

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    UnaryOp(UnaryOpExpr),
}
impl Expr {
    pub fn accept<T>(&self, expr_visitor: &dyn ExprVisitor<T>) -> String {
        match self {
            Expr::Binary(expr) => expr.accept(expr_visitor),
            Expr::Grouping(expr) => expr.accept(expr_visitor),
            Expr::Literal(expr) => expr.accept(expr_visitor),
            Expr::Unary(expr) => expr.accept(expr_visitor),
            Expr::UnaryOp(expr) => expr.accept(expr_visitor),
        }
    }
}
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}
pub struct LiteralExpr {
    pub value: Option<Literal>,
}
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}
pub struct UnaryOpExpr {
    pub left: Box<Expr>,
    pub operator: Token,
}
pub trait ExprVisitor<T> {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> String;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> String;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> String;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> String;
    fn visit_unaryop_expr(&self, expr: &UnaryOpExpr) -> String;
}

impl BinaryExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> String {
        visitor.visit_binary_expr(self)
    }
}

impl GroupingExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> String {
        visitor.visit_grouping_expr(self)
    }
}

impl LiteralExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> String {
        visitor.visit_literal_expr(self)
    }
}

impl UnaryExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> String {
        visitor.visit_unary_expr(self)
    }
}

impl UnaryOpExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> String {
        visitor.visit_unaryop_expr(self)
    }
}

