mod expr_visitor;
mod stmt_visitor;

use std::{ rc::Rc, cell::RefCell };

use crate::{ expr::*, token::Literal, error_handler::{ ErrorHandler, ViskumError }, stmt::Stmt };

pub struct Interpreter<'a> {
    error_handler: &'a Rc<RefCell<ErrorHandler>>,
}

type Output = Literal;

impl<'a> Interpreter<'a> {
    pub fn new(error_handler: &'a Rc<RefCell<ErrorHandler>>) -> Self {
        Interpreter { error_handler: error_handler }
    }

    pub fn interpret(&self, statements: Vec<Stmt>) {
        for stmt in statements {
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

    fn execute(&self, stmt: Stmt) -> Result<(), ViskumError> {
        stmt.accept(self)
    }

    fn evaluate(&self, expr: &Expr) -> Result<Output, ViskumError> {
        expr.accept(self)
    }
}
