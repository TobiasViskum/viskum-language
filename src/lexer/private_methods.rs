use crate::{
    token::{ TokenType, Token, Literal },
    util::{ is_alphabetic, is_digit },
    error_handler::ViskumError,
};

use super::{ Lexer, lexer_util::get_keyword };

impl Lexer {
    pub(super) fn add_token(&mut self, ttype: TokenType) {
        self.add_token_literal(ttype, None)
    }

    pub(super) fn add_token_literal(&mut self, ttype: TokenType, literal: Option<Literal>) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(ttype, lexeme, literal, self.line))
    }

    pub(super) fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub(super) fn advance(&mut self) {
        if !self.is_at_end() {
            self.line_position += 1;
            self.current += 1;
        }
    }

    pub(super) fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }

    pub(super) fn peek_next(&self) -> Option<char> {
        self.source.get(self.current + 1).copied()
    }

    pub(super) fn increment_line(&mut self) {
        self.line += 1;
        self.line_position = 0;
    }

    pub(super) fn match_char(&mut self, expected: char) -> bool {
        match self.source.get(self.current) {
            Some(c) if *c == expected => {
                self.advance();
                true
            }
            _ => false,
        }
    }

    pub(super) fn scan_comment(&mut self) {
        loop {
            match self.peek() {
                Some('*') => {
                    self.advance();
                    if self.match_char('/') {
                        break;
                    }
                }
                Some('/') => {
                    self.advance();
                    if self.match_char('*') {
                        self.scan_comment();
                    }
                }
                Some('\n') => {
                    self.advance();
                    self.increment_line();
                }
                None => {
                    self.report_error(
                        ViskumError::new(
                            "Expected '*/'".to_string(),
                            self.line_position,
                            self.line_position,
                            "file.vs".to_string()
                        )
                    );
                    break;
                }
                _ => self.advance(),
            }
        }
    }

    pub(super) fn string(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == '"' {
                break;
            } else if ch == '\n' {
                self.increment_line();
            }
            self.advance();
        }

        if self.is_at_end() {
            self.report_error(
                ViskumError::new(
                    "Unterminated string".to_string(),
                    self.line,
                    self.line_position,
                    "file.vs".to_string()
                )
            );
            return;
        }

        self.advance();

        let value: String = self.source[self.start + 1..self.current - 1].iter().collect();
        self.add_token_literal(TokenType::String, Some(Literal::Str(value)))
    }

    pub(super) fn number(&mut self) {
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

    pub(super) fn identifier(&mut self) {
        while let Some(ch) = self.peek() {
            if is_alphabetic(Some(ch)) || is_digit(Some(ch)) {
                self.advance();
            } else {
                break;
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

    pub(super) fn report_error(&self, viskum_error: ViskumError) {
        (*self.error_handler).borrow_mut().report_error(viskum_error)
    }
}
