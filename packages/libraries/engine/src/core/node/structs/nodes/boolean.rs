#[derive(PartialEq, Debug, Clone)]
pub struct Boolean {
    pub value: bool,
}

impl Boolean {
    pub fn new(value: bool) -> Boolean {
        Boolean { value }
    }
}
