use translate::Language;

#[cfg(feature = "api_endpoint_distributions_binom")]
use crate::api::distributions::binom::BinomEndpoint;
#[cfg(feature = "api_endpoint_distributions_cbinom")]
use crate::api::distributions::cbinom::CBinomEndpoint;
#[cfg(feature = "api_endpoint_distributions_cnormal")]
use crate::api::distributions::cnormal::CNormalEndpoint;
#[cfg(feature = "api_endpoint_distributions_normal")]
use crate::api::distributions::normal::NormalEndpoint;
#[cfg(feature = "api_endpoint_distributions_qbinom")]
use crate::api::distributions::qbinom::QBinomEndpoint;
#[cfg(feature = "api_endpoint_distributions_qnormal")]
use crate::api::distributions::qnormal::QNormalEndpoint;
use crate::core::HostApiModule;

mod binom;
mod cbinom;
mod cnormal;
mod normal;
mod qbinom;
mod qnormal;

pub fn load_distributions_module() -> HostApiModule {
    let module = HostApiModule::builder()
        .id("distributions")
        .name(Language::English, "Distributions")
        .name(Language::German, "Verteilungen");

    #[cfg(feature = "api_endpoint_distributions_normal")]
    let module = module.function::<NormalEndpoint>();

    #[cfg(feature = "api_endpoint_distributions_cnormal")]
    let module = module.function::<CNormalEndpoint>();

    #[cfg(feature = "api_endpoint_distributions_qnormal")]
    let module = module.function::<QNormalEndpoint>();

    #[cfg(feature = "api_endpoint_distributions_binom")]
    let module = module.function::<BinomEndpoint>();

    #[cfg(feature = "api_endpoint_distributions_cbinom")]
    let module = module.function::<CBinomEndpoint>();

    #[cfg(feature = "api_endpoint_distributions_qbinom")]
    let module = module.function::<QBinomEndpoint>();

    module.build()
}
