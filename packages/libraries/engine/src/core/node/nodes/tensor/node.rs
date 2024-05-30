use crate::core::node::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Tensor {
    pub shape: Vec<usize>,
    pub elements: Vec<Node>,
}

impl Tensor {
    pub fn new(elements: Vec<Node>) -> Tensor {
        let common_inner_shape = elements
            .iter()
            .map(|element| match element {
                Node::Tensor(tensor) => Some(tensor.shape.clone()),
                _ => None,
            })
            .reduce(|acc, element| match (acc, element) {
                (Some(acc_shape), Some(element_shape)) => {
                    if acc_shape == element_shape {
                        Some(acc_shape)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .unwrap_or_else(|| None);

        match common_inner_shape {
            Some(inner_shape) => {
                let mut shape = vec![elements.len()];
                shape.append(&mut inner_shape.clone());
                let flattened_elements = elements
                    .iter()
                    .flat_map(|element| match element {
                        Node::Tensor(tensor) => tensor.elements.clone(),
                        _ => unreachable!(),
                    })
                    .collect();

                Tensor {
                    elements: flattened_elements,
                    shape,
                }
            }
            None => Tensor {
                shape: vec![elements.len()],
                elements,
            },
        }
    }

    pub fn new_with_shape(
        elements: Vec<Node>,
        shape: Vec<usize>,
    ) -> Option<Self> {
        let total_elements = shape.iter().fold(1usize, |acc, &dim| acc * dim);
        if elements.len() != total_elements {
            None
        } else {
            Some(Tensor { elements, shape })
        }
    }
}

impl From<Tensor> for Node {
    fn from(value: Tensor) -> Node {
        Node::Tensor(value)
    }
}

impl BaseNode for Tensor {}

#[cfg(test)]
mod tests {
    use crate::core::node::nodes::*;

    #[test]
    fn flatten_nested_tensors() {
        assert_eq!(
            Tensor::new(vec![
                Tensor::new(vec![
                    Number::new(0.0).into(),
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into(),
                ])
                .into(),
                Tensor::new(vec![
                    Number::new(4.0).into(),
                    Number::new(5.0).into(),
                    Number::new(6.0).into(),
                    Number::new(7.0).into(),
                ])
                .into(),
            ]),
            Tensor {
                shape: vec![2, 4],
                elements: vec![
                    Number::new(0.0).into(),
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into(),
                    Number::new(4.0).into(),
                    Number::new(5.0).into(),
                    Number::new(6.0).into(),
                    Number::new(7.0).into(),
                ]
            }
        );

        assert_eq!(
            Tensor::new(vec![
                Tensor::new(vec![
                    Number::new(0.0).into(),
                    Number::new(1.0).into(),
                ])
                .into(),
                Tensor::new(vec![
                    Number::new(2.0).into(),
                    Number::new(3.0).into(),
                ])
                .into(),
            ]),
            Tensor {
                shape: vec![2, 2],
                elements: vec![
                    Number::new(0.0).into(),
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into(),
                ]
            }
        );
    }

    #[test]
    fn dont_flatten_nested_tensors() {
        assert_eq!(
            Tensor::new(vec![
                Tensor::new(vec![
                    Number::new(0.0).into(),
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into(),
                ])
                .into(),
                Tensor::new(vec![
                    Number::new(4.0).into(),
                    Number::new(5.0).into(),
                    Number::new(6.0).into(),
                ])
                .into(),
            ]),
            Tensor {
                shape: vec![2],
                elements: vec![
                    Tensor {
                        shape: vec![4],
                        elements: vec![
                            Number::new(0.0).into(),
                            Number::new(1.0).into(),
                            Number::new(2.0).into(),
                            Number::new(3.0).into(),
                        ]
                    }
                    .into(),
                    Tensor {
                        shape: vec![3],
                        elements: vec![
                            Number::new(4.0).into(),
                            Number::new(5.0).into(),
                            Number::new(6.0).into(),
                        ]
                    }
                    .into(),
                ]
            }
        )
    }
}
