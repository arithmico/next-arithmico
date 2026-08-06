use std::iter::zip;

use node::{IntoNode, Node, Tensor};

use crate::core::{
    Context, Serialize, SerializeNodeError, SerializeUtils,
    get_argument_separator,
};

impl SerializeUtils for Tensor {
    fn normalize_node(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        let elements: Result<Vec<Node>, SerializeNodeError> = self
            .elements
            .iter()
            .map(|element| element.normalize_node(context))
            .collect();

        Ok(Tensor::new_with_shape(self.shape.clone(), elements?).into_node())
    }

    fn child_requires_parenthesis(
        &self,
        _child: &Node,
        _position: usize,
    ) -> bool {
        false
    }
}

impl Serialize for Tensor {
    fn serialize(
        &self,
        context: &Context,
    ) -> Result<String, SerializeNodeError> {
        let serialized_elements: Result<Vec<String>, _> = self
            .elements
            .iter()
            .map(|element| element.serialize(context))
            .collect();

        let rank = self.get_rank();
        let mut inner_string = String::new();
        inner_string.push_str(&String::from("[").repeat(rank));
        let mut last_inner_index = 0;

        for (current_inner_index, serialized_element) in
            serialized_elements?.into_iter().enumerate()
        {
            let last_outer_index =
                self.convert_to_outer_index(last_inner_index).unwrap();
            let current_outer_index =
                self.convert_to_outer_index(current_inner_index).unwrap();
            let mut index_delta: Vec<_> =
                zip(&last_outer_index, &current_outer_index)
                    .map(|(&last_index, &current_index)| {
                        (last_index as isize - current_index as isize)
                            .abs()
                            .min(1)
                    })
                    .collect();

            index_delta.pop();
            let mut separator = String::new();
            let sep_count = index_delta.iter().fold(0, |a, b| a + b);

            for _ in 0..sep_count {
                separator.push_str("]");
            }

            if current_inner_index != 0 {
                separator.push_str(&get_argument_separator(context));
            }

            for _ in 0..sep_count {
                separator.push_str("[");
            }

            inner_string.push_str(&separator);
            inner_string.push_str(&serialized_element);
            last_inner_index = current_inner_index;
        }
        inner_string.push_str(&String::from("]").repeat(rank));

        Ok(inner_string)
    }
}

#[cfg(test)]
mod tests {
    use node::{Number, Symbol};

    use crate::core::serialize_node;

    use super::*;

    #[test]
    fn serialize_empty_tensor() {
        assert_eq!(
            serialize_node(&Tensor::new_node(vec![]), &Context::default())
                .unwrap(),
            "[]"
        );
    }

    #[test]
    fn serialize_tensor_1() {
        assert_eq!(
            serialize_node(
                &Tensor::new_node(vec![Symbol::new("a")]),
                &Context::default()
            )
            .unwrap(),
            "[a]"
        );
    }

    #[test]
    fn serialize_tensor_2() {
        assert_eq!(
            serialize_node(
                &Tensor::new_node(vec![Symbol::new("a"), Symbol::new("b")]),
                &Context::default()
            )
            .unwrap(),
            "[a, b]"
        );
    }

    #[test]
    fn serialize_nested_tensor_rank_2() {
        assert_eq!(
            serialize_node(
                &Tensor::new_node(vec![
                    Tensor::new_node(vec![Symbol::new("a"), Symbol::new("b")]),
                    Tensor::new_node(vec![Symbol::new("c"), Symbol::new("d")]),
                ]),
                &Context::default()
            )
            .unwrap(),
            "[[a, b], [c, d]]"
        );
    }

    #[test]
    fn serialize_nested_tensor_rank_3() {
        assert_eq!(
            serialize_node(
                &Tensor::new_node(vec![
                    Tensor::new_node(vec![
                        Tensor::new_node(vec![
                            Symbol::new("a"),
                            Symbol::new("b")
                        ]),
                        Tensor::new_node(vec![
                            Symbol::new("a"),
                            Symbol::new("b")
                        ]),
                        Tensor::new_node(vec![
                            Symbol::new("a"),
                            Symbol::new("b")
                        ]),
                    ]),
                    Tensor::new_node(vec![
                        Tensor::new_node(vec![
                            Symbol::new("a"),
                            Symbol::new("b")
                        ]),
                        Tensor::new_node(vec![
                            Symbol::new("a"),
                            Symbol::new("b")
                        ]),
                        Tensor::new_node(vec![
                            Symbol::new("a"),
                            Symbol::new("b")
                        ]),
                    ]),
                ]),
                &Context::default()
            )
            .unwrap(),
            "[[[a, b], [a, b], [a, b]], [[a, b], [a, b], [a, b]]]"
        );
    }

    #[test]
    fn serialize_mixed_nested_tensor() {
        assert_eq!(
            serialize_node(
                &Tensor::new_node(vec![
                    Tensor::new_node(vec![
                        Tensor::new_node(vec![
                            Symbol::new("a"),
                            Symbol::new("b")
                        ]),
                        Tensor::new_node(vec![
                            Symbol::new("a"),
                            Symbol::new("b")
                        ]),
                        Tensor::new_node(vec![
                            Symbol::new("a"),
                            Symbol::new("b")
                        ]),
                    ]),
                    Number::new_node(1.)
                ]),
                &Context::default()
            )
            .unwrap(),
            "[[[a, b], [a, b], [a, b]], 1]"
        );
    }
}
