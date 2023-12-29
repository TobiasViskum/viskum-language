use crate::token::Literal;

#[derive(Debug, Clone)]
pub struct EnvironmentValue {
    value: Literal,
    mutable: bool,
    // value_type: String
}

impl EnvironmentValue {
    pub fn new(value: Literal, mutable: bool) -> Self {
        EnvironmentValue { value, mutable }
    }

    pub fn get_value(&self) -> Literal {
        self.value.clone()
    }
}
