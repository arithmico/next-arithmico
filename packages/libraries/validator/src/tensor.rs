use node::Tensor;
use translate_core::TranslatedMessage;

use crate::{Error, translations::translation_resolver};

/// A trait for validating structural properties of a tensor.
///
/// All methods return `Result<&Self, Error>`, which allows for fluent
/// method chaining when applying multiple validation checks in a row.
pub trait TensorValidator {
    /// Validates that the tensor contains at least one element.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the tensor is empty (e.g., has a size of 0).
    fn validate_not_empty(&self) -> Result<&Self, Error>;

    /// Validates that the tensor has the specified rank (number of dimensions).
    ///
    /// # Arguments
    ///
    /// * `rank` - The expected number of dimensions the tensor should have.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the tensor's actual rank does not match the provided `rank`.
    fn validate_rank(&self, rank: usize) -> Result<&Self, Error>;

    /// Validates that the tensor's shape exactly matches the given shape.
    ///
    /// # Arguments
    ///
    /// * `shape` - A slice representing the expected sizes of each dimension.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the tensor's shape does not strictly match the
    /// provided `shape` slice in both rank and dimension sizes.
    fn validate_shape(&self, shape: &[usize]) -> Result<&Self, Error>;

    /// Validates that this tensor has the exact same shape as another tensor.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another tensor to compare shapes against.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the shape of `self` does not match the shape of `other`.
    fn validate_equal_shapes(&self, other: &Self) -> Result<&Self, Error>;

    /// Validates that the tensor is a vector of the specified size.
    ///
    /// This method asserts two properties:
    /// 1. The tensor is a vector (i.e., it has a rank of 1).
    /// 2. The length (or size) of this vector matches the provided `dimension`
    ///    (e.g., providing `3` ensures the tensor is a 3-element vector).
    ///
    /// # Arguments
    ///
    /// * `dimension` - The expected size (number of elements) of the vector.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the tensor's rank is not exactly 1, or if the
    /// size of the vector does not match the specified `dimension`.
    fn validate_vector_dimension(
        &self,
        dimension: usize,
    ) -> Result<&Self, Error>;

    /// Validates that the tensor is a vector of the specified size.
    ///
    /// This method asserts two properties:
    /// 1. The tensor is a vector (i.e., it has a rank of 1).
    /// 2. The length (or size) of this vector is at least the provided `dimension`
    ///    (e.g., providing `3` ensures that the vector has a size of at least 3 elements).
    ///
    /// # Arguments
    ///
    /// * `dimension` - The expected minimal size (number of elements) of the vector.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the tensor's rank is not exactly 1, or if the
    /// size of the vector does not match the specified `dimension`.
    fn validate_minimum_vector_length(
        &self,
        dimension: usize,
    ) -> Result<&Self, Error>;

    fn validate_square_matrix(&self) -> Result<&Self, Error>;

    fn validate_matrix(&self) -> Result<&Self, Error>;
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

    fn validate_vector_dimension(
        &self,
        dimension: usize,
    ) -> Result<&Self, Error> {
        match self.shape[..] {
            [dim] if dim == dimension => Ok(self),
            _ => {
                let message = TranslatedMessage::new(
                    "error.tensor.vector_dimension",
                    translation_resolver,
                )
                .key("dim", dimension);
                Err(Error::new(self, message))
            }
        }
    }

    fn validate_minimum_vector_length(
        &self,
        dimension: usize,
    ) -> Result<&Self, Error> {
        match self.shape[..] {
            [dim] if dim >= dimension => Ok(self),
            _ => {
                let message = TranslatedMessage::new(
                    "error.tensor.vector_min_dimension",
                    translation_resolver,
                )
                .key("dim", dimension);
                Err(Error::new(self, message))
            }
        }
    }

    fn validate_square_matrix(&self) -> Result<&Self, Error> {
        match self.shape[..] {
            [rows, columns] if rows == columns => Ok(self),
            _ => {
                let message = TranslatedMessage::new(
                    "error.tensor.square_matrix",
                    translation_resolver,
                )
                .key("shape", serialize_shape(&self.shape));
                Err(Error::new(self, message))
            }
        }
    }

    fn validate_matrix(&self) -> Result<&Self, Error> {
        match self.shape.len() {
            2 => Ok(self),
            _ => {
                let message = TranslatedMessage::new(
                    "error.tensor.matrix",
                    translation_resolver,
                )
                .key("shape", serialize_shape(&self.shape));
                Err(Error::new(self, message))
            }
        }
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
