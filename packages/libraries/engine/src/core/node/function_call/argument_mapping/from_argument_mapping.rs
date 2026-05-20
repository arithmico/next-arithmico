use crate::{core::EvaluateNodeError, ArgumentMapping};

pub trait FromArgumentMapping {
    type Mapped<'a>;

    fn from_argument_mapping<'a>(
        arguments: &'a ArgumentMapping,
    ) -> Result<Self::Mapped<'a>, EvaluateNodeError>;
}
