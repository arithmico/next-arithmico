use std::rc::Rc;

use engine::{load_host_api, HostApi, Session};

#[derive(Debug, PartialEq, Clone)]
pub struct AppContext {
    pub host_api: Rc<HostApi>,
    pub session: Session,
}

impl Default for AppContext {
    fn default() -> Self {
        let host_api = Rc::new(load_host_api());
        Self {
            host_api: host_api.clone(),
            session: Session::new(host_api),
        }
    }
}
