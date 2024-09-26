mod and;
mod boolean;
mod division;
mod function;
mod function_call;
mod negate;
mod number;
mod or;
mod power;
mod product;
mod sum;
mod symbol;
mod tensor;

pub use and::*;
pub use boolean::*;
pub use division::*;
pub use function::*;
pub use function_call::*;
pub use negate::*;
pub use number::*;
pub use or::*;
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
    Function(Function),
    FunctionCall(FunctionCall),
    And(And),
    Or(Or),
}
