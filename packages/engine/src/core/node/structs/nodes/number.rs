#[derive(PartialEq, Debug, Clone)]
pub struct Number {
    pub value: f64,
}

impl Number {
    pub fn new(value: f64) -> Number {
        Number { value }
    }
}
