use std::{
    cmp::{max, min},
    iter::zip,
};

use crate::core::node::Tensor;

impl Tensor {
    pub fn get_sub_tensor(&self, range: Vec<(usize, usize)>) -> Option<Self> {
        if range.len() != self.shape.len() {
            return None;
        }
        for (&dim, &(min, max)) in zip(&self.shape, &range) {
            if min > dim || max > dim {
                return None;
            }
        }
        let new_shape: Vec<_> = range
            .iter()
            .map(|(dim_min, dim_max)| {
                max(dim_max, dim_min) - min(dim_max, dim_min) + 1
            })
            .collect();
        let mut new_elements = Vec::new();
        for (inner_index, element) in self.elements.iter().enumerate() {
            let outer_index = self.convert_to_outer_index(inner_index);
            match outer_index {
                None => {
                    return None;
                }
                Some(outer_index) => {
                    let selected = zip(&outer_index, &range).all(
                        |(&pos, &(min_pos, max_pos))| {
                            min(min_pos, max_pos) <= pos
                                && pos <= max(min_pos, max_pos)
                        },
                    );
                    if selected {
                        new_elements.push(element.clone());
                    }
                }
            }
        }
        Tensor::new_with_shape(new_elements, new_shape)
    }

    pub fn convert_to_outer_index(
        &self,
        inner_index: usize,
    ) -> Option<Vec<usize>> {
        if inner_index >= self.elements.len() {
            return None;
        }
        let mut rest = inner_index;
        let mut outer_index = Vec::new();
        for offset in self.dimension_offsets() {
            outer_index.push(rest.div_euclid(offset));
            rest = rest % offset;
        }
        Some(outer_index)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::node::{Number, Tensor};

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
    fn get_sub_tensor() {
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

        assert_eq!(
            tensor.get_sub_tensor(vec![(1, 1), (1, 2)]).unwrap(),
            Tensor::new(vec![Tensor::new(vec![
                Number::new(5.0).into(),
                Number::new(6.0).into(),
            ])
            .into()])
        );
    }
}
