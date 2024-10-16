use engine::{EvaluateNodeOptions, Session};
use gloo_storage::{LocalStorage, Storage};

use super::{settings::Settings, AppAction};

#[derive(Debug, Clone)]
pub struct AppState {
    pub session: Session,
    pub settings: Settings,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            session: Session::new(),
            settings: Settings::default(),
        }
    }
}

const SETTINGS_STORAGE_KEY: &str = "settings";

impl AppState {
    pub fn load_or_default() -> Self {
        if let Ok(settings) = LocalStorage::get::<String>(SETTINGS_STORAGE_KEY)
        {
            if let Ok(settings) = serde_json::from_str::<Settings>(&settings) {
                return Self {
                    session: Session::new(),
                    settings,
                };
            }
        }
        AppState::default()
    }

    fn save(&self) {
        LocalStorage::set(
            SETTINGS_STORAGE_KEY,
            serde_json::to_string(&self.settings).expect("serialized settings"),
        )
        .expect("localstorage");
    }

    pub(super) fn reduce(&mut self, action: AppAction) {
        match action {
            AppAction::Evaluate(input) => {
                self.session.push(&input, &EvaluateNodeOptions::default());
            }
            AppAction::SetLanguage(language) => {
                self.settings.language = language;
                self.save();
            }
        }
    }
}
