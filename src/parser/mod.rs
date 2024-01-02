use std::cell::RefCell;

mod helper_methods;
mod expression_methods;
mod statements;

use crate::{
    token::{ Token, TokenType },
    error_handler::{ ErrorHandler, ViskumError },
    stmt::{ Stmt, BlockStmt, LoopControlStmt },
};

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
    error_handler: &'a RefCell<ErrorHandler>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>, error_handler: &'a RefCell<ErrorHandler>) -> Self {
        Parser { tokens: tokens, current: 0, error_handler: error_handler }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ViskumError> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.is_at_end()? {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    //main method
    fn declaration(&mut self) -> Result<Stmt, ViskumError> {
        let result = if self.match_tokens(&[TokenType::Let])? {
            self.variable_declaration()
        } else if self.match_tokens(&[TokenType::Fn])? {
            self.function_declaration("function".to_string())
        } else {
            self.statement()
        };

        if result.is_err() {
            self.synchronize()?;
        }

        result
    }

    fn statement(&mut self) -> Result<Stmt, ViskumError> {
        if self.match_tokens(&[TokenType::While])? {
            self.while_statement()
        } else if self.match_tokens(&[TokenType::Loop])? {
            self.loop_statement()
        } else if self.match_tokens(&[TokenType::Break, TokenType::Continue])? {
            let keyword = self.peek_previous()?;
            self.consume(TokenType::Semicolon, "Expected ';' after loop control statement")?;
            Ok(Stmt::LoopControl(LoopControlStmt { keyword: keyword }))
        } else if self.match_tokens(&[TokenType::If])? {
            self.if_statement()
        } else if self.match_tokens(&[TokenType::Print])? {
            self.print_statement()
        } else if self.match_tokens(&[TokenType::LeftBrace])? {
            Ok(Stmt::Block(BlockStmt { statements: self.block()? }))
        } else {
            self.expression_statement()
        }
    }
}
