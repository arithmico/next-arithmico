use std::rc::Rc;

use engine::{load_host_api, HostApi, Language, Session, Statement};
use serde::{Deserialize, Serialize};
use web_sys::Storage;

#[derive(Debug, PartialEq, Clone)]
pub struct AppState {
    pub host_api: Rc<HostApi>,
    pub session: Session,
    pub settings: Settings,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub decimal_places: u8,
    pub language: Language,
}

impl Settings {
    const STORAGE_KEY: &'static str = "settings";

    fn get_local_storage() -> Option<Storage> {
        let window = web_sys::window()?;
        window.local_storage().ok()?
    }

    pub fn save(&self) -> Option<()> {
        let storage = Self::get_local_storage()?;
        storage
            .set_item(
                Self::STORAGE_KEY,
                serde_json::to_string(&self).unwrap().as_str(),
            )
            .ok()?;
        Some(())
    }

    pub fn load() -> Option<Self> {
        let storage = Self::get_local_storage()?;
        let item = storage.get_item(Self::STORAGE_KEY).ok()??;
        serde_json::from_str(&item).ok()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            decimal_places: 3,
            language: Language::English,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        let host_api = Rc::new(load_host_api());
        Self {
            host_api: host_api.clone(),
            session: Session::new(host_api),
            settings: Settings::load().unwrap_or_else(|| Settings::default()),
        }
    }

    pub fn get_last_statement(&self) -> Option<&Statement> {
        self.session.last_statement()
    }
}
