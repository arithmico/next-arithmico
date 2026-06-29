use translate::Language;

use crate::{api::distributions::{cnormal::CNormalEndpoint, normal::NormalEndpoint}, core::HostApiModule};

mod normal;
mod cnormal;
mod utils;

pub fn load_distributions_module() -> HostApiModule {
    let module = HostApiModule::builder()
        .id("distributions")
        .name(Language::English, "Distributions")
        .name(Language::German, "Verteilungen");

    let module = module.function::<NormalEndpoint>();

    let module = module.function::<CNormalEndpoint>();

    module.build()
}
