use crate::core::node::*;

impl Tensor {
    pub fn get_rank(&self) -> usize {
        self.shape.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::node::*;

    #[test]
    fn get_rank_1() {
        let tensor = Tensor::new(vec![
            Number::new(0.0).into(),
            Number::new(1.0).into(),
            Number::new(2.0).into(),
            Number::new(3.0).into(),
        ]);

        assert_eq!(tensor.get_rank(), 1);
    }

    #[test]
    fn get_rank_2() {
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

        assert_eq!(tensor.get_rank(), 2);
    }
}
