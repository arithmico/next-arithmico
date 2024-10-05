use engine::{EvaluateNodeOptions, Session};

use super::AppAction;

#[derive(Clone)]
pub struct AppState {
    pub session: Session,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            session: Session::new(),
        }
    }

    pub(super) fn reduce(&mut self, action: AppAction) {
        match action {
            AppAction::Evaluate(input) => {
                self.session.push(&input, &EvaluateNodeOptions::default());
            }
        }
    }
}
