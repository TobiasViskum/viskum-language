use crate::{ token::{ Literal, TokenType, Token }, error_handler::ViskumError };

fn operation_error(op: &TokenType, left: &Literal, right: &Literal) -> ViskumError {
    ViskumError::new(
        format!(
            "'{}' is not defined for {} and {}",
            op.to_lexeme(),
            left.to_type_string(),
            right.to_type_string()
        ).as_str(),
        Token::new(op.clone(), op.to_lexeme(), None, 0),
        "file.vs"
    )
}

pub fn is_equal(left: &Literal, right: &Literal) -> bool {
    match (left, right) {
        (Literal::Null, Literal::Null) => {
            return true;
        }
        (Literal::Str(str1), Literal::Str(str2)) => {
            return str1 == str2;
        }
        (Literal::Num(x), Literal::Num(y)) => {
            return x == y;
        }
        (Literal::Bool(b1), Literal::Bool(b2)) => {
            return b1 == b2;
        }
        _ => false,
    }
}

type Output = Result<Literal, ViskumError>;

pub fn greater(left: &Literal, right: &Literal) -> Output {
    match (left, right) {
        (Literal::Num(x), Literal::Num(y)) => {
            return Ok(Literal::Bool(x > y));
        }
        (Literal::Str(str1), Literal::Str(str2)) => {
            return Ok(Literal::Bool(str1 > str2));
        }
        _ => {
            return Err(operation_error(&TokenType::Greater, left, right));
        }
    }
}

pub fn greater_equal(left: &Literal, right: &Literal) -> Output {
    match (left, right) {
        (Literal::Num(x), Literal::Num(y)) => {
            return Ok(Literal::Bool(x >= y));
        }
        (Literal::Str(str1), Literal::Str(str2)) => {
            return Ok(Literal::Bool(str1 >= str2));
        }
        _ => {
            return Err(operation_error(&TokenType::GreaterEqual, left, right));
        }
    }
}

pub fn less(left: &Literal, right: &Literal) -> Output {
    match (left, right) {
        (Literal::Num(x), Literal::Num(y)) => {
            return Ok(Literal::Bool(x < y));
        }
        (Literal::Str(str1), Literal::Str(str2)) => {
            return Ok(Literal::Bool(str1 < str2));
        }
        _ => {
            return Err(operation_error(&TokenType::Less, left, right));
        }
    }
}

pub fn less_equal(left: &Literal, right: &Literal) -> Output {
    match (left, right) {
        (Literal::Num(x), Literal::Num(y)) => {
            return Ok(Literal::Bool(x <= y));
        }
        (Literal::Str(str1), Literal::Str(str2)) => {
            return Ok(Literal::Bool(str1 <= str2));
        }
        _ => {
            return Err(operation_error(&TokenType::LessEqual, left, right));
        }
    }
}

pub fn plus(left: &Literal, right: &Literal) -> Output {
    match (left, right) {
        (Literal::Num(x), Literal::Num(y)) => {
            return Ok(Literal::Num(x + y));
        }
        (Literal::Str(str1), Literal::Str(str2)) => {
            return Ok(Literal::Str(format!("{}{}", str1, str2).to_string()));
        }
        (Literal::Num(x), Literal::Str(str)) => {
            return Ok(Literal::Str(format!("{}{}", str, x.to_string()).to_string()));
        }
        (Literal::Str(str), Literal::Num(x)) => {
            return Ok(Literal::Str(format!("{}{}", str, x.to_string()).to_string()));
        }
        _ => {
            return Err(operation_error(&TokenType::Plus, left, right));
        }
    }
}

pub fn minus(left: &Literal, right: &Literal) -> Output {
    match (left, right) {
        (Literal::Num(x), Literal::Num(y)) => {
            return Ok(Literal::Num(x - y));
        }
        (Literal::Str(str1), Literal::Str(str2)) => {
            return Ok(Literal::Str(str1.replace(str2, "")));
        }
        _ => {
            return Err(operation_error(&TokenType::Minus, left, right));
        }
    }
}

pub fn division(left: &Literal, right: &Literal) -> Output {
    match (left, right) {
        (Literal::Num(x), Literal::Num(y)) => {
            return Ok(Literal::Num(x / y));
        }
        _ => {
            return Err(operation_error(&TokenType::Slash, left, right));
        }
    }
}

pub fn multiplication(left: &Literal, right: &Literal) -> Output {
    match (left, right) {
        (Literal::Num(x), Literal::Num(y)) => {
            return Ok(Literal::Num(x * y));
        }
        _ => {
            return Err(operation_error(&TokenType::Star, left, right));
        }
    }
}

pub fn exponential(left: &Literal, right: &Literal) -> Output {
    match (left, right) {
        (Literal::Num(x), Literal::Num(y)) => {
            return Ok(Literal::Num(x.powf(*y)));
        }
        _ => {
            return Err(operation_error(&TokenType::Power, left, right));
        }
    }
}
