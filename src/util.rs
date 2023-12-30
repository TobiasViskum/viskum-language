use std::{ rc::Rc, cell::RefCell };
use statrs::function::gamma::gamma;
use crate::error_handler::{ ErrorHandler, ViskumError };

pub fn is_digit(ch: Option<char>) -> bool {
    if let Some(ch) = ch { ch >= '0' && ch <= '9' } else { false }
}

pub fn is_alphabetic(ch: Option<char>) -> bool {
    if let Some(ch) = ch { ch.is_ascii_alphabetic() || ch == '_' } else { false }
}

pub fn report_error(error_handler: &RefCell<ErrorHandler>, viskum_error: ViskumError) {
    error_handler.borrow_mut().report_error(viskum_error)
}

pub fn factorial(n: f64) -> f64 {
    if n < 0.0 {
        std::f64::INFINITY
    } else if n.fract() != 0.0 {
        gamma(n + 1.0)
    } else if n == 0.0 {
        1.0
    } else {
        let num = n.round() as u128;
        let factorial: u128 = (1..=num).product();
        factorial as f64
    }
}
