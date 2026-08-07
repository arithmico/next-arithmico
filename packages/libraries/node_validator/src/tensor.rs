use node::Tensor;
use translate_core::TranslatedMessage;

use crate::{Error, translations::translation_resolver};

pub trait TensorValidator {
    fn validate_not_empty(&self) -> Result<&Self, Error>;

    fn validate_rank(&self, rank: usize) -> Result<&Self, Error>;

    fn validate_shape(&self, shape: &[usize]) -> Result<&Self, Error>;

    fn validate_equal_shapes(&self, other: &Self) -> Result<&Self, Error>;
}

impl TensorValidator for Tensor {
    fn validate_not_empty(&self) -> Result<&Self, Error> {
        if self.elements.is_empty() {
            let message = TranslatedMessage::new(
                "error.tensor.not_empty",
                translation_resolver,
            );
            return Err(Error::new(self, message));
        }
        Ok(self)
    }

    fn validate_rank(&self, rank: usize) -> Result<&Self, Error> {
        if self.get_rank() != rank {
            let message = TranslatedMessage::new(
                "error.tensor.rank",
                translation_resolver,
            )
            .key("expected", rank)
            .key("actual", self.get_rank());
            return Err(Error::new(self, message));
        }
        Ok(self)
    }

    fn validate_shape(&self, shape: &[usize]) -> Result<&Self, Error> {
        if self.shape != shape {
            let message = TranslatedMessage::new(
                "error.tensor.shape",
                translation_resolver,
            )
            .key("expected", serialize_shape(shape))
            .key("actual", serialize_shape(&self.shape));
            return Err(Error::new(self, message));
        }
        Ok(self)
    }

    fn validate_equal_shapes(&self, other: &Self) -> Result<&Self, Error> {
        if self.shape != other.shape {
            let message = TranslatedMessage::new(
                "error.tensor.equal_shape",
                translation_resolver,
            )
            .key("left", serialize_shape(&self.shape))
            .key("right", serialize_shape(&other.shape));
            return Err(Error::new(self, message));
        }
        Ok(self)
    }
}

fn serialize_shape(shape: &[usize]) -> String {
    match shape {
        [] => "0x0".to_string(),
        [dim] => format!("{dim}x1").to_string(),
        dims => dims
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join("x"),
    }
}
