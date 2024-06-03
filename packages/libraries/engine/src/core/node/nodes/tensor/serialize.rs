use std::iter::zip;

use crate::core::{
    context::Context,
    node::{Node, SerializeNode},
};

use super::Tensor;

impl SerializeNode for Tensor {
    fn transform_before_serialization(&self, _context: &Context) -> Node {
        self.clone().into()
    }

    fn serialize(&self, context: &Context) -> String {
        let serialized_elements: Vec<_> = self
            .elements
            .iter()
            .map(|element| element.serialize(context))
            .collect();
        let rank = self.get_rank();
        let mut inner_string = String::new();
        inner_string.push_str(&String::from("[").repeat(rank));
        let mut last_inner_index = 0;
        for (current_inner_index, serialized_element) in
            serialized_elements.iter().enumerate()
        {
            let last_outer_index =
                self.convert_to_outer_index(last_inner_index).unwrap();
            let current_outer_index =
                self.convert_to_outer_index(current_inner_index).unwrap();
            let mut index_delta: Vec<_> =
                zip(&last_outer_index, &current_outer_index)
                    .map(|(&last_index, &current_index)| {
                        (last_index as isize - current_index as isize).abs()
                    })
                    .collect();
            index_delta.pop();
            let mut separator = String::new();
            let sep_count = index_delta.iter().fold(0, |a, b| a + b);
            for _ in 0..sep_count {
                separator.push_str("]");
            }
            if current_inner_index != 0 {
                separator.push_str(", ");
            }
            for _ in 0..sep_count {
                separator.push_str("[");
            }
            inner_string.push_str(&separator);
            inner_string.push_str(&serialized_element);
            last_inner_index = current_inner_index;
        }
        inner_string.push_str(&String::from("]").repeat(rank));
        inner_string
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_utils::serialization_test;

    #[test]
    fn serialize_tensor() {
        serialization_test("[1,2,3]", "[1, 2, 3]");
    }

    #[test]
    fn serialize_empty_tensor() {
        serialization_test("[]", "[]");
    }

    #[test]
    fn serialize_nested_tensor() {
        serialization_test("[[1, 2], [3,4]]", "[[1, 2], [3, 4]]");
        serialization_test(
            "[[1, 2], [3, 4], [5, 6]]",
            "[[1, 2], [3, 4], [5, 6]]",
        );
        serialization_test(
            "[[[1], [2]], [[3], [4]], [[5], [6]]]",
            "[[[1], [2]], [[3], [4]], [[5], [6]]]",
        );
    }
}
