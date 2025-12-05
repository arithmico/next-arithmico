use translate_core::Language;

use crate::{
    api::miscellaneous::echo::load_echo_endpoint, core::HostApiModule,
};

mod echo;

pub fn load_miscellaneous_module() -> HostApiModule {
    HostApiModule::builder()
        .id("miscellaneous")
        .name(Language::English, "Miscellaneous")
        .name(Language::German, "Verschiedenes")
        .endpoint(
            cfg!(feature = "api_endpoint_miscellaneous_echo"),
            "echo",
            load_echo_endpoint,
        )
        .build()
}
