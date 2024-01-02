use std::rc::Rc;

use crate::{ interpreter::Interpreter, token::Literal, error_handler::ViskumError };

#[derive(Clone)]
pub struct Callable {
    pub func: Rc<dyn ViskumCallable>,
    pub arity: usize,
}

pub trait ViskumCallable {
    fn call(
        &self,
        interpreter: &Interpreter,
        arguments: &Vec<Literal>
    ) -> Result<Literal, ViskumError>;
    fn arity(&self) -> usize;

    fn to_string(&self) -> String {
        "<native fn>".to_string()
    }
}

impl ViskumCallable for Callable {
    fn call(
        &self,
        interpreter: &Interpreter,
        arguments: &Vec<Literal>
    ) -> Result<Literal, ViskumError> {
        self.func.call(interpreter, arguments)
    }

    fn arity(&self) -> usize {
        self.arity
    }

    fn to_string(&self) -> String {
        self.func.to_string()
    }
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        self.arity == other.arity
    }
}

impl std::fmt::Debug for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<callable>")
    }
}
