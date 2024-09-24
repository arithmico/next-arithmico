mod boolean;
mod number;
mod product;
mod sum;

pub use boolean::*;
pub use number::*;
pub use product::*;
pub use sum::*;

#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Number(Number),
    Boolean(Boolean),
    Sum(Sum),
    Product(Product),
}
