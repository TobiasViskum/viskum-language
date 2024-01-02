pub mod environment_value;
mod globals;
mod native_functions;

use std::{ collections::HashMap, cell::RefCell, rc::Rc };

use crate::{ token::{ Literal, Token }, error_handler::ViskumError };

use self::environment_value::EnvironmentValue;

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, EnvironmentValue>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        let globals = globals::get_globals();

        Environment { values: globals, enclosing: None }
    }

    pub fn new_with_enclosing(environment: Rc<RefCell<Environment>>) -> Self {
        Environment { values: HashMap::new(), enclosing: Some(environment) }
    }

    pub fn get(&self, token: &Token) -> Result<Literal, ViskumError> {
        if let Some(literal) = self.values.get(&token.lexeme) {
            Ok(literal.get_value())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow().get(token)
        } else {
            Err(
                ViskumError::new(
                    format!("Undefined variable '{}'", token.lexeme).as_str(),
                    token.clone(),
                    "file.vs"
                )
            )
        }
    }

    pub fn define_function(&mut self, function_name: &str, viskum_callable: Literal) {
        self.values.insert(
            function_name.to_string(),
            EnvironmentValue::new(viskum_callable, false)
        );
    }

    pub fn define(
        &mut self,
        token: &Token,
        environment_value: EnvironmentValue
    ) -> Result<Literal, ViskumError> {
        self.values.insert(token.lexeme.clone(), environment_value.clone());

        Ok(environment_value.get_value())
    }

    pub fn assign(
        &mut self,
        token: &Token,
        environment_value: EnvironmentValue
    ) -> Result<Literal, ViskumError> {
        if self.values.contains_key(&token.lexeme) {
            self.define(token, environment_value)
        } else if let Some(enclosing) = &self.enclosing {
            Ok(enclosing.borrow_mut().assign(token, environment_value)?)
        } else {
            Err(
                ViskumError::new(
                    format!("Undefined variable '{}'", token.lexeme).as_str(),
                    token.clone(),
                    "file.vs"
                )
            )
        }
    }
}
