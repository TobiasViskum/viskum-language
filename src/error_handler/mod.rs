mod error;

use error::Error;

pub struct ErrorHandler<'a> {
    errors: Vec<&'a str>,
}

impl<'a> ErrorHandler<'a> {
    pub fn report_error(&self, error: Error) {
        self.errors.push(error.to_string())
    }

    pub fn display_errors(&self) {
        for error in self.errors {
            println!("{}", error);
        }
    }
}
