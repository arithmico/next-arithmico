use engine::{Session, SessionError};
use trace::Trace;
use web_state::WebState;

use super::Settings;

#[derive(Clone)]
pub struct State {
    pub session: Session,
    pub settings: Settings,
    pub current_output: Option<Result<String, SessionError>>,
    pub current_error_trace: Option<Trace>,
}

impl State {
    pub fn new() -> Self {
        Self {
            session: Session::new(),
            settings: Settings::load(),
            current_output: None,
            current_error_trace: None,
        }
    }
}

impl WebState for State {}
