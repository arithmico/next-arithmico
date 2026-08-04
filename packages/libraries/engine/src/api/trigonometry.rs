#[cfg(feature = "api_endpoint_trigonometry_acos")]
use crate::api::trigonometry::acos::AcosEndpoint;
#[cfg(feature = "api_endpoint_trigonometry_asin")]
use crate::api::trigonometry::asin::AsinEndpoint;
#[cfg(feature = "api_endpoint_trigonometry_atan")]
use crate::api::trigonometry::atan::AtanEndpoint;
#[cfg(feature = "api_endpoint_trigonometry_cos")]
use crate::api::trigonometry::cos::CosEndpoint;
#[cfg(feature = "api_endpoint_trigonometry_cosh")]
use crate::api::trigonometry::cosh::CoshEndpoint;
#[cfg(feature = "api_endpoint_trigonometry_pi")]
use crate::api::trigonometry::pi::PiEndpoint;
#[cfg(feature = "api_endpoint_trigonometry_sin")]
use crate::api::trigonometry::sin::SinEndpoint;
#[cfg(feature = "api_endpoint_trigonometry_sinh")]
use crate::api::trigonometry::sinh::SinhEndpoint;
#[cfg(feature = "api_endpoint_trigonometry_tan")]
use crate::api::trigonometry::tan::TanEndpoint;
use crate::core::{HostApiModule, Language};

mod acos;
mod asin;
mod atan;
mod cos;
mod cosh;
mod pi;
mod sin;
mod sinh;
mod tan;

pub fn load_trigonometry_module() -> HostApiModule {
    let module = HostApiModule::builder()
        .id("trigonometry")
        .name(Language::English, "Trigonometry")
        .name(Language::German, "Trigonometrie");

    #[cfg(feature = "api_endpoint_trigonometry_pi")]
    let module = module.constant::<PiEndpoint>();

    #[cfg(feature = "api_endpoint_trigonometry_sin")]
    let module = module.function::<SinEndpoint>();

    #[cfg(feature = "api_endpoint_trigonometry_cos")]
    let module = module.function::<CosEndpoint>();

    #[cfg(feature = "api_endpoint_trigonometry_tan")]
    let module = module.function::<TanEndpoint>();

    #[cfg(feature = "api_endpoint_trigonometry_asin")]
    let module = module.function::<AsinEndpoint>();

    #[cfg(feature = "api_endpoint_trigonometry_acos")]
    let module = module.function::<AcosEndpoint>();

    #[cfg(feature = "api_endpoint_trigonometry_atan")]
    let module = module.function::<AtanEndpoint>();

    #[cfg(feature = "api_endpoint_trigonometry_sinh")]
    let module = module.function::<SinhEndpoint>();

    #[cfg(feature = "api_endpoint_trigonometry_cosh")]
    let module = module.function::<CoshEndpoint>();

    module.build()
}
