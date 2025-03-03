use engine::Session;
use web_state::WebState;

use super::Settings;

#[derive(Clone)]
pub struct State {
    pub session: Session,
    pub settings: Settings,
}

impl State {
    pub fn new() -> Self {
        Self {
            session: Session::new(),
            settings: Settings::load(),
        }
    }
}

impl WebState for State {}
