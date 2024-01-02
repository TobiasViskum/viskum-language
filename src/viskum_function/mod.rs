use crate::{
    viskum_callable::ViskumCallable,
    stmt::FunctionStmt,
    interpreter::Interpreter,
    token::Literal,
    error_handler::ViskumError,
    environment::{ Environment, environment_value::EnvironmentValue },
};

#[derive(Clone)]
pub struct ViskumFunction {
    declaration: FunctionStmt,
}

impl ViskumFunction {
    pub fn new(declaration: FunctionStmt) -> Self {
        ViskumFunction { declaration: declaration }
    }
}

impl ViskumCallable for ViskumFunction {
    fn call(&self, interpreter: &Interpreter, args: &Vec<Literal>) -> Result<Literal, ViskumError> {
        let mut environment = Environment::new();

        for (i, param) in self.declaration.params.iter().enumerate() {
            environment.define(param, EnvironmentValue::new(args[i].clone(), false))?;
        }

        interpreter.execute_block(&self.declaration.body, environment)?;

        Ok(Literal::Null)
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn to_string(&self) -> String {
        format!("<fn {}>", self.declaration.token.lexeme)
    }
}
