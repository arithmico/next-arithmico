use crate::{
    api::{
        algebra::load_algebra_module, analysis::load_analysis_module,
        distributions::load_distributions_module,
        numerical_analysis::load_numerical_analysis_module,
        physics::load_physics_module, trigonometry::load_trigonometry_module,
    },
    core::HostApi,
};

mod algebra;
mod analysis;
mod distributions;
mod numerical_analysis;
mod physics;
mod trigonometry;

pub fn load_host_api() -> HostApi {
    HostApi::builder()
        .module(
            cfg!(feature = "api_module_loader_analysis"),
            load_analysis_module,
        )
        .module(
            cfg!(feature = "api_module_loader_trigonometry"),
            load_trigonometry_module,
        )
        .module(
            cfg!(feature = "api_module_loader_distributions"),
            load_distributions_module,
        )
        .module(
            cfg!(feature = "api_module_loader_physics"),
            load_physics_module,
        )
        .module(
            cfg!(feature = "api_module_loader_numerical_analysis"),
            load_numerical_analysis_module,
        )
        .module(
            cfg!(feature = "api_module_loader_algebra"),
            load_algebra_module,
        )
        .build()
}
