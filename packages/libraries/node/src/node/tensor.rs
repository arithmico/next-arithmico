use std::iter::zip;

use trace::Trace;

use crate::{Node, impl_node_traits};

#[derive(PartialEq, Debug, Clone)]
pub struct Tensor {
    pub elements: Vec<Node>,
    pub shape: Vec<usize>,
    pub trace: Trace,
}

impl Tensor {
    pub fn new(elements: Vec<Node>) -> Self {
        let shape = Tensor::get_shape(&elements);
        let elements = Tensor::flatten_elements(&elements, &shape);
        Tensor::new_with_shape(shape, elements)
    }

    pub fn new_node(elements: Vec<Node>) -> Node {
        Self::new(elements).into()
    }

    pub fn new_with_shape(shape: Vec<usize>, elements: Vec<Node>) -> Self {
        assert!(
            shape
                .clone()
                .into_iter()
                .reduce(|acc, val| acc * val)
                .unwrap_or(0)
                == elements.len()
        );

        Self {
            elements,
            shape,
            trace: Trace::new(),
        }
    }

    pub fn get_rank(&self) -> usize {
        self.shape.len()
    }

    pub fn dimension_offsets(&self) -> Vec<usize> {
        dimension_offsets(&self.shape)
    }

    pub fn convert_to_inner_index(&self, index: &[usize]) -> Option<usize> {
        convert_to_inner_index(&self.shape, index)
    }

    pub fn convert_to_outer_index(
        &self,
        inner_index: usize,
    ) -> Option<Vec<usize>> {
        convert_to_outer_index(&self.shape, inner_index)
    }

    pub fn get_element(&self, index: &[usize]) -> Option<&Node> {
        match self.convert_to_inner_index(index) {
            Some(inner_index) => self.elements.get(inner_index),
            None => None,
        }
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

impl_node_traits!(Tensor);

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
        rest %= offset;
    }
    Some(outer_index)
}

pub fn convert_to_inner_index(
    shape: &[usize],
    index: &[usize],
) -> Option<usize> {
    if shape.len() != index.len() {
        return None;
    }
    let dimension_offsets: Vec<_> = dimension_offsets(shape);

    zip(index, zip(shape, dimension_offsets)).try_fold(
        0usize,
        |acc, (&index, (&dimension_length, dimension_offset))| {
            if index >= dimension_length {
                return None;
            }
            Some(acc + index * dimension_offset)
        },
    )
}

pub fn dimension_offsets(shape: &[usize]) -> Vec<usize> {
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

    use crate::Number;

    use super::*;

    #[test]
    fn tensor_shape_rank1() {
        assert_eq!(Tensor::get_shape(&vec![Number::new_node(1.)]), vec![1])
    }

    #[test]
    fn tensor_shape_rank2() {
        assert_eq!(
            Tensor::get_shape(&vec![
                Tensor::new_node(vec![Number::new_node(1.)]),
                Tensor::new_node(vec![Number::new_node(2.)]),
                Tensor::new_node(vec![Number::new_node(3.)]),
            ]),
            vec![3, 1]
        )
    }

    #[test]
    fn tensor_shape_rank3() {
        assert_eq!(
            Tensor::get_shape(&vec![Tensor::new_node(vec![
                Tensor::new_node(vec![
                    Number::new_node(1.),
                    Number::new_node(2.),
                    Number::new_node(3.)
                ]),
                Tensor::new_node(vec![
                    Number::new_node(4.),
                    Number::new_node(5.),
                    Number::new_node(6.)
                ]),
            ]),]),
            vec![1, 2, 3]
        )
    }

    #[test]
    fn new_tensor_rank1() {
        assert_eq!(
            Tensor::new_node(vec![Number::new_node(1.)]),
            Node::Tensor(Tensor {
                elements: vec![Number::new_node(1.)],
                shape: vec![1],
                trace: Trace::new()
            })
        )
    }

    #[test]
    fn new_tensor_rank2() {
        assert_eq!(
            Tensor::new_node(vec![
                Tensor::new_node(vec![Number::new_node(1.)]),
                Tensor::new_node(vec![Number::new_node(2.)]),
                Tensor::new_node(vec![Number::new_node(3.)]),
            ]),
            Node::Tensor(Tensor {
                elements: vec![
                    Number::new_node(1.),
                    Number::new_node(2.),
                    Number::new_node(3.)
                ],
                shape: vec![3, 1],
                trace: Trace::new()
            })
        )
    }

    #[test]
    fn new_tensor_rank3() {
        assert_eq!(
            Tensor::new_node(vec![Tensor::new_node(vec![
                Tensor::new_node(vec![
                    Number::new_node(1.),
                    Number::new_node(2.),
                    Number::new_node(3.)
                ]),
                Tensor::new_node(vec![
                    Number::new_node(4.),
                    Number::new_node(5.),
                    Number::new_node(6.)
                ]),
            ]),]),
            Node::Tensor(Tensor {
                elements: vec![
                    Number::new_node(1.),
                    Number::new_node(2.),
                    Number::new_node(3.),
                    Number::new_node(4.),
                    Number::new_node(5.),
                    Number::new_node(6.),
                ],
                shape: vec![1, 2, 3],
                trace: Trace::new()
            })
        )
    }
}
