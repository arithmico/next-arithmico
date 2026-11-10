use aws_sdk_dynamodb::error::SdkError;
use derive_builder::UninitializedFieldError;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("DatabaseError")]
pub enum Error {
    QueryBuilderError(Box<UninitializedFieldError>),
    DynamoDbError(String),
    Serde(#[from] serde_dynamo::Error),
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
