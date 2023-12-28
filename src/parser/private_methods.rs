use crate::{ token::{ TokenType, Token }, error_handler::ViskumError };

pub use super::Parser;

impl Parser {
    pub(super) fn consume(&mut self, ttype: TokenType, msg: String) -> Result<(), ViskumError> {
        if self.check(&ttype)? {
            Ok(self.advance()?)
        } else {
            Err(
                ViskumError::new(
                    format!("Internal error: Failed to consume token: {:?}", ttype),
                    0,
                    0,
                    "file.vs".to_string()
                )
            )
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

    pub(super) fn check(&self, ttype: &TokenType) -> Result<bool, ViskumError> {
        if self.is_at_end()? { Ok(false) } else { Ok(&self.peek()?.ttype == ttype) }
    }

    pub(super) fn advance(&mut self) -> Result<(), ViskumError> {
        if !self.is_at_end()? {
            self.current += 1;
        }
        Ok(())
    }

    pub(super) fn is_at_end(&self) -> Result<bool, ViskumError> {
        Ok(self.peek()?.ttype == TokenType::Eof)
    }

    pub(super) fn peek(&self) -> Result<&Token, ViskumError> {
        match self.tokens.get(self.current) {
            Some(token) => Ok(token),
            None =>
                Err(
                    ViskumError::new(
                        "Internal error: Failed to find token".to_string(),
                        0,
                        0,
                        "file.vs".to_string()
                    )
                ),
        }
    }

    pub(super) fn previous(&self) -> Result<&Token, ViskumError> {
        match self.tokens.get(self.current - 1) {
            Some(token) => Ok(token),
            None =>
                Err(
                    ViskumError::new(
                        "Internal error: Failed to find token".to_string(),
                        0,
                        0,
                        "file.vs".to_string()
                    )
                ),
        }
    }
}
