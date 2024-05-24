#[derive(PartialEq, Debug, Clone)]
pub struct Symbol {
    pub name: String,
}

impl Symbol {
    pub fn new<T: Into<String>>(name: T) -> Symbol {
        Symbol { name: name.into() }
    }
}
