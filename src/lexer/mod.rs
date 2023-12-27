use std::cell::RefCell;
use std::rc::Rc;

mod lexer_util;

use crate::error_handler::ErrorHandler;
use crate::token::{ Token, TokenType, Literal };
use crate::error_handler::ViskumError;
use crate::util::{ is_digit, is_alphabetic };

use self::lexer_util::get_keyword;

pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    error_handler: Rc<RefCell<ErrorHandler>>,
}

impl Lexer {
    pub fn new(source: String, error_handler: Rc<RefCell<ErrorHandler>>) -> Self {
        Lexer {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            error_handler: error_handler,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, ViskumError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::eof(self.line));

        Ok(&self.tokens)
    }

    fn scan_token(&mut self) {
        let c = match self.peek() {
            Some(c) => c,
            None => {
                return;
            }
        };
        self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.match_char('/') {
                    while let Some(ch) = self.peek() {
                        if ch == '\n' {
                            break;
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            '0'..='9' => self.number(),
            _ => {
                if is_alphabetic(Some(c)) {
                    self.identifier();
                } else {
                    self.report_error(
                        ViskumError::new(
                            format!("Unexpetected character: {}", c),
                            self.line,
                            self.start,
                            "file.vs".to_string()
                        )
                    )
                }
            }
        }
    }

    fn string(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == '"' {
                break;
            } else if ch == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.report_error(
                ViskumError::new(
                    "Unterminated string".to_string(),
                    self.line,
                    0,
                    "file.vs".to_string()
                )
            );
            return;
        }

        self.advance();

        let value: String = self.source[self.start + 1..self.current - 1].iter().collect();
        self.add_token_literal(TokenType::String, Some(Literal::Str(value)))
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == Some('.') && is_digit(self.peek_next()) {
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let value: String = self.source[self.start..self.current].iter().collect();
        let num: f64 = value.parse().unwrap();

        self.add_token_literal(TokenType::Number, Some(Literal::Num(num)))
    }

    fn identifier(&mut self) {
        while let Some(ch) = self.peek() {
            if is_alphabetic(Some(ch)) || is_digit(Some(ch)) {
                self.advance();
            }
        }

        let text: String = self.source[self.start..self.current].iter().collect();

        if let Some(ttype) = get_keyword(text) {
            self.add_token(ttype);
        } else {
            self.add_token(TokenType::Identifier);
        }

        self.add_token(TokenType::Identifier)
    }

    fn report_error(&self, viskum_error: ViskumError) {
        (*self.error_handler).borrow_mut().report_error(viskum_error)
    }
}

include!("./private_methods.rs");
// include!("./private_char_match_methods.rs");
