use translate::Language;

#[cfg(feature = "api_endpoint_algebra_length")]
use crate::api::algebra::length::LengthEndpoint;
use crate::core::HostApiModule;

mod length;

pub fn load_algebra_module() -> HostApiModule {
    let module = HostApiModule::builder()
        .id("analysis")
        .name(Language::English, "Algebra")
        .name(Language::German, "Algebra");

    #[cfg(feature = "api_endpoint_algebra_length")]
    let module = module.function::<LengthEndpoint>();

    module.build()
}
