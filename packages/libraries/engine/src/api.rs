use crate::{api::miscellaneous::load_miscellaneous_module, core::HostApi};
use trigonometry::load_trigonometry_module;

mod miscellaneous;
mod trigonometry;

pub fn load_host_api() -> HostApi {
    HostApi::builder()
        .module(
            cfg!(feature = "api_module_loader_trigonometry"),
            load_trigonometry_module,
        )
        .module(
            cfg!(feature = "api_module_loader_miscellaneous"),
            load_miscellaneous_module,
        )
        .build()
}
