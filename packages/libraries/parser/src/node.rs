mod boolean;
mod number;

pub use boolean::*;
pub use number::*;

pub enum Node {
    Number(Number),
    Boolean(Boolean),
}
