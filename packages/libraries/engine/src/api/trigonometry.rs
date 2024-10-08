use common::{HostApiModule, Language};
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
        .endpoint(
            cfg!(feature = "api_endpoint_trigonometry_pi"),
            "pi",
            load_pi_endpoint,
        )
        .endpoint(
            cfg!(feature = "api_endpoint_trigonometry_sin"),
            "sin",
            load_sin_endpoint,
        )
        .endpoint(
            cfg!(feature = "api_endpoint_trigonometry_cos"),
            "cos",
            load_cos_endpoint,
        )
        .build()
}
