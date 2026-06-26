use crate::{api::physics::load_physics_module, core::HostApi};
use trigonometry::load_trigonometry_module;

mod physics;
mod trigonometry;

pub fn load_host_api() -> HostApi {
    HostApi::builder()
        .module(
            cfg!(feature = "api_module_loader_trigonometry"),
            load_trigonometry_module,
        )
        .module(cfg!("api_module_physics"), load_physics_module)
        .build()
}
