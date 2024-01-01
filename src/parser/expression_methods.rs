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
    },
    error_handler::ViskumError,
    token::{ TokenType, Literal },
    stmt::{ Stmt, LetStmt },
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
        //  else if self.match_tokens(&[TokenType::Increment, TokenType::Decrement])? {
        //     if let Expr::Variable(var_expr) = expr.borrow() {
        //         return Ok(
        //             Expr::Assign(AssignExpr {
        //                 token: var_expr.token.clone(),

        //                 value: Box::from(
        //                     Expr::Postfix(PostfixExpr {
        //                         left: Box::from(expr),
        //                         operator: self.peek_previous()?,
        //                     })
        //                 ),
        //             })
        //         );
        //     } else {
        //         let equals = self.peek_previous()?;

        //         report_error(
        //             self.error_handler,
        //             ViskumError::new(
        //                 format!("Invalid assignment target at '{}'", equals.lexeme).as_str(),
        //                 equals,
        //                 "file.vs"
        //             )
        //         );
        //     }
        // }

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

        Ok(self.primary()?)
    }

    // fn postfix(&mut self) -> Result<Expr, ViskumError> {
    //     println!("{}", self.peek()?);
    //     println!("{}", self.peek_previous()?);
    //     if
    //         self.match_next_tokens(
    //             &[TokenType::Factorial, TokenType::Increment, TokenType::Decrement]
    //         )?
    //     {
    //         let operator = self.peek_next()?;
    //         let left = self.primary()?;

    //         self.advance()?;

    //         return Ok(Expr::Postfix(PostfixExpr { left: Box::from(left), operator: operator }));
    //     }

    //     Ok(self.primary()?)
    // }

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
