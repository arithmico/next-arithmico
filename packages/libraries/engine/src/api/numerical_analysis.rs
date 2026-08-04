#[cfg(feature = "api_endpoint_numerical_analysis_nsolve")]
use crate::api::numerical_analysis::nsolve::NSolveEndpoint;
use crate::core::{HostApiModule, Language};

mod nsolve;

pub fn load_numerical_analysis_module() -> HostApiModule {
    let module = HostApiModule::builder()
        .id("numerical_analysis")
        .name(Language::English, "Numerical analysis")
        .name(Language::German, "Numerik");

    #[cfg(feature = "api_endpoint_numerical_analysis_nsolve")]
    let module = module.function::<NSolveEndpoint>();

    module.build()
}
