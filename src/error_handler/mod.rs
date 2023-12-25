mod error;

use error::ViskumError;

pub struct ErrorHandler<'a> {
    errors: Vec<&'a str>,
}

impl<'a> ErrorHandler<'a> {
    pub fn report_error(&self, error: ViskumError) {
        self.errors.push(error.to_string())
    }

    pub fn display_errors(&self) {
        for error in self.errors {
            eprintln!("{}", error);
        }
    }
}
