use std::fmt::Debug;

use engine::{DecimalPlaces, Language};
use gloo_storage::{LocalStorage, Storage};
use override_decimal_format::OverrideDecimalFormat;
use serde::{Deserialize, Serialize};
use theme::Theme;

pub mod override_decimal_format;
pub mod theme;

const SETTINGS_STORAGE_KEY: &str = "settings";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub decimal_places: DecimalPlaces,
    pub language: Language,
    pub override_decimal_format: OverrideDecimalFormat,
    pub theme: Theme,
}

impl Settings {
    pub fn load() -> Self {
        Self::load_from_localstorage().unwrap_or_default()
    }

    fn load_from_localstorage() -> Option<Self> {
        let settings =
            LocalStorage::get::<String>(SETTINGS_STORAGE_KEY).ok()?;

        serde_json::from_str::<Settings>(&settings).ok()
    }

    pub fn save(&self) {
        LocalStorage::set(
            SETTINGS_STORAGE_KEY,
            serde_json::to_string(&self).expect("serialized settings"),
        )
        .expect("localstorage");
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            decimal_places: DecimalPlaces::from(5),
            language: Language::English,
            override_decimal_format: OverrideDecimalFormat::new(),
            theme: Theme::System,
        }
    }
}
