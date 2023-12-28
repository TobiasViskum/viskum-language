use std::{ rc::Rc, cell::RefCell };
use crate::error_handler::{ ErrorHandler, ViskumError };

pub fn is_digit(ch: Option<char>) -> bool {
    if let Some(ch) = ch { ch >= '0' && ch <= '9' } else { false }
}

pub fn is_alphabetic(ch: Option<char>) -> bool {
    if let Some(ch) = ch { ch.is_ascii_alphabetic() || ch == '_' } else { false }
}

pub fn report_error(error_handler: &Rc<RefCell<ErrorHandler>>, viskum_error: ViskumError) {
    error_handler.borrow_mut().report_error(viskum_error)
}
