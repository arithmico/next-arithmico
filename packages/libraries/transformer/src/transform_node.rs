use node::Node;

use crate::{TransformNodeWithContext, sealed::Sealed};

#[allow(private_bounds)]
pub trait TransformNode: Sealed {
    fn transform<E>(
        self,
        transformer: impl Fn(Node) -> Result<Node, E>,
    ) -> Result<Node, E>;
}

impl<T: TransformNodeWithContext> TransformNode for T {
    fn transform<E>(
        self,
        transformer: impl Fn(Node) -> Result<Node, E>,
    ) -> Result<Node, E> {
        self.transform_with_context(|node, _: &()| transformer(node), &())
    }
}
