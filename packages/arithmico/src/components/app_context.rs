use std::rc::Rc;

use engine::HostApi;

#[derive(Debug, PartialEq, Clone)]
pub struct AppContext {
    pub host_api: Rc<HostApi>,
}
