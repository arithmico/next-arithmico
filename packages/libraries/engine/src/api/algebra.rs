use evaluator::ApiModule;
use translate::Language;

#[cfg(feature = "api_endpoint_algebra_cross")]
use crate::api::algebra::cross::CrossEndpoint;
#[cfg(feature = "api_endpoint_algebra_dims")]
use crate::api::algebra::dims::DimsEndpoint;
#[cfg(feature = "api_endpoint_algebra_length")]
use crate::api::algebra::length::LengthEndpoint;
#[cfg(feature = "api_endpoint_algebra_matrix_id")]
use crate::api::algebra::matrix_id::MatrixIdEndpoint;
#[cfg(feature = "api_endpoint_algebra_rank")]
use crate::api::algebra::rank::RankEndpoint;

mod cross;
mod dims;
mod length;
mod matrix_id;
mod rank;

pub fn load_algebra_module() -> ApiModule {
    let module = ApiModule::builder()
        .id("algebra")
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

    #[cfg(feature = "api_endpoint_algebra_matrix_id")]
    let module = module.function::<MatrixIdEndpoint>();

    module.build()
}
