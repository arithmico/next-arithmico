use node::Node;

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
        let node = match self {
            Node::Boolean(node) => transformer(Node::Boolean(node))?,
            Node::Number(node) => transformer(Node::Number(node))?,
            Node::Symbol(node) => transformer(Node::Symbol(node))?,
            Node::HostFunction(node) => transformer(Node::HostFunction(node))?,
            Node::Sum(mut node) => {
                node.elements = node
                    .elements
                    .into_iter()
                    .map(|node| transformer(node))
                    .collect::<Result<Vec<_>, E>>()?;
                transformer(Node::Sum(node))?
            }
            Node::Negate(mut node) => {
                *node.value = transformer(*node.value)?;
                transformer(Node::Negate(node))?
            }
            Node::Product(mut node) => {
                node.elements = node
                    .elements
                    .into_iter()
                    .map(|node| transformer(node))
                    .collect::<Result<Vec<_>, E>>()?;

                transformer(Node::Product(node))?
            }
            Node::Division(mut node) => {
                *node.dividend = transformer(*node.dividend)?;
                *node.divisor = transformer(*node.divisor)?;
                transformer(Node::Division(node))?
            }
            Node::Power(mut node) => {
                *node.base = transformer(*node.base)?;
                *node.exponent = transformer(*node.exponent)?;
                transformer(Node::Power(node))?
            }
            Node::Tensor(mut node) => {
                node.elements = node
                    .elements
                    .into_iter()
                    .map(|node| transformer(node))
                    .collect::<Result<Vec<_>, E>>()?;
                transformer(Node::Tensor(node))?
            }
            Node::Function(mut node) => {
                *node.expression = transformer(*node.expression)?;
                transformer(Node::Function(node))?
            }
            Node::FunctionCall(mut node) => {
                *node.target = transformer(*node.target)?;
                node.arguments = node
                    .arguments
                    .into_iter()
                    .map(|node| transformer(node))
                    .collect::<Result<Vec<_>, E>>()?;
                transformer(Node::FunctionCall(node))?
            }
            Node::And(mut node) => {
                node.elements = node
                    .elements
                    .into_iter()
                    .map(|node| transformer(node))
                    .collect::<Result<Vec<_>, E>>()?;
                transformer(Node::And(node))?
            }
            Node::Or(mut node) => {
                node.elements = node
                    .elements
                    .into_iter()
                    .map(|node| transformer(node))
                    .collect::<Result<Vec<_>, E>>()?;
                transformer(Node::Or(node))?
            }
            Node::Equals(mut node) => {
                *node.left = transformer(*node.left)?;
                *node.right = transformer(*node.right)?;
                transformer(Node::Equals(node))?
            }
            Node::LessThan(mut node) => {
                *node.left = transformer(*node.left)?;
                *node.right = transformer(*node.right)?;
                transformer(Node::LessThan(node))?
            }
            Node::LessThanOrEquals(mut node) => {
                *node.left = transformer(*node.left)?;
                *node.right = transformer(*node.right)?;
                transformer(Node::LessThanOrEquals(node))?
            }
            Node::GreaterThan(mut node) => {
                *node.left = transformer(*node.left)?;
                *node.right = transformer(*node.right)?;
                transformer(Node::GreaterThan(node))?
            }
            Node::GreaterThanOrEquals(mut node) => {
                *node.left = transformer(*node.left)?;
                *node.right = transformer(*node.right)?;
                transformer(Node::GreaterThanOrEquals(node))?
            }
            Node::Definition(mut node) => {
                *node.expression = transformer(*node.expression)?;
                transformer(Node::Definition(node))?
            }
            Node::Factorial(mut node) => {
                *node.value = transformer(*node.value)?;
                transformer(Node::Factorial(node))?
            }
            Node::DataFrame(mut node) => {
                node.data = node
                    .data
                    .into_iter()
                    .map(|node| match node {
                        Some(node) => transformer(node).map(|node| Some(node)),
                        None => Ok(None),
                    })
                    .collect::<Result<Vec<Option<_>>, E>>()?;
                transformer(Node::DataFrame(node))?
            }
        };

        Ok(node)
    }
}
