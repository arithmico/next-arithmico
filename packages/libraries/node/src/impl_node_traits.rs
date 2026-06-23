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
    };
}
