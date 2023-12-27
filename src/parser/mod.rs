use std::{ cell::RefCell, rc::Rc };

use crate::{ token::{ TokenType, Token }, error_handler::ErrorHandler, expr::{ Expr, BinaryExpr } };

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    error_handler: Rc<RefCell<ErrorHandler>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, error_handler: Rc<RefCell<ErrorHandler>>) -> Self {
        Parser { tokens: tokens, current: 0, error_handler: error_handler }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_tokens(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            if let Some(op) = self.previous() {
                let right = self.comparison();
                expr = Expr::Binary(BinaryExpr {
                    left: Box::from(expr),
                    operator: *op,
                    right: Box::from(right),
                });
            } else {
                break;
            }
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while
            self.match_tokens(
                vec![
                    TokenType::Greater,
                    TokenType::GreaterEqual,
                    TokenType::Less,
                    TokenType::LessEqual
                ]
            )
        {
            if let Some(op) = self.previous() {
                let right = self.term();
                expr = Expr::Binary(BinaryExpr {
                    left: Box::from(expr),
                    operator: *op,
                    right: Box::from(right),
                });
            } else {
                break;
            }
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_tokens(vec![TokenType::Minus, TokenType::Plus]) {
            if let Some(op) = self.previous() {
                let right = self.factor();
                expr = Expr::Binary(BinaryExpr {
                    left: Box::from(expr),
                    operator: *op,
                    right: Box::from(right),
                });
            } else {
                break;
            }
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_tokens(vec![TokenType::Slash, TokenType::Star]) {
            if let Some(op) = self.previous() {
                let right = self.unary();
                expr = Expr::Binary(BinaryExpr {
                    left: Box::from(expr),
                    operator: *op,
                    right: Box::from(right),
                });
            } else {
                break;
            }
        }

        expr
    }

    fn unary(&mut self) -> Expr {}

    fn match_tokens(&mut self, ttypes: Vec<TokenType>) -> bool {
        for ttype in ttypes {
            if self.check(ttype) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            if let Some(token) = self.peek() { token.ttype == ttype } else { false }
        }
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        if let Some(token) = self.peek() { token.ttype == TokenType::Eof } else { false }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }
}
