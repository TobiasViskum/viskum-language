use crate::expr::*;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) {
        println!("{}", expr.accept(self))
    }

    fn parenthesize(&self, name: &String, exprs: &Vec<&Box<Expr>>) -> String {
        let mut str_builder = format!("({}", name);
        for expr in exprs {
            str_builder = format!("{str_builder} {}", expr.accept(self));
        }
        str_builder = format!("{str_builder})");

        str_builder
    }

    fn parenthesize_postfix(&self, name: &String, exprs: &Vec<&Box<Expr>>) -> String {
        let mut str_builder = format!("{})", name);
        for expr in exprs {
            str_builder = format!("{} {str_builder}", expr.accept(self));
        }
        str_builder = format!("({str_builder}");

        str_builder
    }

    fn parenthesize_ternary(
        &self,
        condition_expr: &Box<Expr>,
        true_expr: &Box<Expr>,
        false_expr: &Box<Expr>
    ) -> String {
        let mut str_builder = format!("(condition {}", condition_expr.accept(self));

        str_builder = format!("{str_builder} (if_true {})", true_expr.accept(self));

        str_builder = format!("{str_builder} (if_false {})", false_expr.accept(self));

        str_builder = format!("{str_builder})");

        str_builder
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> String {
        self.parenthesize(&expr.operator.lexeme, &vec![&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> String {
        self.parenthesize(&"group".to_string(), &vec![&expr.expression])
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> String {
        if let Some(v) = &expr.value { v.to_string() } else { "null".to_string() }
    }

    fn visit_prefix_expr(&self, expr: &PrefixExpr) -> String {
        self.parenthesize(&expr.operator.lexeme, &vec![&expr.right])
    }

    fn visit_postfix_expr(&self, expr: &PostfixExpr) -> String {
        self.parenthesize_postfix(&expr.operator.lexeme, &vec![&expr.left])
    }

    fn visit_ternary_expr(&self, expr: &TernaryExpr) -> String {
        self.parenthesize_ternary(&expr.condition, &expr.true_expr, &expr.false_expr)
    }
}
