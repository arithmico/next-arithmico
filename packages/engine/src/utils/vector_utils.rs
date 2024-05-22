use crate::core::node::{Node, Tensor};

pub fn get_tensor_dimensions(node: &Tensor) -> Vec<usize> {
    let mut common_item_dimensions = node
        .elements
        .iter()
        .map(|item| match item {
            Node::Tensor(node) => Some(get_tensor_dimensions(node)),
            _ => None,
        })
        .reduce(|acc, item| match (acc, item) {
            (None, _) => None,
            (_, None) => None,
            (Some(acc_dims), Some(item_dims)) => {
                if acc_dims == item_dims {
                    Some(acc_dims)
                } else {
                    None
                }
            }
        })
        .unwrap_or_else(|| Some(Vec::<usize>::new())) // empty vector
        .unwrap_or_else(|| Vec::<usize>::new()); // inconsistent shape

    let mut dimensions = vec![node.elements.len()];
    dimensions.append(&mut common_item_dimensions);
    dimensions
}

pub fn get_tensor_rank(node: &Tensor) -> usize {
    get_tensor_dimensions(node).len()
}

#[cfg(test)]
mod tests {
    use crate::core::node::Number;

    use super::*;

    #[test]
    fn dimensions_of_1d_vector_with_inconsistent_shape() {
        assert_eq!(
            get_tensor_dimensions(&Tensor::new(vec![
                Tensor::new(vec![Number::new(1.0).into()]).into(),
                Number::new(2.0).into(),
                Number::new(3.0).into()
            ])),
            vec![3]
        );
    }

    #[test]
    fn dimensions_of_2x3_matrix() {
        assert_eq!(
            get_tensor_dimensions(&Tensor::new(vec![
                Tensor::new(vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into()
                ])
                .into(),
                Tensor::new(vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into()
                ])
                .into()
            ])),
            vec![2, 3]
        );
    }

    #[test]
    fn dimensions_of_1d_vector() {
        assert_eq!(
            get_tensor_dimensions(&Tensor::new(vec![
                Number::new(1.0).into(),
                Number::new(2.0).into(),
                Number::new(3.0).into()
            ])),
            vec![3]
        );
    }

    #[test]
    fn rank_of_1d_vector() {
        assert_eq!(
            get_tensor_rank(&Tensor::new(vec![
                Number::new(1.0).into(),
                Number::new(2.0).into(),
                Number::new(3.0).into()
            ])),
            1
        );
    }

    #[test]
    fn rank_of_2x3_matrix() {
        assert_eq!(
            get_tensor_rank(&Tensor::new(vec![
                Tensor::new(vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into()
                ])
                .into(),
                Tensor::new(vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into()
                ])
                .into()
            ])),
            2
        );
    }

    #[test]
    fn rank_of_1d_vector_with_inconsistent_shape() {
        assert_eq!(
            get_tensor_rank(&Tensor::new(vec![
                Tensor::new(vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into()
                ])
                .into(),
                Tensor::new(vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                ])
                .into()
            ])),
            1
        );
    }
}
