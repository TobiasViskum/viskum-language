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

            self.consume(TokenType::Colon, "Expected ':' in ternary expression".to_string())?;

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
        let mut expr = self.post_pre_fix()?;

        while self.match_tokens(&[TokenType::Slash, TokenType::Star, TokenType::Power])? {
            let operator = self.peek_previous()?;
            let right = self.post_pre_fix()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::from(expr),
                operator: operator,
                right: Box::from(right),
            });
        }

        Ok(expr)
    }

    pub(super) fn post_pre_fix(&mut self) -> Result<Expr, ViskumError> {
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
            let right = self.post_pre_fix()?;
            return Ok(Expr::Prefix(PrefixExpr { operator: operator, right: Box::from(right) }));
        }

        Ok(self.primary()?)
    }

    pub(super) fn primary(&mut self) -> Result<Expr, ViskumError> {
        if self.match_tokens(&[TokenType::False])? {
            return Ok(Expr::Literal(LiteralExpr { value: Some(Literal::False) }));
        }
        if self.match_tokens(&[TokenType::True])? {
            return Ok(Expr::Literal(LiteralExpr { value: Some(Literal::True) }));
        }
        if self.match_tokens(&[TokenType::Null])? {
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
            let _ = self.consume(
                TokenType::RightParen,
                "Expected ')' after expression".to_string()
            )?;

            return Ok(Expr::Grouping(GroupingExpr { expression: Box::from(expr) }));
        }

        Err(
            ViskumError::new(
                "Expected expression".to_string(),
                self.peek()?.line,
                0,
                "file.vs".to_string()
            )
        )
    }
}
