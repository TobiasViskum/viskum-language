use crate::{
    expr::{ Expr, BinaryExpr, PrefixExpr, LiteralExpr, GroupingExpr, PostfixExpr, TernaryExpr },
    error_handler::ViskumError,
    token::{ TokenType, Literal },
};

use super::Parser;

impl<'a> Parser<'a> {
    pub(super) fn expression(&mut self) -> Result<Expr, ViskumError> {
        self.ternary()
    }

    pub(super) fn ternary(&mut self) -> Result<Expr, ViskumError> {
        let condition_expr = self.equality()?;

        if self.match_tokens(&[TokenType::QuestionMark])? {
            let true_expr = self.equality()?;

            self.consume(TokenType::Colon, "Expected ':' in ternary expression")?;

            let false_expr = self.equality()?;

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

    pub(super) fn equality(&mut self) -> Result<Expr, ViskumError> {
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

    pub(super) fn comparison(&mut self) -> Result<Expr, ViskumError> {
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

    pub(super) fn term(&mut self) -> Result<Expr, ViskumError> {
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

    pub(super) fn factor(&mut self) -> Result<Expr, ViskumError> {
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

    pub(super) fn unary(&mut self) -> Result<Expr, ViskumError> {
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

    pub(super) fn primary(&mut self) -> Result<Expr, ViskumError> {
        if self.match_tokens(&[TokenType::False])? {
            self.advance()?;
            return Ok(Expr::Literal(LiteralExpr { value: Some(Literal::Bool(false)) }));
        }
        if self.match_tokens(&[TokenType::True])? {
            self.advance()?;
            return Ok(Expr::Literal(LiteralExpr { value: Some(Literal::Bool(true)) }));
        }
        if self.match_tokens(&[TokenType::Null])? {
            self.advance()?;
            return Ok(Expr::Literal(LiteralExpr { value: Some(Literal::Null) }));
        }
        if self.match_tokens(&[TokenType::String])? {
            return Ok(
                Expr::Literal(LiteralExpr {
                    value: Some(Literal::Str(self.peek_previous()?.literal.unwrap().to_string())),
                })
            );
        }
        if self.match_tokens(&[TokenType::Number])? {
            return Ok(
                Expr::Literal(LiteralExpr {
                    value: Some(
                        Literal::Num(
                            self.peek_previous()?.literal.unwrap().to_string().parse().unwrap()
                        )
                    ),
                })
            );
        }

        if self.match_tokens(&[TokenType::LeftParen])? {
            let expr = self.expression()?;
            let _ = self.consume(TokenType::RightParen, "Expected ')' after expression")?;

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

        Err(ViskumError::new("Expected expression", self.peek()?, "file.vs"))
    }
}
