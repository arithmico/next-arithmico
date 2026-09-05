use common::Language;
use evaluator::ApiModule;

#[cfg(feature = "api_endpoint_discrete_mathematics_binco")]
use crate::api::discrete_mathematics::binco::BincoEndpoint;
#[cfg(feature = "api_endpoint_discrete_mathematics_fact")]
use crate::api::discrete_mathematics::fact::FactEndpoint;
#[cfg(feature = "api_endpoint_discrete_mathematics_fib")]
use crate::api::discrete_mathematics::fib::FibEndpoint;
#[cfg(feature = "api_endpoint_discrete_mathematics_gcd")]
use crate::api::discrete_mathematics::gcd::GCDEndpoint;
#[cfg(feature = "api_endpoint_discrete_mathematics_idiv")]
use crate::api::discrete_mathematics::idiv::IDivEndpoint;
#[cfg(feature = "api_endpoint_discrete_mathematics_lcm")]
use crate::api::discrete_mathematics::lcm::LCMEndpoint;
#[cfg(feature = "api_endpoint_discrete_mathematics_mod")]
use crate::api::discrete_mathematics::modulus::ModEndpoint;
#[cfg(feature = "api_endpoint_discrete_mathematics_perm")]
use crate::api::discrete_mathematics::perm::PermEndpoint;

mod binco;
mod fact;
mod fib;
mod gcd;
mod idiv;
mod lcm;
mod modulus;
mod perm;

pub fn load_discrete_mathematics_module() -> ApiModule {
    let module = ApiModule::builder()
        .id("discrete mathematics")
        .name(Language::English, "Discrete mathematics")
        .name(Language::German, "Diskrete Mathematik");

    #[cfg(feature = "api_endpoint_discrete_mathematics_mod")]
    let module = module.function::<ModEndpoint>();

    #[cfg(feature = "api_endpoint_discrete_mathematics_idiv")]
    let module = module.function::<IDivEndpoint>();

    #[cfg(feature = "api_endpoint_discrete_mathematics_gcd")]
    let module = module.function::<GCDEndpoint>();

    #[cfg(feature = "api_endpoint_discrete_mathematics_lcm")]
    let module = module.function::<LCMEndpoint>();

    #[cfg(feature = "api_endpoint_discrete_mathematics_binco")]
    let module = module.function::<BincoEndpoint>();

    #[cfg(feature = "api_endpoint_discrete_mathematics_perm")]
    let module = module.function::<PermEndpoint>();

    #[cfg(feature = "api_endpoint_discrete_mathematics_fact")]
    let module = module.function::<FactEndpoint>();

    #[cfg(feature = "api_endpoint_discrete_mathematics_fib")]
    let module = module.function::<FibEndpoint>();

    module.build()
}
