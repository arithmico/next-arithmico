use aws_sdk_dynamodb::error::SdkError;
use derive_builder::UninitializedFieldError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("QueryBuilderError: {0:#?}")]
    QueryBuilderError(Box<UninitializedFieldError>),
    #[error("DynamoDbError: {0}")]
    DynamoDbError(String),
    #[error("SerdeError: {0:#?}")]
    Serde(#[from] serde_dynamo::Error),
    #[error("NotFoundError")]
    NotFound,
}

impl From<UninitializedFieldError> for Error {
    fn from(value: derive_builder::UninitializedFieldError) -> Self {
        Self::QueryBuilderError(value.into())
    }
}

impl<E: std::error::Error + 'static, R: std::fmt::Debug> From<SdkError<E, R>>
    for Error
{
    fn from(value: SdkError<E, R>) -> Self {
        Self::DynamoDbError(format!("{:#?}", value))
    }
}
