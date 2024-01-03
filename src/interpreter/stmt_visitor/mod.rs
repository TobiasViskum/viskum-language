mod helper_methods;

use std::{ rc::Rc, cell::RefCell };

use crate::{
    stmt::*,
    error_handler::{ ViskumError, AbortReason },
    environment::{ environment_value::EnvironmentValue, Environment },
    token::{ TokenType, Literal },
    viskum_function::ViskumFunction,
    viskum_callable::Callable,
};

use super::Interpreter;

type Output = ();

impl<'a> StmtVisitor<Output> for Interpreter<'a> {
    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<Output, ViskumError> {
        let e = self.environment.borrow().clone();
        self.execute_block(
            &stmt.statements,
            Rc::new(RefCell::new(Environment::new_with_enclosing(e)))
        )
    }

    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<Output, ViskumError> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_if_stmt(&self, stmt: &IfStmt) -> Result<Output, ViskumError> {
        if self.is_truthy(&self.evaluate(&stmt.condition)?) {
            self.execute(&stmt.then_branch)?;
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(&else_branch)?;
        }

        Ok(())
    }

    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<Output, ViskumError> {
        let expr = self.evaluate(&stmt.expression)?;
        println!("{}", expr);
        Ok(())
    }

    fn visit_let_stmt(&self, stmt: &LetStmt) -> Result<Output, ViskumError> {
        let value = self.evaluate(&stmt.initializer)?;

        self.environment_define(&stmt.token, EnvironmentValue::new(value, false))?;

        Ok(())
    }

    fn visit_while_stmt(&self, stmt: &WhileStmt) -> Result<Output, ViskumError> {
        while self.is_truthy(&self.evaluate(&stmt.condition)?) {
            match self.execute(&stmt.body) {
                Ok(_) => (),
                Err(e) => {
                    if e.is_abort_error(AbortReason::Break) {
                        break;
                    } else if e.is_abort_error(AbortReason::Continue) {
                        continue;
                    } else {
                        return Err(e);
                    }
                }
            };
        }

        Ok(())
    }

    fn visit_loop_stmt(&self, stmt: &LoopStmt) -> Result<Output, ViskumError> {
        loop {
            match self.execute(&stmt.body) {
                Ok(_) => (),
                Err(e) => {
                    if e.is_abort_error(AbortReason::Break) {
                        break;
                    } else if e.is_abort_error(AbortReason::Continue) {
                        continue;
                    } else {
                        return Err(e);
                    }
                }
            };
        }

        Ok(())
    }

    fn visit_loopcontrol_stmt(&self, stmt: &LoopControlStmt) -> Result<Output, ViskumError> {
        match stmt.keyword.ttype {
            TokenType::Break =>
                Err(
                    ViskumError::new_with_abort(
                        "Unexpected break statement: Must be inside of a loop",
                        stmt.keyword.clone(),
                        "file.vs",
                        AbortReason::Break
                    )
                ),
            TokenType::Continue =>
                Err(
                    ViskumError::new_with_abort(
                        "Unexpected continue statement: Must be inside of a loop",
                        stmt.keyword.clone(),
                        "file.vs",
                        AbortReason::Continue
                    )
                ),
            _ =>
                Err(
                    ViskumError::new(
                        format!("Unknown loop keyword: '{}'", stmt.keyword.lexeme).as_str(),
                        stmt.keyword.clone(),
                        "file.vs"
                    )
                ),
        }
    }

    fn visit_function_stmt(&self, stmt: &FunctionStmt) -> Result<Output, ViskumError> {
        let function = ViskumFunction::new(stmt.clone(), self.environment.borrow().clone());

        self.environment_define(
            &stmt.token,
            EnvironmentValue::new(
                Literal::Func(Callable { func: Rc::new(function), arity: stmt.params.len() }),
                false
            )
        )?;

        Ok(())
    }

    fn visit_return_stmt(&self, stmt: &ReturnStmt) -> Result<Output, ViskumError> {
        let value = if let Some(value) = &stmt.value {
            self.evaluate(&value)?
        } else {
            Literal::Null
        };

        Err(
            ViskumError::new_with_abort(
                "Unexptected return statement. Must be inside of a function",
                stmt.keyword.clone(),
                "file.vs",
                AbortReason::Return(value)
            )
        )
    }
}
