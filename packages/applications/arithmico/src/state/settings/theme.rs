use serde::{Deserialize, Serialize};
use translate::IntoTranslationId;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Theme {
    pub fn resolve_html_class(&self, prefers_dark: bool) -> &str {
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

    /*
    pub fn get_class(&self) -> &str {
        match self {
            Theme::Light => "theme-light",
            Theme::Dark => "theme-dark",
            Theme::System => evaluate_theme(),
        }
    }
     */
}

impl IntoTranslationId for Theme {
    fn into_translation_id(&self) -> &str {
        match self {
            Theme::Light => "settings.theme.light",
            Theme::Dark => "settings.theme.dark",
            Theme::System => "settings.theme.system",
        }
    }
}
