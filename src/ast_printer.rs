use crate::expr::*;

struct AstPrinter;

impl AstPrinter {
    fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &String, exprs: &Vec<&Box<Expr>>) -> String {
        let mut str_builder = format!("({}", name);
        for expr in exprs {
            str_builder = format!("{str_builder} {}", expr.accept(self));
        }
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

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> String {
        self.parenthesize(&expr.operator.lexeme, &vec![&expr.right])
    }
}

mod tests {
    use crate::{
        expr::{ Expr, BinaryExpr, UnaryExpr, LiteralExpr, GroupingExpr },
        token::{ TokenType, Token, Literal },
    };

    use super::AstPrinter;

    #[test]
    fn check_ast_printer() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::from(
                Expr::Unary(UnaryExpr {
                    operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
                    right: Box::from(
                        Expr::Literal(LiteralExpr { value: Some(Literal::Num(123.0)) })
                    ),
                })
            ),
            operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
            right: Box::from(
                Expr::Grouping(GroupingExpr {
                    expression: Box::from(
                        Expr::Literal(LiteralExpr { value: Some(Literal::Num(45.67)) })
                    ),
                })
            ),
        });

        let ast_printer = AstPrinter;

        println!("{:?}", ast_printer.print(&expr))
    }
}
