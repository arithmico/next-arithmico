use std::rc::Rc;

use engine::{load_host_api, Session};

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
}
