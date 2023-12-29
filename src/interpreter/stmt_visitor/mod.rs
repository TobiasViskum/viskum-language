use crate::{ stmt::*, error_handler::ViskumError };

use super::Interpreter;

type Output = ();

impl<'a> StmtVisitor<Output> for Interpreter<'a> {
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<Output, ViskumError> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<Output, ViskumError> {
        let expr = self.evaluate(&stmt.expression)?;
        println!("{}", expr);
        Ok(())
    }
}
