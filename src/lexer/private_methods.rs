impl Lexer {
    fn add_token(&mut self, ttype: TokenType) {
        self.add_token_literal(ttype, None)
    }

    fn add_token_literal(&mut self, ttype: TokenType, literal: Option<Literal>) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(ttype, lexeme, literal, self.line))
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.line_position += 1;
            self.current += 1;
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.source.get(self.current + 1).copied()
    }

    fn increment_line(&mut self) {
        self.line += 1;
        self.line_position = 0;
    }

    fn match_char(&mut self, expected: char) -> bool {
        match self.source.get(self.current) {
            Some(c) if *c == expected => {
                self.advance();
                true
            }
            _ => false,
        }
    }
}
