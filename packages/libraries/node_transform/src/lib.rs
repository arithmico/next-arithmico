use node::{
    And, Definition, Division, Equals, Factorial, Function, FunctionCall,
    GreaterThan, GreaterThanOrEquals, IntoNode, LessThan, LessThanOrEquals,
    Negate, Node, Or, Power, Product, Sum, Tensor,
};
use trace::{Tracable, TracableMut};

trait Sealed {}

impl Sealed for Node {}

#[allow(private_bounds)]
pub trait TransformNode: Sealed {
    fn transform<E>(
        self,
        transformer: impl Fn(Node) -> Result<Node, E>,
    ) -> Result<Node, E>;
}

impl TransformNode for Node {
    fn transform<E>(
        self,
        transformer: impl Fn(Node) -> Result<Node, E>,
    ) -> Result<Node, E> {
        let trace = self.trace().clone();

        let mut node = match self {
            Node::Boolean(node) => transformer(Node::Boolean(node))?,
            Node::Number(node) => transformer(Node::Number(node))?,
            Node::Symbol(node) => transformer(Node::Symbol(node))?,
            Node::HostFunction(node) => transformer(Node::HostFunction(node))?,

            Node::Sum(node) => {
                let elements = node
                    .elements
                    .into_iter()
                    .map(|node| transformer(node))
                    .collect::<Result<Vec<_>, E>>()?;
                transformer(Sum::new(elements))?
            }
            Node::Negate(node) => {
                let value = transformer(*node.value)?;
                transformer(Negate::new(value))?
            }
            Node::Product(node) => {
                let elements = node
                    .elements
                    .into_iter()
                    .map(|node| transformer(node))
                    .collect::<Result<Vec<_>, E>>()?;

                transformer(Product::new(elements))?
            }
            Node::Division(node) => {
                let dividend = transformer(*node.dividend)?;
                let divisor = transformer(*node.divisor)?;
                transformer(Division::new(dividend, divisor))?
            }
            Node::Power(node) => {
                let base = transformer(*node.base)?;
                let exponent = transformer(*node.exponent)?;
                transformer(Power::new(base, exponent))?
            }
            Node::Tensor(node) => {
                let elements = node
                    .elements
                    .into_iter()
                    .map(|node| transformer(node))
                    .collect::<Result<Vec<_>, E>>()?;
                transformer(
                    Tensor::new_with_shape(node.shape, elements).into_node(),
                )?
            }
            Node::Function(node) => {
                let expression = transformer(*node.expression)?;
                transformer(Function::new(node.signature, expression))?
            }
            Node::FunctionCall(node) => {
                let target = transformer(*node.target)?;
                let elements = node
                    .arguments
                    .into_iter()
                    .map(|node| transformer(node))
                    .collect::<Result<Vec<_>, E>>()?;
                transformer(FunctionCall::new(target, elements))?
            }
            Node::And(node) => {
                let elements = node
                    .elements
                    .into_iter()
                    .map(|node| transformer(node))
                    .collect::<Result<Vec<_>, E>>()?;
                transformer(And::new(elements))?
            }
            Node::Or(node) => {
                let elements = node
                    .elements
                    .into_iter()
                    .map(|node| transformer(node))
                    .collect::<Result<Vec<_>, E>>()?;
                transformer(Or::new(elements))?
            }
            Node::Equals(node) => {
                let left = transformer(*node.left)?;
                let right = transformer(*node.right)?;
                transformer(Equals::new(left, right))?
            }
            Node::LessThan(node) => {
                let left = transformer(*node.left)?;
                let right = transformer(*node.right)?;
                transformer(LessThan::new(left, right))?
            }
            Node::LessThanOrEquals(node) => {
                let left = transformer(*node.left)?;
                let right = transformer(*node.right)?;
                transformer(LessThanOrEquals::new(left, right))?
            }
            Node::GreaterThan(node) => {
                let left = transformer(*node.left)?;
                let right = transformer(*node.right)?;
                transformer(GreaterThan::new(left, right))?
            }
            Node::GreaterThanOrEquals(node) => {
                let left = transformer(*node.left)?;
                let right = transformer(*node.right)?;
                transformer(GreaterThanOrEquals::new(left, right))?
            }
            Node::Definition(node) => {
                let expression = transformer(*node.expression)?;
                transformer(Definition::new(node.symbol, expression))?
            }
            Node::Factorial(node) => {
                let value = transformer(*node.value)?;
                transformer(Factorial::new(value))?
            }
        };

        *node.trace_mut() = trace;

        Ok(node)
    }
}
