use common::HostApi;
use trigonometry::load_trigonometry_module;

mod trigonometry;

pub fn load_host_api() -> HostApi {
    HostApi::builder()
        .module(
            cfg!(feature = "api_module_loader_trigonometry"),
            load_trigonometry_module,
        )
        .build()
}
