use evaluator::ApiModule;
use translate_core::Language;

mod constants;

#[cfg(feature = "api_endpoint_physics_a_0")]
use constants::A0Endpoint;
#[cfg(feature = "api_endpoint_physics_atm")]
use constants::ATMEndpoint;
#[cfg(feature = "api_endpoint_physics_alpha")]
use constants::AlphaEndpoint;
#[cfg(feature = "api_endpoint_physics_c_1")]
use constants::C1Endpoint;
#[cfg(feature = "api_endpoint_physics_c_2")]
use constants::C2Endpoint;
#[cfg(feature = "api_endpoint_physics_c")]
use constants::CEndpoint;
#[cfg(feature = "api_endpoint_physics_e")]
use constants::EEndpoint;
#[cfg(feature = "api_endpoint_physics_epsilon_0")]
use constants::Epsilon0Endpoint;
#[cfg(feature = "api_endpoint_physics_f")]
use constants::FEndpoint;
#[cfg(feature = "api_endpoint_physics_g_0")]
use constants::G0Endpoint;
#[cfg(feature = "api_endpoint_physics_g")]
use constants::GEndpoint;
#[cfg(feature = "api_endpoint_physics_g_standard")]
use constants::GNEndpoint;
#[cfg(feature = "api_endpoint_physics_gamma_p")]
use constants::GammaPEndpoint;
#[cfg(feature = "api_endpoint_physics_hbar")]
use constants::HBarEndpoint;
#[cfg(feature = "api_endpoint_physics_h")]
use constants::HEndpoint;
#[cfg(feature = "api_endpoint_physics_k")]
use constants::KEndpoint;
#[cfg(feature = "api_endpoint_physics_lambda_c")]
use constants::LambdaCEndpoint;
#[cfg(feature = "api_endpoint_physics_lambda_cn")]
use constants::LambdaCNEndpoint;
#[cfg(feature = "api_endpoint_physics_lambda_cp")]
use constants::LambdaCPEndpoint;
#[cfg(feature = "api_endpoint_physics_m_e")]
use constants::MEEndpoint;
#[cfg(feature = "api_endpoint_physics_m_mu")]
use constants::MMuEndpoint;
#[cfg(feature = "api_endpoint_physics_m_n")]
use constants::MNEndpoint;
#[cfg(feature = "api_endpoint_physics_m_p")]
use constants::MPEndpoint;
#[cfg(feature = "api_endpoint_physics_mu_0")]
use constants::Mu0Endpoint;
#[cfg(feature = "api_endpoint_physics_mu_b")]
use constants::MuBEndpoint;
#[cfg(feature = "api_endpoint_physics_mu_e")]
use constants::MuEEndpoint;
#[cfg(feature = "api_endpoint_physics_mu_mu")]
use constants::MuMuEndpoint;
#[cfg(feature = "api_endpoint_physics_mu_nucleus")]
use constants::MuNEndpoint;
#[cfg(feature = "api_endpoint_physics_mu_n")]
use constants::MuNNEndpoint;
#[cfg(feature = "api_endpoint_physics_mu_p")]
use constants::MuPEndpoint;
#[cfg(feature = "api_endpoint_physics_n_a")]
use constants::NAEndpoint;
#[cfg(feature = "api_endpoint_physics_phi_0")]
use constants::Phi0Endpoint;
#[cfg(feature = "api_endpoint_physics_r_e")]
use constants::REEndpoint;
#[cfg(feature = "api_endpoint_physics_r")]
use constants::REndpoint;
#[cfg(feature = "api_endpoint_physics_r_inf")]
use constants::RInfEndpoint;
#[cfg(feature = "api_endpoint_physics_sigma")]
use constants::SigmaEndpoint;
#[cfg(feature = "api_endpoint_physics_t")]
use constants::TEndpoint;
#[cfg(feature = "api_endpoint_physics_u")]
use constants::UEndpoint;
#[cfg(feature = "api_endpoint_physics_v_m")]
use constants::VMEndpoint;
#[cfg(feature = "api_endpoint_physics_z_0")]
use constants::Z0Endpoint;

pub fn load_physics_module() -> ApiModule {
    let module = ApiModule::builder()
        .id("physics")
        .name(Language::English, "Physics")
        .name(Language::German, "Physik");

    #[cfg(feature = "api_endpoint_physics_m_p")]
    let module = module.constant::<MPEndpoint>();

    #[cfg(feature = "api_endpoint_physics_m_n")]
    let module = module.constant::<MNEndpoint>();

    #[cfg(feature = "api_endpoint_physics_m_e")]
    let module = module.constant::<MEEndpoint>();

    #[cfg(feature = "api_endpoint_physics_m_mu")]
    let module = module.constant::<MMuEndpoint>();

    #[cfg(feature = "api_endpoint_physics_a_0")]
    let module = module.constant::<A0Endpoint>();

    #[cfg(feature = "api_endpoint_physics_h")]
    let module = module.constant::<HEndpoint>();

    #[cfg(feature = "api_endpoint_physics_mu_nucleus")]
    let module = module.constant::<MuNEndpoint>();

    #[cfg(feature = "api_endpoint_physics_mu_b")]
    let module = module.constant::<MuBEndpoint>();

    #[cfg(feature = "api_endpoint_physics_hbar")]
    let module = module.constant::<HBarEndpoint>();

    #[cfg(feature = "api_endpoint_physics_alpha")]
    let module = module.constant::<AlphaEndpoint>();

    #[cfg(feature = "api_endpoint_physics_r_e")]
    let module = module.constant::<REEndpoint>();

    #[cfg(feature = "api_endpoint_physics_lambda_c")]
    let module = module.constant::<LambdaCEndpoint>();

    #[cfg(feature = "api_endpoint_physics_gamma_p")]
    let module = module.constant::<GammaPEndpoint>();

    #[cfg(feature = "api_endpoint_physics_lambda_cp")]
    let module = module.constant::<LambdaCPEndpoint>();

    #[cfg(feature = "api_endpoint_physics_lambda_cn")]
    let module = module.constant::<LambdaCNEndpoint>();

    #[cfg(feature = "api_endpoint_physics_r_inf")]
    let module = module.constant::<RInfEndpoint>();

    #[cfg(feature = "api_endpoint_physics_u")]
    let module = module.constant::<UEndpoint>();

    #[cfg(feature = "api_endpoint_physics_mu_p")]
    let module = module.constant::<MuPEndpoint>();

    #[cfg(feature = "api_endpoint_physics_mu_e")]
    let module = module.constant::<MuEEndpoint>();

    #[cfg(feature = "api_endpoint_physics_mu_n")]
    let module = module.constant::<MuNNEndpoint>();

    #[cfg(feature = "api_endpoint_physics_mu_mu")]
    let module = module.constant::<MuMuEndpoint>();

    #[cfg(feature = "api_endpoint_physics_f")]
    let module = module.constant::<FEndpoint>();

    #[cfg(feature = "api_endpoint_physics_e")]
    let module = module.constant::<EEndpoint>();

    #[cfg(feature = "api_endpoint_physics_n_a")]
    let module = module.constant::<NAEndpoint>();

    #[cfg(feature = "api_endpoint_physics_k")]
    let module = module.constant::<KEndpoint>();

    #[cfg(feature = "api_endpoint_physics_v_m")]
    let module = module.constant::<VMEndpoint>();

    #[cfg(feature = "api_endpoint_physics_r")]
    let module = module.constant::<REndpoint>();

    #[cfg(feature = "api_endpoint_physics_c")]
    let module = module.constant::<CEndpoint>();

    #[cfg(feature = "api_endpoint_physics_c_1")]
    let module = module.constant::<C1Endpoint>();

    #[cfg(feature = "api_endpoint_physics_c_2")]
    let module = module.constant::<C2Endpoint>();

    #[cfg(feature = "api_endpoint_physics_sigma")]
    let module = module.constant::<SigmaEndpoint>();

    #[cfg(feature = "api_endpoint_physics_epsilon_0")]
    let module = module.constant::<Epsilon0Endpoint>();

    #[cfg(feature = "api_endpoint_physics_mu_0")]
    let module = module.constant::<Mu0Endpoint>();

    #[cfg(feature = "api_endpoint_physics_phi_0")]
    let module = module.constant::<Phi0Endpoint>();

    #[cfg(feature = "api_endpoint_physics_g_standard")]
    let module = module.constant::<GNEndpoint>();

    #[cfg(feature = "api_endpoint_physics_g_0")]
    let module = module.constant::<G0Endpoint>();

    #[cfg(feature = "api_endpoint_physics_z_0")]
    let module = module.constant::<Z0Endpoint>();

    #[cfg(feature = "api_endpoint_physics_t")]
    let module = module.constant::<TEndpoint>();

    #[cfg(feature = "api_endpoint_physics_g")]
    let module = module.constant::<GEndpoint>();

    #[cfg(feature = "api_endpoint_physics_atm")]
    let module = module.constant::<ATMEndpoint>();

    module.build()
}
