use std::{ rc::Rc, collections::HashMap };

use crate::{ token::Literal, viskum_callable::Callable };

use super::{ environment_value::EnvironmentValue, native_functions::time::NativeClock };

pub fn get_globals() -> HashMap<String, EnvironmentValue> {
    let mut globals: HashMap<String, EnvironmentValue> = HashMap::new();

    globals.insert(
        "time".to_string(),
        EnvironmentValue::new(
            Literal::Func(Callable { arity: 0, func: Rc::new(NativeClock {}) }),
            false
        )
    );

    globals.insert(
        "new_print".to_string(),
        EnvironmentValue::new(
            Literal::Func(Callable { arity: 0, func: Rc::new(NativeClock {}) }),
            false
        )
    );

    globals
}
