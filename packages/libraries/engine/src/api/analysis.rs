use translate::Language;

#[cfg(feature = "api_endpoint_analysis_exp")]
use crate::api::analysis::exp::ExpEndpoint;
#[cfg(feature = "api_endpoint_analysis_ln")]
use crate::api::analysis::ln::LnEndpoint;
#[cfg(feature = "api_endpoint_analysis_log")]
use crate::api::analysis::log::LogEndpoint;
use crate::core::HostApiModule;

mod exp;
mod ln;
mod log;

pub fn load_analysis_module() -> HostApiModule {
    let module = HostApiModule::builder()
        .id("analysis")
        .name(Language::English, "Analysis")
        .name(Language::German, "Analysis");

    #[cfg(feature = "api_endpoint_analysis_exp")]
    let module = module.function::<ExpEndpoint>();

    #[cfg(feature = "api_endpoint_analysis_log")]
    let module = module.function::<LogEndpoint>();

    #[cfg(feature = "api_endpoint_analysis_ln")]
    let module = module.function::<LnEndpoint>();

    module.build()
}
