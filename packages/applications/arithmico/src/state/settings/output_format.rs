use serde::{Deserialize, Serialize};
use translate::IntoTranslationId;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum OutputFormat {
    Text,
    MathMl,
}

impl IntoTranslationId for OutputFormat {
    fn into_translation_id(&self) -> &str {
        match self {
            OutputFormat::Text => "settings.output_format.text",
            OutputFormat::MathMl => "settings.output_format.mathml",
        }
    }
}
