use crate::core::node::*;

impl Tensor {
    pub fn get_element(&self, index: &Vec<usize>) -> Option<&Node> {
        match self.convert_to_inner_index(index) {
            Some(inner_index) => self.elements.get(inner_index),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::node::*;

    #[test]
    fn get_item_1d() {
        let tensor = Tensor::new(vec![
            Number::new(0.0).into(),
            Number::new(1.0).into(),
            Number::new(2.0).into(),
            Number::new(3.0).into(),
        ]);

        assert_eq!(
            tensor.get_element(&vec![0]).unwrap().clone(),
            Number::new(0.0).into(),
        );

        assert_eq!(
            tensor.get_element(&vec![1]).unwrap().clone(),
            Number::new(1.0).into(),
        );

        assert_eq!(
            tensor.get_element(&vec![2]).unwrap().clone(),
            Number::new(2.0).into(),
        );

        assert_eq!(
            tensor.get_element(&vec![3]).unwrap().clone(),
            Number::new(3.0).into(),
        );

        assert_eq!(tensor.get_element(&vec![4]), None,);
    }

    #[test]
    fn get_item_2d() {
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
            tensor.get_element(&vec![0, 0]).unwrap().clone(),
            Number::new(0.0).into(),
        );

        assert_eq!(
            tensor.get_element(&vec![1, 2]).unwrap().clone(),
            Number::new(6.0).into(),
        );

        assert_eq!(
            tensor.get_element(&vec![1, 3]).unwrap().clone(),
            Number::new(7.0).into(),
        );

        assert_eq!(
            tensor.get_element(&vec![0, 3]).unwrap().clone(),
            Number::new(3.0).into(),
        );

        assert_eq!(tensor.get_element(&vec![0, 4]), None);
        assert_eq!(tensor.get_element(&vec![2, 3]), None);
    }

    #[test]
    fn get_item_3d() {
        let tensor = Tensor::new(vec![
            Tensor::new(vec![
                Tensor::new(vec![
                    Number::new(0.0).into(),
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                ])
                .into(),
                Tensor::new(vec![
                    Number::new(3.0).into(),
                    Number::new(4.0).into(),
                    Number::new(5.0).into(),
                ])
                .into(),
            ])
            .into(),
            Tensor::new(vec![
                Tensor::new(vec![
                    Number::new(6.0).into(),
                    Number::new(7.0).into(),
                    Number::new(8.0).into(),
                ])
                .into(),
                Tensor::new(vec![
                    Number::new(9.0).into(),
                    Number::new(10.0).into(),
                    Number::new(11.0).into(),
                ])
                .into(),
            ])
            .into(),
        ]);

        assert_eq!(
            tensor.get_element(&vec![0, 0, 0]).unwrap().clone(),
            Number::new(0.0).into(),
        );

        assert_eq!(
            tensor.get_element(&vec![1, 1, 2]).unwrap().clone(),
            Number::new(11.0).into(),
        );

        assert_eq!(
            tensor.get_element(&vec![1, 0, 0]).unwrap().clone(),
            Number::new(6.0).into(),
        );

        assert_eq!(tensor.get_element(&vec![0, 3]), None);
        assert_eq!(tensor.get_element(&vec![0, 4]), None);
        assert_eq!(tensor.get_element(&vec![2, 3]), None);
    }
}
