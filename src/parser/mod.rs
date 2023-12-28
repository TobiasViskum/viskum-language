use std::{ cell::RefCell, rc::Rc };

mod private_methods;

use crate::{
    token::{ TokenType, Token, Literal },
    error_handler::{ ErrorHandler, ViskumError },
    expr::{ Expr, BinaryExpr, UnaryExpr, LiteralExpr, GroupingExpr },
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    error_handler: Rc<RefCell<ErrorHandler>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, error_handler: Rc<RefCell<ErrorHandler>>) -> Self {
        Parser { tokens: tokens, current: 0, error_handler: error_handler }
    }

    pub fn expression(&self) -> Result<Expr, ViskumError> {
        self.equality()
    }

    fn equality(&self) -> Result<Expr, ViskumError> {
        let mut expr = self.comparison()?;

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual])? {
            let operator = self.previous()?;
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::from(expr),
                operator: *operator,
                right: Box::from(right),
            });
        }
        Ok(expr)
    }

    fn comparison(&self) -> Result<Expr, ViskumError> {
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
            let operator = self.previous()?;

            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::from(expr),
                operator: *operator,
                right: Box::from(right),
            });
        }

        Ok(expr)
    }

    fn term(&self) -> Result<Expr, ViskumError> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus])? {
            let operator = self.previous()?;
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::from(expr),
                operator: *operator,
                right: Box::from(right),
            });
        }

        Ok(expr)
    }

    fn factor(&self) -> Result<Expr, ViskumError> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[TokenType::Slash, TokenType::Star])? {
            let operator = self.previous()?;
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::from(expr),
                operator: *operator,
                right: Box::from(right),
            });
        }

        Ok(expr)
    }

    fn unary(&self) -> Result<Expr, ViskumError> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus])? {
            let operator = self.previous()?;
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr { operator: *operator, right: Box::from(right) }));
        }
        // self.match_next_tokens(&[TokenType::Bang]) (unaryop)

        Ok(self.primary()?)
    }

    fn primary(&mut self) -> Result<Expr, ViskumError> {
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
                    value: Some(Literal::Str(self.previous()?.literal.unwrap().to_string())),
                })
            );
        }
        if self.match_tokens(&[TokenType::Number])? {
            return Ok(
                Expr::Literal(LiteralExpr {
                    value: Some(
                        Literal::Num(self.previous()?.literal.unwrap().to_string().parse().unwrap())
                    ),
                })
            );
        }

        if self.match_tokens(&[TokenType::LeftParen])? {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression".to_string());
            return Ok(Expr::Grouping(GroupingExpr { expression: Box::from(expr) }));
        }

        Err(
            ViskumError::new(
                "Failed primary parsing".to_string(),
                self.peek()?.line,
                0,
                "file.vs".to_string()
            )
        )
    }
}
