use std::borrow::Borrow;

use crate::{
    expr::{
        Expr,
        BinaryExpr,
        PrefixExpr,
        LiteralExpr,
        GroupingExpr,
        PostfixExpr,
        TernaryExpr,
        VariableExpr,
        AssignExpr,
        LogicalExpr,
        CallExpr,
    },
    error_handler::ViskumError,
    token::{ TokenType, Literal, Token },
    stmt::{ Stmt, LetStmt, FunctionStmt },
    util::report_error,
};

use super::Parser;

impl<'a> Parser<'a> {
    pub(super) fn variable_declaration(&mut self) -> Result<Stmt, ViskumError> {
        let token = self.consume_and_get(TokenType::Identifier, "Expected variable name")?;

        let initializer = if self.match_tokens(&[TokenType::Equal])? {
            self.expression()?
        } else {
            Expr::Literal(LiteralExpr { value: Some(Literal::Null) })
        };

        self.consume(TokenType::Semicolon, "Expected ';' after variable declaration")?;

        Ok(Stmt::Let(LetStmt { token: token, initializer: initializer }))
    }

    pub(super) fn function_declaration(&mut self, kind: String) -> Result<Stmt, ViskumError> {
        let function_token = self.consume_and_get(
            TokenType::Identifier,
            format!("Expected {} name", kind).as_str()
        )?;
        self.consume(TokenType::LeftParen, format!("Expected '(' after {} name", kind).as_str())?;

        let mut params: Vec<Token> = Vec::new();

        if !self.check(&TokenType::RightParen)? {
            loop {
                if params.len() >= 255 {
                    report_error(
                        self.error_handler,
                        ViskumError::new(
                            "Cannot have more than 255 parameters",
                            self.peek()?,
                            "file.vs"
                        )
                    );
                }

                params.push(
                    self.consume_and_get(TokenType::Identifier, "Expected parameter name")?
                );

                if !self.match_tokens(&[TokenType::Comma])? {
                    break;
                }
            }
        }
        self.consume(
            TokenType::RightParen,
            format!("Expected ')' after {} parameters", kind).as_str()
        )?;
        self.consume(TokenType::LeftBrace, format!("Expected '{{' before {} body", kind).as_str())?;

        let body = self.block()?;

        Ok(
            Stmt::Function(FunctionStmt {
                token: function_token,
                params,
                body,
            })
        )
    }

    pub(super) fn expression(&mut self) -> Result<Expr, ViskumError> {
        self.logical()
    }

    pub(super) fn assignment(&mut self) -> Result<Expr, ViskumError> {
        let expr = self.expression()?;

        if
            self.match_tokens(
                &[
                    TokenType::Equal,
                    TokenType::PlusEqual,
                    TokenType::MinusEqual,
                    TokenType::StarEqual,
                    TokenType::SlashEqual,
                    TokenType::PowerEqual,
                    TokenType::Increment,
                    TokenType::Decrement,
                ]
            )?
        {
            let assignment_token = self.peek_previous()?;
            if let Expr::Variable(var_expr) = expr.borrow() {
                match self.peek_previous()?.ttype {
                    TokenType::Increment | TokenType::Decrement => {
                        return Ok(
                            Expr::Assign(AssignExpr {
                                token: var_expr.token.clone(),
                                assignment_token: assignment_token,
                                value: Box::from(expr),
                            })
                        );
                    }
                    _ => {
                        let value = self.expression()?;

                        return Ok(
                            Expr::Assign(AssignExpr {
                                token: var_expr.token.clone(),
                                assignment_token: assignment_token,
                                value: Box::from(value),
                            })
                        );
                    }
                }
            } else {
                report_error(
                    self.error_handler,
                    ViskumError::new(
                        format!(
                            "Invalid assignment target at '{}'",
                            assignment_token.lexeme
                        ).as_str(),
                        assignment_token,
                        "file.vs"
                    )
                );
            }
        }

        Ok(expr)
    }

    fn logical(&mut self) -> Result<Expr, ViskumError> {
        let lhs = self.ternary()?;

        if self.match_tokens(&[TokenType::Or, TokenType::And])? {
            let operator = self.peek_previous()?;
            let rhs = self.expression()?;

            Ok(
                Expr::Logical(LogicalExpr {
                    left: Box::from(lhs),
                    operator: operator,
                    right: Box::from(rhs),
                })
            )
        } else {
            Ok(lhs)
        }
    }

    fn ternary(&mut self) -> Result<Expr, ViskumError> {
        let condition_expr = self.equality()?;

        if self.match_tokens(&[TokenType::QuestionMark])? {
            let true_expr = self.expression()?;

            self.consume(TokenType::Colon, "Expected ':' in ternary expression")?;

            let false_expr = self.expression()?;

            return Ok(
                Expr::Ternary(TernaryExpr {
                    condition: Box::from(condition_expr),
                    true_expr: Box::from(true_expr),
                    false_expr: Box::from(false_expr),
                })
            );
        }

        Ok(condition_expr)
    }

    fn equality(&mut self) -> Result<Expr, ViskumError> {
        let mut expr = self.comparison()?;

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual])? {
            let operator = self.peek_previous()?;
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::from(expr),
                operator: operator,
                right: Box::from(right),
            });
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ViskumError> {
        let mut expr = self.term()?;

        while
            self.match_tokens(
                &[
                    TokenType::Greater,
                    TokenType::GreaterEqual,
                    TokenType::Less,
                    TokenType::LessEqual,
                ]
            )?
        {
            let operator = self.peek_previous()?;

            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::from(expr),
                operator: operator,
                right: Box::from(right),
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ViskumError> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus])? {
            let operator = self.peek_previous()?;
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::from(expr),
                operator: operator,
                right: Box::from(right),
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ViskumError> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[TokenType::Slash, TokenType::Star, TokenType::Power])? {
            let operator = self.peek_previous()?;
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::from(expr),
                operator: operator,
                right: Box::from(right),
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ViskumError> {
        // Postfix e.g. 5!
        if self.match_next_tokens(&[TokenType::Factorial])? {
            let operator = self.peek_next()?;
            let left = self.primary()?;

            self.advance()?;

            return Ok(Expr::Postfix(PostfixExpr { left: Box::from(left), operator: operator }));
        }

        // Prefix e.g. !5
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus])? {
            let operator = self.peek_previous()?;
            let right = self.unary()?;
            return Ok(Expr::Prefix(PrefixExpr { operator: operator, right: Box::from(right) }));
        }

        Ok(self.call()?)
    }

    fn call(&mut self) -> Result<Expr, ViskumError> {
        let mut expr = self.primary()?;

        loop {
            if self.match_tokens(&[TokenType::LeftParen])? {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, calle: Expr) -> Result<Expr, ViskumError> {
        let mut arguments: Vec<Expr> = Vec::new();

        if !self.check(&TokenType::RightParen)? {
            arguments.push(self.expression()?);
            while self.match_tokens(&[TokenType::Comma])? {
                if arguments.len() >= 255 {
                    report_error(
                        self.error_handler,
                        ViskumError::new(
                            "Cannot have more than 255 arguments",
                            self.peek_previous()?,
                            "file.vs"
                        )
                    );
                }
                arguments.push(self.expression()?);
            }
        }

        let paren = self.consume_and_get(
            TokenType::RightParen,
            "Expected ')' after function arguments"
        )?;

        Ok(
            Expr::Call(CallExpr {
                callee: Box::from(calle),
                paren: paren,
                arguments: arguments,
            })
        )
    }

    fn primary(&mut self) -> Result<Expr, ViskumError> {
        if self.match_tokens(&[TokenType::False])? {
            return Ok(Expr::Literal(LiteralExpr { value: Some(Literal::Bool(false)) }));
        }
        if self.match_tokens(&[TokenType::True])? {
            return Ok(Expr::Literal(LiteralExpr { value: Some(Literal::Bool(true)) }));
        }
        if self.match_tokens(&[TokenType::Null])? {
            return Ok(Expr::Literal(LiteralExpr { value: Some(Literal::Null) }));
        }

        if self.match_tokens(&[TokenType::Number, TokenType::String])? {
            return Ok(
                Expr::Literal(LiteralExpr {
                    value: self.peek_previous()?.literal.clone(),
                })
            );
        }

        if self.match_tokens(&[TokenType::Identifier])? {
            return Ok(Expr::Variable(VariableExpr { token: self.peek_previous()? }));
        }

        if self.match_tokens(&[TokenType::LeftParen])? {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression")?;

            if self.match_tokens(&[TokenType::Factorial])? {
                let operator = self.peek_previous()?;

                return Ok(
                    Expr::Postfix(PostfixExpr {
                        left: Box::from(
                            Expr::Grouping(GroupingExpr { expression: Box::from(expr) })
                        ),
                        operator: operator,
                    })
                );
            }

            return Ok(Expr::Grouping(GroupingExpr { expression: Box::from(expr) }));
        }

        Err(
            ViskumError::new(
                format!("Expected expression: Unexpected '{}'", self.peek()?.lexeme).as_str(),
                self.peek()?,
                "file.vs"
            )
        )
    }
}
