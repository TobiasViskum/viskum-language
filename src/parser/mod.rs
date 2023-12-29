use std::{ cell::RefCell, rc::Rc };

mod private_methods;
mod expression_methods;
mod statements;

use crate::{
    token::{ Token, TokenType },
    error_handler::{ ErrorHandler, ViskumError },
    expr::Expr,
    util::report_error,
    stmt::Stmt,
};

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
    error_handler: &'a Rc<RefCell<ErrorHandler>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>, error_handler: &'a Rc<RefCell<ErrorHandler>>) -> Self {
        Parser { tokens: tokens, current: 0, error_handler: error_handler }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ViskumError> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.is_at_end()? {
            statements.push(self.statement()?);
        }

        Ok(statements)
    }

    //main method
    fn statement(&mut self) -> Result<Stmt, ViskumError> {
        if self.match_tokens(&[TokenType::Print])? {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }
}
