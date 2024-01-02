use crate::{
    viskum_callable::ViskumCallable,
    token::Literal,
    error_handler::ViskumError,
    interpreter::Interpreter,
};

pub struct NativeClock;

impl ViskumCallable for NativeClock {
    fn call(
        &self,
        _interpreter: &Interpreter,
        _arguments: &Vec<Literal>
    ) -> Result<Literal, ViskumError> {
        Ok(
            Literal::Num(
                std::time::SystemTime
                    ::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64()
            )
        )
    }

    fn arity(&self) -> usize {
        0
    }
}
