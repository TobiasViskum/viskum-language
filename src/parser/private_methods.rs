use crate::{ token::{ TokenType, Token }, error_handler::ViskumError };

pub use super::Parser;

impl<'a> Parser<'a> {
    pub(super) fn consume(&mut self, ttype: TokenType, msg: &str) -> Result<(), ViskumError> {
        if self.check(&ttype)? {
            Ok(self.advance()?)
        } else {
            let token = self.peek()?;
            Err(ViskumError::new(msg, token, "file.vs"))
        }
    }

    pub(super) fn match_tokens(&mut self, ttypes: &[TokenType]) -> Result<bool, ViskumError> {
        for ttype in ttypes {
            if self.check(ttype)? {
                self.advance()?;
                return Ok(true);
            }
        }

        Ok(false)
    }
    pub(super) fn match_next_tokens(&mut self, ttypes: &[TokenType]) -> Result<bool, ViskumError> {
        if !self.is_at_end()? {
            for ttype in ttypes {
                if let Ok(bool) = self.check_next(ttype) {
                    return Ok(bool);
                }
            }
        }

        Ok(false)
    }

    pub(super) fn check(&self, ttype: &TokenType) -> Result<bool, ViskumError> {
        if self.is_at_end()? { Ok(false) } else { Ok(&self.peek()?.ttype == ttype) }
    }
    pub(super) fn check_next(&self, ttype: &TokenType) -> Result<bool, ViskumError> {
        if self.is_at_end()? { Ok(false) } else { Ok(&self.peek_next()?.ttype == ttype) }
    }

    pub(super) fn advance(&mut self) -> Result<(), ViskumError> {
        if !self.is_at_end()? {
            self.current += 1;
        }
        Ok(())
    }

    pub(super) fn is_at_end(&self) -> Result<bool, ViskumError> {
        Ok(self.peek()?.is(TokenType::Eof))
    }

    pub(super) fn peek(&self) -> Result<Token, ViskumError> {
        match self.tokens.get(self.current) {
            Some(token) => Ok(token.clone()),
            None =>
                Err(
                    ViskumError::new(
                        "Internal error: Failed to find token",
                        Token::invalid(None),
                        "file.vs"
                    )
                ),
        }
    }

    pub(super) fn peek_next(&self) -> Result<Token, ViskumError> {
        match self.tokens.get(self.current + 1) {
            Some(token) => Ok(token.clone()),
            None =>
                Err(
                    ViskumError::new(
                        "Internal error: Failed to find token",
                        Token::invalid(None),
                        "file.vs"
                    )
                ),
        }
    }

    pub(super) fn peek_previous(&self) -> Result<Token, ViskumError> {
        match self.tokens.get(self.current - 1) {
            Some(token) => Ok(token.clone()),
            None =>
                Err(
                    ViskumError::new(
                        "Internal error: Failed to find token",
                        Token::invalid(None),
                        "file.vs"
                    )
                ),
        }
    }

    pub(super) fn synchronize(&mut self) -> Result<(), ViskumError> {
        self.advance()?;

        while !self.is_at_end()? {
            if self.peek_previous()?.is(TokenType::Semicolon) {
                return Ok(());
            }
            if
                matches!(
                    self.peek()?.ttype,
                    TokenType::Class |
                        TokenType::Let |
                        TokenType::For |
                        TokenType::If |
                        TokenType::While |
                        TokenType::Print |
                        TokenType::Return
                )
            {
                return Ok(());
            }
            let _ = self.advance();
        }

        Ok(())
    }
}

impl<'a> Parser<'a> {}
