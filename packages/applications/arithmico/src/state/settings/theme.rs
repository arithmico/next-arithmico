use serde::{Deserialize, Serialize};
use translate::GetTranslationId;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Theme {
    pub fn get_class(&self, prefers_dark: bool) -> &str {
        match self {
            Theme::Light => "theme-light",
            Theme::Dark => "theme-dark",
            Theme::System => {
                if prefers_dark {
                    "theme-dark"
                } else {
                    "theme-light"
                }
            }
        }
    }
}

impl GetTranslationId for Theme {
    fn get_translation_id(&self) -> &str {
        match self {
            Theme::Light => "settings.theme.light",
            Theme::Dark => "settings.theme.dark",
            Theme::System => "settings.theme.system",
        }
    }
}
