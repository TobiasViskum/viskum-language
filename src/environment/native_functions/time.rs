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

// 0.000000992138147354126
// 0.0000012109593391418457
// 0.0000012109122276306153

// 0.000004377958822250366
// 0.000004366387033462524
