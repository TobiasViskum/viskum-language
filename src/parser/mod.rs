use std::{ cell::RefCell, rc::Rc };

mod private_methods;
mod expression_methods;

use crate::{ token::Token, error_handler::ErrorHandler, expr::Expr, util::report_error };

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
    error_handler: &'a Rc<RefCell<ErrorHandler>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>, error_handler: &'a Rc<RefCell<ErrorHandler>>) -> Self {
        Parser { tokens: tokens, current: 0, error_handler: error_handler }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(e) => {
                report_error(&self.error_handler, e);
                None
            }
        }
    }
}
