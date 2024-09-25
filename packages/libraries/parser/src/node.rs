mod boolean;
mod division;
mod negate;
mod number;
mod power;
mod product;
mod sum;

pub use boolean::*;
pub use division::*;
pub use negate::*;
pub use number::*;
pub use power::*;
pub use product::*;
pub use sum::*;

#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Number(Number),
    Boolean(Boolean),
    Sum(Sum),
    Negate(Negate),
    Product(Product),
    Division(Division),
    Power(Power),
}
