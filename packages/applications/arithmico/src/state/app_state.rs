use std::rc::Rc;

use engine::{load_host_api, Session, Settings};
use leptos::*;

use super::AppAction;

#[derive(Clone)]
pub struct AppState {
    pub session: Session,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            session: Session::new(Rc::new(load_host_api())),
        }
    }

    pub(super) fn reduce(&self, action: AppAction) -> Self {
        match action {
            AppAction::Evaluate(input) => {
                logging::log!("reduce");
                let next_session =
                    self.session.push(&input, &Settings::default());
                logging::log!("{:?}", next_session);
                AppState {
                    session: next_session,
                }
            }
        }
    }
}
