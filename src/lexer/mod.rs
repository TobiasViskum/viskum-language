use std::cell::RefCell;
use std::rc::Rc;

mod lexer_util;
mod private_methods;

use crate::error_handler::ErrorHandler;
use crate::token::{ Token, TokenType };
use crate::error_handler::ViskumError;
use crate::util::{ is_alphabetic, report_error };

pub struct Lexer<'a> {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    line_position: usize,
    error_handler: &'a Rc<RefCell<ErrorHandler>>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: String, error_handler: &'a Rc<RefCell<ErrorHandler>>) -> Self {
        Lexer {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            line_position: 0,
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
            ')' => {
                self.add_token(TokenType::RightParen);
                if self.peek() == Some('!') {
                    self.start = self.current;
                    self.advance();
                    self.add_token(TokenType::Factorial)
                }
            }
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '^' => self.add_token(TokenType::Power),
            ':' => self.add_token(TokenType::Colon),
            '?' => self.add_token(TokenType::QuestionMark),
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
                } else if self.match_char('*') {
                    self.scan_comment()
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.increment_line(),
            '"' => self.string(),
            '0'..='9' => self.number(),
            _ => {
                if is_alphabetic(Some(c)) {
                    self.identifier();
                } else {
                    report_error(
                        self.error_handler,
                        ViskumError::new(
                            format!("Unrecognizable character: {}", c).as_str(),
                            Token::invalid(Some(self.line)),
                            "file.vs"
                        )
                    )
                }
            }
        }
    }
}
