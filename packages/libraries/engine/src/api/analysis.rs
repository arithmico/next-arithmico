use translate::Language;

#[cfg(feature = "api_endpoint_analysis_abs")]
use crate::api::analysis::abs::AbsEndpoint;
#[cfg(feature = "api_endpoint_analysis_ceil")]
use crate::api::analysis::ceil::CeilEndpoint;
#[cfg(feature = "api_endpoint_analysis_e")]
use crate::api::analysis::e::EEndpoint;
#[cfg(feature = "api_endpoint_analysis_exp")]
use crate::api::analysis::exp::ExpEndpoint;
#[cfg(feature = "api_endpoint_analysis_floor")]
use crate::api::analysis::floor::FloorEndpoint;
#[cfg(feature = "api_endpoint_analysis_lg")]
use crate::api::analysis::lg::LgEndpoint;
#[cfg(feature = "api_endpoint_analysis_ln")]
use crate::api::analysis::ln::LnEndpoint;
#[cfg(feature = "api_endpoint_analysis_log")]
use crate::api::analysis::log::LogEndpoint;
#[cfg(feature = "api_endpoint_analysis_min")]
use crate::api::analysis::min::MinEndpoint;
#[cfg(feature = "api_endpoint_analysis_root")]
use crate::api::analysis::root::RootEndpoint;
#[cfg(feature = "api_endpoint_analysis_round")]
use crate::api::analysis::round::RoundEndpoint;
#[cfg(feature = "api_endpoint_analysis_sqrt")]
use crate::api::analysis::sqrt::SqrtEndpoint;
use crate::core::HostApiModule;

mod abs;
mod ceil;
mod e;
mod exp;
mod floor;
mod lg;
mod ln;
mod log;
mod min;
mod root;
mod round;
mod sqrt;

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

    #[cfg(feature = "api_endpoint_analysis_lg")]
    let module = module.function::<LgEndpoint>();

    #[cfg(feature = "api_endpoint_analysis_e")]
    let module = module.constant::<EEndpoint>();

    #[cfg(feature = "api_endpoint_analysis_sqrt")]
    let module = module.function::<SqrtEndpoint>();

    #[cfg(feature = "api_endpoint_analysis_root")]
    let module = module.function::<RootEndpoint>();

    #[cfg(feature = "api_endpoint_analysis_round")]
    let module = module.function::<RoundEndpoint>();

    #[cfg(feature = "api_endpoint_analysis_floor")]
    let module = module.function::<FloorEndpoint>();

    #[cfg(feature = "api_endpoint_analysis_ceil")]
    let module = module.function::<CeilEndpoint>();

    #[cfg(feature = "api_endpoint_analysis_abs")]
    let module = module.function::<AbsEndpoint>();

    #[cfg(feature = "api_endpoint_analysis_min")]
    let module = module.function::<MinEndpoint>();

    module.build()
}
