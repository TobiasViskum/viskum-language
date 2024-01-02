mod expr_visitor;
mod stmt_visitor;
mod helper_methods;

use std::{ rc::Rc, cell::RefCell };

use crate::{
    expr::*,
    token::Literal,
    error_handler::{ ErrorHandler, ViskumError },
    stmt::Stmt,
    environment::Environment,
};

pub struct Interpreter<'a> {
    error_handler: &'a RefCell<ErrorHandler>,
    environment: &'a RefCell<Rc<RefCell<Environment>>>,
}

type Output = Literal;

impl<'a> Interpreter<'a> {
    pub fn new(
        error_handler: &'a RefCell<ErrorHandler>,
        environment: &'a RefCell<Rc<RefCell<Environment>>>
    ) -> Self {
        Interpreter {
            error_handler: error_handler,
            environment: environment,
        }
    }

    pub fn interpret(&self, statements: Vec<Stmt>) {
        for stmt in &statements {
            match self.execute(stmt) {
                Ok(_) => (),
                Err(e) => {
                    (*self.error_handler).borrow_mut().report_error(e);
                    (*self.error_handler).borrow_mut().print_errors();
                    break;
                }
            }
        }
    }

    fn execute(&self, stmt: &Stmt) -> Result<(), ViskumError> {
        stmt.accept(self)
    }

    fn evaluate(&self, expr: &Expr) -> Result<Output, ViskumError> {
        expr.accept(self)
    }

    fn is_truthy(&self, literal: &Literal) -> bool {
        match literal {
            Literal::Bool(false) | Literal::Null => false,
            Literal::Num(x) => {
                if *x == 0.0 { false } else { true }
            }
            _ => true,
        }
    }
}
