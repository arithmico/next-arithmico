use std::{
    cmp::{max, min},
    iter::zip,
};

use crate::core::node::nodes::Tensor;

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
}

#[cfg(test)]
mod tests {
    use crate::core::node::nodes::*;

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
