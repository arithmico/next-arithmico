use std::iter::zip;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::space0,
    multi::many0, sequence::tuple, IResult,
};

use super::{FunctionCall, Node};

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

    pub fn parse(input: &str) -> IResult<&str, Node> {
        alt((parse_empty_tensor, parse_tensor, FunctionCall::parse))(input)
    }
}

fn parse_empty_tensor(input: &str) -> IResult<&str, Node> {
    let (remaining_input, _) = tuple((tag("["), space0, tag("]")))(input)?;
    Ok((remaining_input, Tensor::new(vec![])))
}

fn parse_tensor(input: &str) -> IResult<&str, Node> {
    let (remaining_input, (_, _, first, _, mut rest, _, _)) = tuple((
        tag("["),
        space0,
        Node::parse,
        space0,
        many0(parse_tensor_item),
        space0,
        tag("]"),
    ))(input)?;

    let mut elements = vec![first];
    elements.append(&mut rest);

    Ok((remaining_input, Tensor::new(elements)))
}

fn parse_tensor_item(input: &str) -> IResult<&str, Node> {
    let (remaining_input, (_, _, _, node)) =
        tuple((space0, tag(","), space0, Node::parse))(input)?;
    Ok((remaining_input, node))
}

#[cfg(test)]
mod tests {
    use crate::Number;

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

    #[test]
    fn parse_empty_tensor() {
        assert_eq!(
            Tensor::parse("[]").unwrap(),
            (
                "",
                Node::Tensor(Tensor {
                    elements: vec![],
                    shape: vec![0]
                })
            )
        )
    }

    #[test]
    fn parse_tensor_rank1() {
        assert_eq!(
            Tensor::parse("[1]").unwrap(),
            (
                "",
                Node::Tensor(Tensor {
                    elements: vec![Number::new(1.)],
                    shape: vec![1]
                })
            )
        )
    }

    #[test]
    fn parse_tensor_rank2() {
        assert_eq!(
            Tensor::parse("[[1],[2],[3]]").unwrap(),
            (
                "",
                Node::Tensor(Tensor {
                    elements: vec![
                        Number::new(1.),
                        Number::new(2.),
                        Number::new(3.)
                    ],
                    shape: vec![3, 1]
                })
            )
        )
    }

    #[test]
    fn parse_tensor_rank3() {
        assert_eq!(
            Tensor::parse("[[[1,2,3],[4,5,6]]]").unwrap(),
            (
                "",
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
        )
    }
}
