use engine::{Session, SessionError};
use web_state::WebState;

use super::Settings;

#[derive(Clone)]
pub struct State {
    pub session: Session,
    pub settings: Settings,
    pub current_output: Option<Result<String, SessionError>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            session: Session::new(),
            settings: Settings::load(),
            current_output: None,
        }
    }
}

impl WebState for State {}
