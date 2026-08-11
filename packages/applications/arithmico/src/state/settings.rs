use std::fmt::Debug;

use engine::{AngleUnit, DecimalPlaces, Language};
use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::window;
use override_decimal_format::OverrideDecimalFormat;
use serde::{Deserialize, Serialize};
use theme::Theme;

use crate::state::language::LanguageValue;

pub mod language;
pub mod override_decimal_format;
pub mod theme;

const SETTINGS_STORAGE_KEY: &str = "settings";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub decimal_places: DecimalPlaces,
    pub language: LanguageValue,
    pub override_decimal_format: OverrideDecimalFormat,
    pub theme: Theme,
    pub angle_unit: AngleUnit,
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

    pub fn get_language(&self) -> Language {
        if let LanguageValue::Language(language) = &self.language {
            *language
        } else {
            match window().navigator().language() {
                Some(lang) if lang == "en" => Language::English,
                Some(lang) if lang == "de" => Language::German,
                _ => Language::English,
            }
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            decimal_places: DecimalPlaces::default(),
            language: LanguageValue::System,
            override_decimal_format: OverrideDecimalFormat::new(),
            theme: Theme::System,
            angle_unit: Default::default(),
        }
    }
}
