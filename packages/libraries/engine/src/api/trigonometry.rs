use crate::core::{HostApiModule, Language};
use cos::load_cos_endpoint;
use pi::load_pi_endpoint;
use sin::load_sin_endpoint;

mod cos;
mod pi;
mod sin;

pub fn load_trigonometry_module() -> HostApiModule {
    HostApiModule::builder()
        .id("trigonometry")
        .name(Language::English, "Trigonometry")
        .name(Language::German, "Trigonometrie")
        .endpoints(&[
            #[cfg(feature = "api_endpoint_trigonometry_pi")]
            load_pi_endpoint,
            #[cfg(feature = "api_endpoint_trigonometry_sin")]
            load_sin_endpoint,
            #[cfg(feature = "api_endpoint_trigonometry_cos")]
            load_cos_endpoint,
        ])
        .build()
}
