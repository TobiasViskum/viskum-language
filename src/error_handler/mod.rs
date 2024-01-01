mod error;

pub use error::ViskumError;
pub use error::AbortReason;

pub struct ErrorHandler {
    errors: Vec<ViskumError>,
}

impl ErrorHandler {
    pub fn new() -> Self {
        ErrorHandler { errors: Vec::new() }
    }

    pub fn report_error(&mut self, error: ViskumError) {
        self.errors.push(error)
    }

    pub fn print_errors(&self) {
        for error in &self.errors {
            eprintln!("{}", error.to_string());
        }
    }

    pub fn has_error(&self) -> bool {
        self.errors.len() > 0
    }
}
