pub fn is_digit(ch: Option<char>) -> bool {
    if let Some(ch) = ch { ch >= '0' && ch <= '9' } else { false }
}

pub fn is_alphabetic(ch: Option<char>) -> bool {
    if let Some(ch) = ch { ch.is_ascii_alphabetic() || ch == '_' } else { false }
}
