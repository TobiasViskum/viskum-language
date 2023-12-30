use crate::{ stmt::*, token::TokenType, error_handler::ViskumError };

use super::Parser;

impl<'a> Parser<'a> {
    pub(super) fn print_statement(&mut self) -> Result<Stmt, ViskumError> {
        let value = self.expression()?;

        self.consume(TokenType::Semicolon, "Epxected ';' after expression")?;

        Ok(Stmt::Print(PrintStmt { expression: value }))
    }

    pub(super) fn expression_statement(&mut self) -> Result<Stmt, ViskumError> {
        let expr = self.expression()?;

        self.consume(TokenType::Semicolon, "Expected ';' after expression")?;

        Ok(Stmt::Expression(ExpressionStmt { expression: expr }))
    }

    pub(super) fn if_statement(&mut self) -> Result<Stmt, ViskumError> {
        let condition = self.expression()?;

        let peeked_token: crate::token::Token = self.peek()?;

        let then_branch = self.statement()?;

        if peeked_token.ttype == TokenType::LeftBrace {
            let else_branch = if self.match_tokens(&[TokenType::Else])? {
                Some(self.statement()?)
            } else {
                None
            };
            Ok(
                Stmt::If(IfStmt {
                    condition: condition,
                    then_branch: Box::from(then_branch),
                    else_branch: if let Some(else_branch) = else_branch {
                        Some(Box::from(else_branch))
                    } else {
                        None
                    },
                })
            )
        } else {
            Err(
                ViskumError::new("Expected '{' after an \"if\" expression", peeked_token, "file.vs")
            )
        }
    }

    pub(super) fn block_statement(&mut self) -> Result<Vec<Stmt>, ViskumError> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(&TokenType::RightBrace)? && !self.is_at_end()? {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expected '}' after block")?;

        Ok(statements)
    }
}
