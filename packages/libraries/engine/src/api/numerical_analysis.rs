use evaluator::ApiModule;
use translate::Language;

#[cfg(feature = "api_endpoint_numerical_analysis_nderive")]
use crate::api::numerical_analysis::nderive::NDeriveEndpoint;
#[cfg(feature = "api_endpoint_numerical_analysis_nintegrate")]
use crate::api::numerical_analysis::nintegrate::NIntegrateEndpoint;
#[cfg(feature = "api_endpoint_numerical_analysis_nsolve")]
use crate::api::numerical_analysis::nsolve::NSolveEndpoint;

mod nderive;
mod nintegrate;
mod nsolve;

pub fn load_numerical_analysis_module() -> ApiModule {
    let module = ApiModule::builder()
        .id("numerical_analysis")
        .name(Language::English, "Numerical analysis")
        .name(Language::German, "Numerik");

    #[cfg(feature = "api_endpoint_numerical_analysis_nsolve")]
    let module = module.function::<NSolveEndpoint>();

    #[cfg(feature = "api_endpoint_numerical_analysis_nintegrate")]
    let module = module.function::<NIntegrateEndpoint>();

    #[cfg(feature = "api_endpoint_numerical_analysis_nderive")]
    let module = module.function::<NDeriveEndpoint>();

    module.build()
}
