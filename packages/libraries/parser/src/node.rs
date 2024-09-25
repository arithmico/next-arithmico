mod boolean;
mod division;
mod negate;
mod number;
mod power;
mod product;
mod sum;
mod symbol;
mod tensor;

pub use boolean::*;
pub use division::*;
pub use negate::*;
use nom::IResult;
pub use number::*;
pub use power::*;
pub use product::*;
pub use sum::*;
pub use symbol::*;
pub use tensor::*;

#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Boolean(Boolean),
    Sum(Sum),
    Negate(Negate),
    Product(Product),
    Division(Division),
    Power(Power),
    Tensor(Tensor),
    Number(Number),
    Symbol(Symbol),
}

impl Node {
    pub fn parse(input: &str) -> IResult<&str, Node> {
        Sum::parse(input)
    }
}
