#[macro_export]
macro_rules! impl_node_traits {
    ($node:ident) => {
        impl crate::GetNodeType for $node {
            fn node_type(&self) -> crate::NodeType {
                crate::NodeType::$node
            }
        }

        impl crate::GetStaticNodeType for $node {
            fn static_node_type() -> crate::NodeType {
                crate::NodeType::$node
            }
        }

        impl AsRef<trace::Trace> for $node {
            fn as_ref(&self) -> &Trace {
                &self.trace
            }
        }

        impl AsMut<trace::Trace> for $node {
            fn as_mut(&mut self) -> &mut Trace {
                &mut self.trace
            }
        }

        impl crate::IntoNode for $node {
            fn into_node(self) -> crate::Node {
                crate::Node::$node(self)
            }
        }

        impl<'a> TryFrom<&'a crate::Node> for &'a $node {
            type Error = crate::DowncastNodeError;

            fn try_from(value: &'a Node) -> Result<Self, Self::Error> {
                use crate::GetNodeType;
                use trace::Tracable;

                match value {
                    crate::Node::$node(node) => Ok(node),
                    node => Err(crate::DowncastNodeError {
                            expected: <Self as crate::GetStaticNodeType>::static_node_type(),
                            received: node.node_type(),
                            trace: node.trace().clone(),
                    }),
                }
            }
        }
    };
}
