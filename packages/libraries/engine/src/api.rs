use evaluator::Api;

use crate::api::{
    algebra::load_algebra_module, analysis::load_analysis_module,
    descriptive_statistics::load_descriptive_statistics_module,
    discrete_mathematics::load_discrete_mathematics_module,
    distributions::load_distributions_module,
    numerical_analysis::load_numerical_analysis_module,
    physics::load_physics_module, trigonometry::load_trigonometry_module,
};

mod algebra;
mod analysis;
mod descriptive_statistics;
mod discrete_mathematics;
mod distributions;
mod numerical_analysis;
mod physics;
mod trigonometry;

pub fn load_host_api() -> Api {
    Api::builder()
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
        .module(
            cfg!(feature = "api_module_loader_descriptive_statistics"),
            load_descriptive_statistics_module,
        )
        .module(
            cfg!(feature = "api_module_loader_discrete_mathematics"),
            load_discrete_mathematics_module,
        )
        .build()
}
