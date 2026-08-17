use node::Node;

use crate::sealed::Sealed;

#[allow(private_bounds)]
pub trait TransformNodeWithContext: Sealed {
    fn transform_with_context<E, Ctx>(
        self,
        transformer: impl Fn(Node, &Ctx) -> Result<Node, E>,
        context: &Ctx,
    ) -> Result<Node, E>;
}

impl TransformNodeWithContext for Node {
    fn transform_with_context<E, Ctx>(
        self,
        transformer: impl Fn(Node, &Ctx) -> Result<Node, E>,
        context: &Ctx,
    ) -> Result<Node, E> {
        let node = match self {
            Node::Boolean(node) => transformer(Node::Boolean(node), context)?,
            Node::Number(node) => transformer(Node::Number(node), context)?,
            Node::Symbol(node) => transformer(Node::Symbol(node), context)?,
            Node::HostFunction(node) => {
                transformer(Node::HostFunction(node), context)?
            }
            Node::Sum(mut node) => {
                node.elements = node
                    .elements
                    .into_iter()
                    .map(|node| transformer(node, context))
                    .collect::<Result<Vec<_>, E>>()?;
                transformer(Node::Sum(node), context)?
            }
            Node::Negate(mut node) => {
                *node.value = transformer(*node.value, context)?;
                transformer(Node::Negate(node), context)?
            }
            Node::Product(mut node) => {
                node.elements = node
                    .elements
                    .into_iter()
                    .map(|node| transformer(node, context))
                    .collect::<Result<Vec<_>, E>>()?;

                transformer(Node::Product(node), context)?
            }
            Node::Division(mut node) => {
                *node.dividend = transformer(*node.dividend, context)?;
                *node.divisor = transformer(*node.divisor, context)?;
                transformer(Node::Division(node), context)?
            }
            Node::Power(mut node) => {
                *node.base = transformer(*node.base, context)?;
                *node.exponent = transformer(*node.exponent, context)?;
                transformer(Node::Power(node), context)?
            }
            Node::Tensor(mut node) => {
                node.elements = node
                    .elements
                    .into_iter()
                    .map(|node| transformer(node, context))
                    .collect::<Result<Vec<_>, E>>()?;
                transformer(Node::Tensor(node), context)?
            }
            Node::Function(mut node) => {
                *node.expression = transformer(*node.expression, context)?;
                transformer(Node::Function(node), context)?
            }
            Node::FunctionCall(mut node) => {
                *node.target = transformer(*node.target, context)?;
                node.arguments = node
                    .arguments
                    .into_iter()
                    .map(|node| transformer(node, context))
                    .collect::<Result<Vec<_>, E>>()?;
                transformer(Node::FunctionCall(node), context)?
            }
            Node::And(mut node) => {
                node.elements = node
                    .elements
                    .into_iter()
                    .map(|node| transformer(node, context))
                    .collect::<Result<Vec<_>, E>>()?;
                transformer(Node::And(node), context)?
            }
            Node::Or(mut node) => {
                node.elements = node
                    .elements
                    .into_iter()
                    .map(|node| transformer(node, context))
                    .collect::<Result<Vec<_>, E>>()?;
                transformer(Node::Or(node), context)?
            }
            Node::Equals(mut node) => {
                *node.left = transformer(*node.left, context)?;
                *node.right = transformer(*node.right, context)?;
                transformer(Node::Equals(node), context)?
            }
            Node::LessThan(mut node) => {
                *node.left = transformer(*node.left, context)?;
                *node.right = transformer(*node.right, context)?;
                transformer(Node::LessThan(node), context)?
            }
            Node::LessThanOrEquals(mut node) => {
                *node.left = transformer(*node.left, context)?;
                *node.right = transformer(*node.right, context)?;
                transformer(Node::LessThanOrEquals(node), context)?
            }
            Node::GreaterThan(mut node) => {
                *node.left = transformer(*node.left, context)?;
                *node.right = transformer(*node.right, context)?;
                transformer(Node::GreaterThan(node), context)?
            }
            Node::GreaterThanOrEquals(mut node) => {
                *node.left = transformer(*node.left, context)?;
                *node.right = transformer(*node.right, context)?;
                transformer(Node::GreaterThanOrEquals(node), context)?
            }
            Node::Definition(mut node) => {
                *node.expression = transformer(*node.expression, context)?;
                transformer(Node::Definition(node), context)?
            }
            Node::Factorial(mut node) => {
                *node.value = transformer(*node.value, context)?;
                transformer(Node::Factorial(node), context)?
            }
            Node::DataFrame(mut node) => {
                node.data =
                    node.data
                        .into_iter()
                        .map(|node| match node {
                            Some(node) => transformer(node, context)
                                .map(|node| Some(node)),
                            None => Ok(None),
                        })
                        .collect::<Result<Vec<Option<_>>, E>>()?;
                transformer(Node::DataFrame(node), context)?
            }
        };

        Ok(node)
    }
}
