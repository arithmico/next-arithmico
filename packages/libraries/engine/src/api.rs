#![allow(dead_code)]

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
        .module(load_analysis_module)
        .module(load_trigonometry_module)
        .module(load_distributions_module)
        .module(load_physics_module)
        .module(load_numerical_analysis_module)
        .module(load_algebra_module)
        .module(load_descriptive_statistics_module)
        .module(load_discrete_mathematics_module)
        .build()
}
