use crate::context::HostApi;

use super::trigonometry::load_trigonometry_module;

pub fn load_host_api() -> HostApi {
    HostApi::builder().module(load_trigonometry_module).build()
}
