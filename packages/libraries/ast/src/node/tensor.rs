use std::iter::zip;

use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Tensor {
    pub elements: Vec<Node>,
    pub shape: Vec<usize>,
}

impl Tensor {
    pub fn new(elements: Vec<Node>) -> Node {
        let shape = Tensor::get_shape(&elements);
        Node::Tensor(Self {
            elements: Tensor::flatten_elements(&elements, &shape),
            shape,
        })
    }

    pub fn new_with_shape(shape: Vec<usize>, elements: Vec<Node>) -> Node {
        assert!(
            shape
                .clone()
                .into_iter()
                .reduce(|acc, val| acc * val)
                .unwrap_or(0)
                == elements.len()
        );

        Node::Tensor(Self { elements, shape })
    }

    pub fn get_rank(&self) -> usize {
        self.shape.len()
    }

    pub fn dimension_offsets(&self) -> Vec<usize> {
        dimension_offsets(&self.shape)
    }

    pub fn convert_to_inner_index(&self, index: &Vec<usize>) -> Option<usize> {
        convert_to_inner_index(&self.shape, index)
    }

    pub fn convert_to_outer_index(
        &self,
        inner_index: usize,
    ) -> Option<Vec<usize>> {
        convert_to_outer_index(&self.shape, inner_index)
    }

    fn get_shape(elements: &Vec<Node>) -> Vec<usize> {
        let inner_shape = elements
            .iter()
            .map(|element| match element {
                Node::Tensor(tensor) => Some(tensor.shape.clone()),
                _ => None,
            })
            .reduce(|left, right| match (left, right) {
                (Some(left), Some(right)) => {
                    if left.len() != right.len() {
                        None
                    } else {
                        if zip(left.iter(), right.iter())
                            .all(|(left, right)| left == right)
                        {
                            Some(left)
                        } else {
                            None
                        }
                    }
                }
                _ => None,
            });

        match inner_shape {
            None => vec![elements.len()],
            Some(inner_shape) => match inner_shape {
                None => vec![elements.len()],
                Some(mut inner_shape) => {
                    let mut shape = vec![elements.len()];
                    shape.append(&mut inner_shape);
                    shape
                }
            },
        }
    }

    fn flatten_elements(elements: &Vec<Node>, shape: &Vec<usize>) -> Vec<Node> {
        if shape.len() == 0 {
            unreachable!();
        }
        if shape.len() == 1 {
            return elements.clone();
        }
        elements
            .iter()
            .flat_map(|item| match item {
                Node::Tensor(tensor) => tensor.elements.clone(),
                _ => unreachable!(),
            })
            .collect()
    }
}

pub fn get_capacity(shape: &Vec<usize>) -> usize {
    shape.iter().fold(1, |a, &b| a * b)
}

pub fn convert_to_outer_index(
    shape: &Vec<usize>,
    inner_index: usize,
) -> Option<Vec<usize>> {
    if inner_index >= get_capacity(shape) {
        return None;
    }
    let mut rest = inner_index;
    let mut outer_index = Vec::new();
    for offset in dimension_offsets(shape) {
        outer_index.push(rest.div_euclid(offset));
        rest = rest % offset;
    }
    Some(outer_index)
}

pub fn convert_to_inner_index(
    shape: &Vec<usize>,
    index: &Vec<usize>,
) -> Option<usize> {
    if shape.len() != index.len() {
        return None;
    }
    let dimension_offsets: Vec<_> = dimension_offsets(shape);

    zip(index, zip(shape, dimension_offsets)).fold(
        Some(0usize),
        |acc, (&index, (&dimension_length, dimension_offset))| match acc {
            None => None,
            Some(acc) => {
                if index >= dimension_length {
                    return None;
                }
                Some(acc + index * dimension_offset)
            }
        },
    )
}

pub fn dimension_offsets(shape: &Vec<usize>) -> Vec<usize> {
    shape
        .iter()
        .rev()
        .scan(1usize, |state, &dimension| {
            let offset = *state;
            *state = offset * dimension;
            Some(offset)
        })
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {

    use crate::node::Number;

    use super::*;

    #[test]
    fn tensor_shape_rank1() {
        assert_eq!(Tensor::get_shape(&vec![Number::new(1.)]), vec![1])
    }

    #[test]
    fn tensor_shape_rank2() {
        assert_eq!(
            Tensor::get_shape(&vec![
                Tensor::new(vec![Number::new(1.)]),
                Tensor::new(vec![Number::new(2.)]),
                Tensor::new(vec![Number::new(3.)]),
            ]),
            vec![3, 1]
        )
    }

    #[test]
    fn tensor_shape_rank3() {
        assert_eq!(
            Tensor::get_shape(&vec![Tensor::new(vec![
                Tensor::new(vec![
                    Number::new(1.),
                    Number::new(2.),
                    Number::new(3.)
                ]),
                Tensor::new(vec![
                    Number::new(4.),
                    Number::new(5.),
                    Number::new(6.)
                ]),
            ]),]),
            vec![1, 2, 3]
        )
    }

    #[test]
    fn new_tensor_rank1() {
        assert_eq!(
            Tensor::new(vec![Number::new(1.)]),
            Node::Tensor(Tensor {
                elements: vec![Number::new(1.)],
                shape: vec![1]
            })
        )
    }

    #[test]
    fn new_tensor_rank2() {
        assert_eq!(
            Tensor::new(vec![
                Tensor::new(vec![Number::new(1.)]),
                Tensor::new(vec![Number::new(2.)]),
                Tensor::new(vec![Number::new(3.)]),
            ]),
            Node::Tensor(Tensor {
                elements: vec![
                    Number::new(1.),
                    Number::new(2.),
                    Number::new(3.)
                ],
                shape: vec![3, 1]
            })
        )
    }

    #[test]
    fn new_tensor_rank3() {
        assert_eq!(
            Tensor::new(vec![Tensor::new(vec![
                Tensor::new(vec![
                    Number::new(1.),
                    Number::new(2.),
                    Number::new(3.)
                ]),
                Tensor::new(vec![
                    Number::new(4.),
                    Number::new(5.),
                    Number::new(6.)
                ]),
            ]),]),
            Node::Tensor(Tensor {
                elements: vec![
                    Number::new(1.),
                    Number::new(2.),
                    Number::new(3.),
                    Number::new(4.),
                    Number::new(5.),
                    Number::new(6.),
                ],
                shape: vec![1, 2, 3]
            })
        )
    }
}
