use crate::{
    stmt::*,
    error_handler::ViskumError,
    environment::environment_value::EnvironmentValue,
};

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

    fn visit_let_stmt(&self, stmt: &LetStmt) -> Result<Output, ViskumError> {
        let value = self.evaluate(&stmt.initializer)?;

        self.environment_define(&stmt.token, EnvironmentValue::new(value, false));

        Ok(())
    }
}
