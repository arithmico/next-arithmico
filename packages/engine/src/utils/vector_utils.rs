use crate::core::node::{Node, Tensor};

pub fn get_tensor_dimensions(node: &Node) -> Option<Vec<usize>> {
    match node {
        Node::Tensor(Tensor { elements: values }) => {
            let mut common_item_dimensions = values
                .iter()
                .map(|item| get_tensor_dimensions(item))
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

            let mut dimensions = vec![values.len()];
            dimensions.append(&mut common_item_dimensions);
            Some(dimensions)
        }
        _ => None,
    }
}

pub fn get_tensor_rank(node: &Node) -> Option<usize> {
    get_tensor_dimensions(node).and_then(|dims| Some(dims.len()))
}

#[cfg(test)]
mod tests {
    use crate::core::node::Number;

    use super::*;

    #[test]
    fn dimensions_of_non_vector_node() {
        assert_eq!(get_tensor_dimensions(&Number::new(1.0).into()), None);
    }

    #[test]
    fn dimensions_of_1d_vector_with_inconsistent_shape() {
        assert_eq!(
            get_tensor_dimensions(
                &Tensor::new(vec![
                    Tensor::new(vec![Number::new(1.0).into()]).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into()
                ])
                .into()
            ),
            Some(vec![3])
        );
    }

    #[test]
    fn dimensions_of_2x3_matrix() {
        assert_eq!(
            get_tensor_dimensions(
                &Tensor::new(vec![
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
                ])
                .into()
            ),
            Some(vec![2, 3])
        );
    }

    #[test]
    fn dimensions_of_1d_vector() {
        assert_eq!(
            get_tensor_dimensions(
                &Tensor::new(vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into()
                ])
                .into(),
            ),
            Some(vec![3])
        );
    }

    #[test]
    fn rank_of_non_vector() {
        assert_eq!(get_tensor_rank(&Number::new(1.0).into()), None);
    }

    #[test]
    fn rank_of_1d_vector() {
        assert_eq!(
            get_tensor_rank(
                &Tensor::new(vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into()
                ])
                .into(),
            ),
            Some(1)
        );
    }

    #[test]
    fn rank_of_2x3_matrix() {
        assert_eq!(
            get_tensor_rank(
                &Tensor::new(vec![
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
                ])
                .into()
            ),
            Some(2)
        );
    }

    #[test]
    fn rank_of_1d_vector_with_inconsistent_shape() {
        assert_eq!(
            get_tensor_rank(
                &Tensor::new(vec![
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
                ])
                .into()
            ),
            Some(1)
        );
    }
}
