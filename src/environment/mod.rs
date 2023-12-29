pub mod environment_value;

use std::collections::HashMap;

use crate::{ token::{ Literal, Token }, error_handler::ViskumError };

use self::environment_value::EnvironmentValue;

pub struct Environment {
    values: HashMap<String, EnvironmentValue>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { values: HashMap::new(), enclosing: None }
    }

    pub fn get(&self, token: &Token) -> Result<Literal, ViskumError> {
        if let Some(v) = self.values.get(&token.lexeme) {
            Ok(v.get_value())
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
