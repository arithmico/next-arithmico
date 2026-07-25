use crate::{
    api::{
        distributions::load_distributions_module, physics::load_physics_module,
    },
    core::HostApi,
};
use trigonometry::load_trigonometry_module;

mod distributions;
mod physics;
mod trigonometry;
mod validations;

pub fn load_host_api() -> HostApi {
    HostApi::builder()
        .module(
            cfg!(feature = "api_module_loader_trigonometry"),
            load_trigonometry_module,
        )
        .module(
            cfg!(feature = "api_module_loader_physics"),
            load_physics_module,
        )
        .build()
}
