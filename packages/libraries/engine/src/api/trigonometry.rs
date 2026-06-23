#[cfg(feature = "api_endpoint_trigonometry_cos")]
use crate::api::trigonometry::cos::CosEndpoint;
#[cfg(feature = "api_endpoint_trigonometry_sin")]
use crate::api::trigonometry::sin::SinEndpoint;
#[cfg(feature = "api_endpoint_trigonometry_tan")]
use crate::api::trigonometry::tan::TanEndpoint;
use crate::core::{HostApiModule, Language};
use pi::load_pi_endpoint;

mod cos;
mod pi;
mod sin;
mod tan;

pub fn load_trigonometry_module() -> HostApiModule {
    let module = HostApiModule::builder()
        .id("trigonometry")
        .name(Language::English, "Trigonometry")
        .name(Language::German, "Trigonometrie")
        .endpoints(&[
            #[cfg(feature = "api_endpoint_trigonometry_pi")]
            load_pi_endpoint,
        ]);

    #[cfg(feature = "api_endpoint_trigonometry_sin")]
    let module = module.function::<SinEndpoint>();

    #[cfg(feature = "api_endpoint_trigonometry_cos")]
    let module = module.function::<CosEndpoint>();

    #[cfg(feature = "api_endpoint_trigonometry_tan")]
    let module = module.function::<TanEndpoint>();

    module.build()
}
