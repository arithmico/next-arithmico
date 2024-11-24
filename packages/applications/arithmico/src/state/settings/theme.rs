use serde::{Deserialize, Serialize};
use translate::IntoTranslationId;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum Theme {
    Light,
    Dark,
}

impl IntoTranslationId for Theme {
    fn into_translation_id(&self) -> &str {
        match self {
            Theme::Light => "settings.theme.light",
            Theme::Dark => "settings.theme.dark",
        }
    }
}
