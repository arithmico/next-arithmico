#[derive(PartialEq, Debug, Clone)]
pub struct HostApiFunctionEndpoint {
    pub name: String,
}

impl HostApiFunctionEndpoint {
    pub fn new<T: Into<String>>(name: T) -> HostApiFunctionEndpoint {
        HostApiFunctionEndpoint { name: name.into() }
    }
}
