use crate::token::Token;
use crate::token::Literal;

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Prefix(PrefixExpr),
    Postfix(PostfixExpr),
    Ternary(TernaryExpr),
}
impl Expr {
    pub fn accept<T>(&self, expr_visitor: &dyn ExprVisitor<T>) -> String {
        match self {
            Expr::Binary(expr) => expr.accept(expr_visitor),
            Expr::Grouping(expr) => expr.accept(expr_visitor),
            Expr::Literal(expr) => expr.accept(expr_visitor),
            Expr::Prefix(expr) => expr.accept(expr_visitor),
            Expr::Postfix(expr) => expr.accept(expr_visitor),
            Expr::Ternary(expr) => expr.accept(expr_visitor),
        }
    }
}
#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}
#[derive(Debug)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}
#[derive(Debug)]
pub struct LiteralExpr {
    pub value: Option<Literal>,
}
#[derive(Debug)]
pub struct PrefixExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}
#[derive(Debug)]
pub struct PostfixExpr {
    pub left: Box<Expr>,
    pub operator: Token,
}
#[derive(Debug)]
pub struct TernaryExpr {
    pub condition: Box<Expr>,
    pub true_expr: Box<Expr>,
    pub false_expr: Box<Expr>,
}
pub trait ExprVisitor<T> {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> String;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> String;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> String;
    fn visit_prefix_expr(&self, expr: &PrefixExpr) -> String;
    fn visit_postfix_expr(&self, expr: &PostfixExpr) -> String;
    fn visit_ternary_expr(&self, expr: &TernaryExpr) -> String;
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

impl PrefixExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> String {
        visitor.visit_prefix_expr(self)
    }
}

impl PostfixExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> String {
        visitor.visit_postfix_expr(self)
    }
}

impl TernaryExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> String {
        visitor.visit_ternary_expr(self)
    }
}

