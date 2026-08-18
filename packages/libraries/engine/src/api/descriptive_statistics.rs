use common::Language;
use evaluator::ApiModule;

#[cfg(feature = "api_endpoint_descriptive_statistics_avg")]
use crate::api::descriptive_statistics::avg::AvgEndpoint;
#[cfg(feature = "api_endpoint_descriptive_statistics_corr")]
use crate::api::descriptive_statistics::corr::CorrEndpoint;
#[cfg(feature = "api_endpoint_descriptive_statistics_cov")]
use crate::api::descriptive_statistics::cov::CovEndpoint;
#[cfg(feature = "api_endpoint_descriptive_statistics_max")]
use crate::api::descriptive_statistics::max::MaxEndpoint;
#[cfg(feature = "api_endpoint_descriptive_statistics_median")]
use crate::api::descriptive_statistics::median::MedianEndpoint;
#[cfg(feature = "api_endpoint_descriptive_statistics_min")]
use crate::api::descriptive_statistics::min::MinEndpoint;
#[cfg(feature = "api_endpoint_descriptive_statistics_quantile")]
use crate::api::descriptive_statistics::quantile::QuantileEndpoint;
#[cfg(feature = "api_endpoint_descriptive_statistics_sd")]
use crate::api::descriptive_statistics::sd::SdEndpoint;
#[cfg(feature = "api_endpoint_descriptive_statistics_unbiased_sd")]
use crate::api::descriptive_statistics::unbiased_sd::UnbiasedSdEndpoint;
#[cfg(feature = "api_endpoint_descriptive_statistics_unbiased_var")]
use crate::api::descriptive_statistics::unbiased_var::UnbiasedVarEndpoint;
#[cfg(feature = "api_endpoint_descriptive_statistics_var")]
use crate::api::descriptive_statistics::var::VarEndpoint;

mod avg;
mod corr;
mod cov;
mod max;
mod median;
mod min;
mod quantile;
mod sd;
mod unbiased_sd;
mod unbiased_var;
mod var;

pub fn load_descriptive_statistics_module() -> ApiModule {
    let module = ApiModule::builder()
        .id("descriptive")
        .name(Language::English, "Descriptive statistics")
        .name(Language::German, "Deskriptive Statistik");

    #[cfg(feature = "api_endpoint_descriptive_statistics_min")]
    let module = module.function::<MinEndpoint>();

    #[cfg(feature = "api_endpoint_descriptive_statistics_max")]
    let module = module.function::<MaxEndpoint>();

    #[cfg(feature = "api_endpoint_descriptive_statistics_avg")]
    let module = module.function::<AvgEndpoint>();

    #[cfg(feature = "api_endpoint_descriptive_statistics_var")]
    let module = module.function::<VarEndpoint>();

    #[cfg(feature = "api_endpoint_descriptive_statistics_unbiased_var")]
    let module = module.function::<UnbiasedVarEndpoint>();

    #[cfg(feature = "api_endpoint_descriptive_statistics_sd")]
    let module = module.function::<SdEndpoint>();

    #[cfg(feature = "api_endpoint_descriptive_statistics_unbiased_sd")]
    let module = module.function::<UnbiasedSdEndpoint>();

    #[cfg(feature = "api_endpoint_descriptive_statistics_median")]
    let module = module.function::<MedianEndpoint>();

    #[cfg(feature = "api_endpoint_descriptive_statistics_quantile")]
    let module = module.function::<QuantileEndpoint>();

    #[cfg(feature = "api_endpoint_descriptive_statistics_cov")]
    let module = module.function::<CovEndpoint>();

    #[cfg(feature = "api_endpoint_descriptive_statistics_corr")]
    let module = module.function::<CorrEndpoint>();

    module.build()
}
