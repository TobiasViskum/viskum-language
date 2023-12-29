use crate::{ stmt::*, token::TokenType, error_handler::ViskumError };

use super::Parser;

impl<'a> Parser<'a> {
    pub(super) fn print_statement(&mut self) -> Result<Stmt, ViskumError> {
        let value = self.expression()?;

        self.consume(TokenType::Semicolon, "Epxected ';' after expression")?;

        Ok(Stmt::Print(PrintStmt { expression: Box::from(value) }))
    }

    pub(super) fn expression_statement(&mut self) -> Result<Stmt, ViskumError> {
        let expr = self.expression()?;

        self.consume(TokenType::Semicolon, "Expected ';' after expression")?;

        Ok(Stmt::Expression(ExpressionStmt { expression: Box::from(expr) }))
    }
}
