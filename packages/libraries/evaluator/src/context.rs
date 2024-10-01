use settings::Settings;
use stack::Stack;

mod host_api;
mod settings;
mod stack;

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    pub stack: Stack,
    pub settings: Settings,
    pub host_api: Rc<HostApi>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            stack: Stack::new(),
            settings: Settings::default(),
            host_api: load_host_api().into(),
        }
    }
}

impl Context {
    pub fn new(
        stack: Stack,
        settings: Settings,
        host_api: Rc<HostApi>,
    ) -> Context {
        Context {
            stack,
            settings,
            host_api,
        }
    }

    pub fn lookup(&self, name: &str) -> Option<Node> {
        if let Some(node) = self.stack.lookup(name) {
            return Some(node);
        }
        if let Some(endpoint) = self.host_api.endpoint(name) {
            return match endpoint {
                HostEndpoint::Function { .. } => {
                    Some(HostFunction::new(name).into())
                }
                HostEndpoint::Constant { executor, .. } => Some(executor(self)),
            };
        }
        None
    }

    pub fn endpoint(&self, name: &str) -> Option<&HostEndpoint> {
        self.host_api.endpoint(name)
    }
}
