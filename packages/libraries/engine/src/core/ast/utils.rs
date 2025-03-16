#[macro_export]
macro_rules! impl_node_traits {
    ($node:ident) => {
        impl crate::core::GetNodeType for $node {
            fn node_type(&self) -> crate::core::NodeType {
                crate::core::NodeType::$node
            }
        }

        impl trace::Tracable for $node {
            fn trace(&self) -> &Trace {
                &self.trace
            }
        }

        impl trace::TracableMut for $node {
            fn trace_mut(&mut self) -> &mut Trace {
                &mut self.trace
            }
        }
    };
}
