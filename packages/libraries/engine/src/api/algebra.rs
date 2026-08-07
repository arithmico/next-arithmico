use translate::Language;

#[cfg(feature = "api_endpoint_algebra_cross")]
use crate::api::algebra::cross::CrossEndpoint;
#[cfg(feature = "api_endpoint_algebra_dims")]
use crate::api::algebra::dims::DimsEndpoint;
#[cfg(feature = "api_endpoint_algebra_length")]
use crate::api::algebra::length::LengthEndpoint;
#[cfg(feature = "api_endpoint_algebra_rank")]
use crate::api::algebra::rank::RankEndpoint;
use crate::core::HostApiModule;

mod cross;
mod dims;
mod length;
mod rank;

pub fn load_algebra_module() -> HostApiModule {
    let module = HostApiModule::builder()
        .id("analysis")
        .name(Language::English, "Algebra")
        .name(Language::German, "Algebra");

    #[cfg(feature = "api_endpoint_algebra_length")]
    let module = module.function::<LengthEndpoint>();

    #[cfg(feature = "api_endpoint_algebra_rank")]
    let module = module.function::<RankEndpoint>();

    #[cfg(feature = "api_endpoint_algebra_dims")]
    let module = module.function::<DimsEndpoint>();

    #[cfg(feature = "api_endpoint_algebra_cross")]
    let module = module.function::<CrossEndpoint>();

    module.build()
}
