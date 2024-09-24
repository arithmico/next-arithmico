mod boolean;
mod number;
mod sum;

pub use boolean::*;
pub use number::*;
pub use sum::*;

#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Number(Number),
    Boolean(Boolean),
    Sum(Sum),
}
