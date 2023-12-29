use crate::{ expr::*, error_handler::ViskumError };

pub struct AstPrinter;

type Output = String;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) {
        if let Ok(printed) = expr.accept(self) {
            println!("{}", printed)
        } else {
            println!("Error while printing")
        }
    }

    fn parenthesize(&self, name: &String, exprs: &Vec<&Box<Expr>>) -> Result<Output, ViskumError> {
        let mut str_builder = format!("({}", name);
        for expr in exprs {
            str_builder = format!("{str_builder} {}", expr.accept(self)?);
        }
        str_builder = format!("{str_builder})");

        Ok(str_builder)
    }

    fn parenthesize_postfix(
        &self,
        name: &String,
        exprs: &Vec<&Box<Expr>>
    ) -> Result<Output, ViskumError> {
        let mut str_builder = format!("{})", name);
        for expr in exprs {
            str_builder = format!("{} {str_builder}", expr.accept(self)?);
        }
        str_builder = format!("({str_builder}");

        Ok(str_builder)
    }

    fn parenthesize_ternary(
        &self,
        condition_expr: &Box<Expr>,
        true_expr: &Box<Expr>,
        false_expr: &Box<Expr>
    ) -> Result<Output, ViskumError> {
        let mut str_builder = format!("(condition {}", condition_expr.accept(self)?);

        str_builder = format!("{str_builder} (if_true {})", true_expr.accept(self)?);

        str_builder = format!("{str_builder} (if_false {})", false_expr.accept(self)?);

        str_builder = format!("{str_builder})");

        Ok(str_builder)
    }
}

impl ExprVisitor<Output> for AstPrinter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Output, ViskumError> {
        self.parenthesize(&expr.operator.lexeme, &vec![&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Output, ViskumError> {
        self.parenthesize(&"group".to_string(), &vec![&expr.expression])
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Output, ViskumError> {
        if let Some(v) = &expr.value { Ok(v.to_string()) } else { Ok("null".to_string()) }
    }

    fn visit_prefix_expr(&self, expr: &PrefixExpr) -> Result<Output, ViskumError> {
        self.parenthesize(&expr.operator.lexeme, &vec![&expr.right])
    }

    fn visit_postfix_expr(&self, expr: &PostfixExpr) -> Result<Output, ViskumError> {
        self.parenthesize_postfix(&expr.operator.lexeme, &vec![&expr.left])
    }

    fn visit_ternary_expr(&self, expr: &TernaryExpr) -> Result<Output, ViskumError> {
        self.parenthesize_ternary(&expr.condition, &expr.true_expr, &expr.false_expr)
    }
    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<Output, ViskumError> {
        todo!()
    }
    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<Output, ViskumError> {
        todo!()
    }
}
