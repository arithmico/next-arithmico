use std::rc::Rc;

use engine::{load_host_api, HostApi, Language, Session, Statement};

#[derive(Debug, PartialEq, Clone)]
pub struct AppState {
    pub host_api: Rc<HostApi>,
    pub session: Session,
    pub settings: Settings,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Settings {
    pub decimal_places: u8,
    pub language: Language,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            decimal_places: 3,
            language: Language::English,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        let host_api = Rc::new(load_host_api());
        Self {
            host_api: host_api.clone(),
            session: Session::new(host_api),
            settings: Settings::default(),
        }
    }
}

impl AppState {
    pub fn get_last_statement(&self) -> Option<&Statement> {
        self.session.last_statement()
    }
}
