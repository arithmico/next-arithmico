mod boolean;
mod number;

pub use boolean::*;
pub use number::*;

#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Number(Number),
    Boolean(Boolean),
}
