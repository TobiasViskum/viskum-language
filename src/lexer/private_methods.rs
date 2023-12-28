use crate::token::{ TokenType, Token, Literal };

use super::Lexer;

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
}
