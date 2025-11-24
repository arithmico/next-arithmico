use serde::{Deserialize, Serialize};
use translate::IntoTranslationId;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum Theme {
    Light,
    Dark,
    Default,
}

impl Theme {
    pub fn get_class(&self) -> &str {
        match self {
            Theme::Light => "theme-light",
            Theme::Dark => "theme-dark",
            _ => "",
        }
    }
}

impl IntoTranslationId for Theme {
    fn into_translation_id(&self) -> &str {
        match self {
            Theme::Light => "settings.theme.light",
            Theme::Dark => "settings.theme.dark",
            Theme::Default => "settings.theme.default",
        }
    }
}
