use std::{ cell::RefCell, rc::Rc };

use crate::{
    interpreter::Interpreter,
    environment::Environment,
    stmt::Stmt,
    error_handler::ViskumError,
};

impl<'a> Interpreter<'a> {
    pub fn execute_block(
        &self,
        statements: &[Stmt],
        environment: Rc<RefCell<Environment>>
    ) -> Result<(), ViskumError> {
        let previous = self.environment.replace(environment);

        let result = statements.iter().try_for_each(|stmt| self.execute(stmt));

        self.environment.replace(previous);

        result
    }
}
