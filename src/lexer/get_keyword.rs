use crate::token::TokenType;

pub fn get_keyword(check: String) -> Option<TokenType> {
    match check.as_str() {
        "and" => Some(TokenType::And),
        "class" => Some(TokenType::Class),
        "else" => Some(TokenType::Else),
        "false" => Some(TokenType::False),
        "for" => Some(TokenType::For),
        "if" => Some(TokenType::If),
        "null" => Some(TokenType::Null),
        "or" => Some(TokenType::Or),
        "print" => Some(TokenType::Print),
        "return" => Some(TokenType::Return),
        "this" => Some(TokenType::This),
        "true" => Some(TokenType::True),
        "let" => Some(TokenType::Let),
        "mut" => Some(TokenType::Mut),
        "while" => Some(TokenType::While),
        "new" => Some(TokenType::New),
        "break" => Some(TokenType::Break),
        "continue" => Some(TokenType::Continue),
        "loop" => Some(TokenType::Loop),
        "fn" => Some(TokenType::Fn),
        _ => None,
    }
}
