use crate::core::node::Tensor;

impl Tensor {
    pub fn get_rank(&self) -> usize {
        self.shape.len()
    }
}
