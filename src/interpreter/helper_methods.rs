use crate::{
    environment::environment_value::EnvironmentValue,
    token::{ Literal, Token },
    error_handler::ViskumError,
};

use super::Interpreter;

impl<'a> Interpreter<'a> {
    pub fn environment_get(&self, token: &Token) -> Result<Literal, ViskumError> {
        (*self.environment).borrow().get(token)
    }

    pub fn environment_define(
        &self,
        token: &Token,
        environment_value: EnvironmentValue
    ) -> Result<Literal, ViskumError> {
        (*self.environment).borrow_mut().define(token, environment_value)
    }

    pub fn environment_assign(
        &self,
        token: &Token,
        environment_value: EnvironmentValue
    ) -> Result<Literal, ViskumError> {
        (*self.environment).borrow_mut().assign(token, environment_value)
    }
}
