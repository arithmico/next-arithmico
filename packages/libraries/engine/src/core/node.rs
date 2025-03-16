mod and;
mod boolean;
mod definition;
mod division;
mod equals;
mod function;
mod function_call;
mod greater_than;
mod greater_than_or_equals;
mod host_function;
mod less_than;
mod less_than_or_equals;
mod negate;
mod number;
mod or;
mod power;
mod product;
mod sum;
mod symbol;
mod tensor;

mod evaluate;
mod for_each;
mod parse;
mod serialize;
mod trace;

pub use and::*;
pub use boolean::*;
pub use definition::*;
pub use division::*;
pub use equals::*;
pub use function::*;
pub use function_call::*;
pub use greater_than::*;
pub use greater_than_or_equals::*;
pub use host_function::*;
pub use less_than::*;
pub use less_than_or_equals::*;
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
    Equals(Equals),
    LessThan(LessThan),
    LessThanOrEquals(LessThanOrEquals),
    GreaterThan(GreaterThan),
    GreaterThanOrEquals(GreaterThanOrEquals),
    HostFunction(HostFunction),
    Definition(Definition),
}
