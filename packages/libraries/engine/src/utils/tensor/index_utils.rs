use core::iter::zip;

use crate::core::node::*;

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

impl Tensor {
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
}

#[cfg(test)]
mod tests {
    use crate::core::node::*;

    #[test]
    fn convert_to_outer_index() {
        let tensor = Tensor::new(vec![
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
        ]);

        assert_eq!(tensor.convert_to_outer_index(0).unwrap(), vec![0, 0]);
        assert_eq!(tensor.convert_to_outer_index(3).unwrap(), vec![0, 3]);
        assert_eq!(tensor.convert_to_outer_index(4).unwrap(), vec![1, 0]);
        assert_eq!(tensor.convert_to_outer_index(7).unwrap(), vec![1, 3]);
        assert_eq!(tensor.convert_to_outer_index(8), None);
    }

    #[test]
    fn dimension_offsets() {
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
            ])
            .dimension_offsets(),
            vec![4, 1],
        )
    }

    #[test]
    fn convert_to_inner_index() {
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
            ])
            .convert_to_inner_index(&vec![1, 2])
            .unwrap(),
            6,
        )
    }
}
