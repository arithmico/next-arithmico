use common::Language;
use engine_derive::ConstantMetadata;
use evaluator::{ConstantEndpoint, Options};
use node::Number;

// source: https://physics.nist.gov/cgi-bin/cuu/Value?mp
#[derive(ConstantMetadata)]
#[name("physics:m_p")]
#[description(Language::English, "Proton mass in kg.")]
#[description(Language::German, "Protonenmasse in kg.")]
pub struct MPEndpoint;

impl ConstantEndpoint for MPEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(1.672_621_926e-27)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?mn
#[derive(ConstantMetadata)]
#[name("physics:m_n")]
#[description(Language::English, "Neutron mass in kg.")]
#[description(Language::German, "Neutronenmasse in kg")]
pub struct MNEndpoint;

impl ConstantEndpoint for MNEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(1.674_927_501e-27)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?me
#[derive(ConstantMetadata)]
#[name("physics:m_e")]
#[description(Language::English, "Electron mass in kg.")]
#[description(Language::German, "Elektronenmasse in kg")]
pub struct MEEndpoint;

impl ConstantEndpoint for MEEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(9.109_383_71e-31)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?mmu
#[derive(ConstantMetadata)]
#[name("physics:m_mu")]
#[description(Language::English, "Muon mass in kg.")]
#[description(Language::German, "Masse des Myons in kg.")]
pub struct MMuEndpoint;

impl ConstantEndpoint for MMuEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(1.883_531_6e-28)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?bohrrada0
#[derive(ConstantMetadata)]
#[name("physics:a_0")]
#[description(
    Language::English,
    "Bohr radius in m denotes the radius of the hydrogen atom in the lowest energy state."
)]
#[description(
    Language::German,
    "Der bohrsche Radius in m bezeichnet den Radius des Wasserstoffatoms im niedrigsten Energiezustand."
)]
pub struct A0Endpoint;

impl ConstantEndpoint for A0Endpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(5.291_772_105e-11)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?h
#[derive(ConstantMetadata)]
#[name("physics:h")]
#[description(Language::English, "Planck constant in J s.")]
#[description(Language::German, "Planck'sches Wirkungsquantum in J s.")]
pub struct HEndpoint;

impl ConstantEndpoint for HEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(6.626_070_15e-34)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?mun
#[derive(ConstantMetadata)]
#[name("physics:mu_N")]
#[description(
    Language::English,
    "Nuclear magneton in J/T is a constant of magnetic moment."
)]
#[description(
    Language::German,
    "Das Kernmagneton in J/T wird als Einheit für magnetische Momente verwendet."
)]
pub struct MuNEndpoint;

impl ConstantEndpoint for MuNEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(5.050_783_74e-27)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?mub
#[derive(ConstantMetadata)]
#[name("physics:mu_B")]
#[description(
    Language::English,
    "Bohr magneton in J/T the magnitude of the magnetic moment of an electron with orbital angular momentum quantum number ℓ = 1."
)]
#[description(
    Language::German,
    "Das bohrsche Magneton in J/T der Betrag des magnetischen Moments eines Elektrons mit Bahndrehimpulsquantenzahl ℓ = 1."
)]
pub struct MuBEndpoint;

impl ConstantEndpoint for MuBEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(9.274_010_07e-24)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?hbar
#[derive(ConstantMetadata)]
#[name("physics:hbar")]
#[description(
    Language::English,
    "Reduced Planck constant in J s equals the Planck constant divided by 2π."
)]
#[description(
    Language::German,
    "Reduzierte Planck-Konstante in J s entspricht dem Planckschen Wirkungsquantum geteilt durch 2π."
)]
pub struct HBarEndpoint;

impl ConstantEndpoint for HBarEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(1.054_571_817e-34)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?alph
#[derive(ConstantMetadata)]
#[name("physics:alpha")]
#[description(
    Language::English,
    "The fine-structure constant is dimensionless and indicates the strength of the electromagnetic interaction."
)]
#[description(
    Language::German,
    "Die Feinstrukturkonstante ist dimensionslos und gibt die Stärke der elektromagnetischen Wechselwirkung an."
)]
pub struct AlphaEndpoint;

impl ConstantEndpoint for AlphaEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(7.297_352_56e-3)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?re
#[derive(ConstantMetadata)]
#[name("physics:r_e")]
#[description(
    Language::English,
    "Classical electron radius in m represents the effective size of an electron in classical electrodynamics."
)]
#[description(
    Language::German,
    "Klassischer Elektronenradius in m beschreibt die effektive Größe eines Elektrons in der klassischen Elektrodynamik."
)]
pub struct REEndpoint;

impl ConstantEndpoint for REEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(2.817_940_32e-15)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?ecomwl
#[derive(ConstantMetadata)]
#[name("physics:lambda_C")]
#[description(
    Language::English,
    "Compton wavelength in m characterizes the quantum mechanical wavelength associated with a particle."
)]
#[description(
    Language::German,
    "Compton-Wellenlänge in m charakterisiert die quantenmechanische Wellenlänge eines Teilchens."
)]
pub struct LambdaCEndpoint;

impl ConstantEndpoint for LambdaCEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(2.426_310_235e-12)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?gammap
#[derive(ConstantMetadata)]
#[name("physics:gamma_p")]
#[description(
    Language::English,
    "Proton gyromagnetic ratio in s^-1 T^-1 relates the magnetic moment of a proton to its angular momentum."
)]
#[description(
    Language::German,
    "Das gyromagnetische Verhältnis des Protons in s^-1 T^-1 beschreibt den Zusammenhang zwischen magnetischem Moment und Drehimpuls eines Protons."
)]
pub struct GammaPEndpoint;

impl ConstantEndpoint for GammaPEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(2.675_221_87e8)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?pcomwl
#[derive(ConstantMetadata)]
#[name("physics:lambda_Cp")]
#[description(
    Language::English,
    "Compton wavelength of the proton in m characterizes the quantum mechanical wavelength associated with a proton."
)]
#[description(
    Language::German,
    "Compton-Wellenlänge des Protons in m charakterisiert die quantenmechanische Wellenlänge eines Protons."
)]
pub struct LambdaCPEndpoint;

impl ConstantEndpoint for LambdaCPEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(1.321_409_854e-15)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?ncomwl
#[derive(ConstantMetadata)]
#[name("physics:lambda_Cn")]
#[description(
    Language::English,
    "Compton wavelength of the neutron in m characterizes the quantum mechanical wavelength associated with a neutron."
)]
#[description(
    Language::German,
    "Compton-Wellenlänge des Neutrons in m charakterisiert die quantenmechanische Wellenlänge eines Neutrons."
)]
pub struct LambdaCNEndpoint;

impl ConstantEndpoint for LambdaCNEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(1.319_590_904e-15)
    }
}

// https://physics.nist.gov/cgi-bin/cuu/Value?ryd
#[derive(ConstantMetadata)]
#[name("physics:R_inf")]
#[description(
    Language::English,
    "The Rydberg constant in 1/m is used for the calculation of atomic spectra."
)]
#[description(
    Language::German,
    "Die Rydberg-Konstante in 1/m wird zur Berechnung von Atomspektren verwendet."
)]
pub struct RInfEndpoint;

impl ConstantEndpoint for RInfEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(10_973_731.568_1)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?u
#[derive(ConstantMetadata)]
#[name("physics:u")]
#[description(
    Language::English,
    "Atomic mass constant in kg is defined as one twelfth of the mass of a carbon-12 atom."
)]
#[description(
    Language::German,
    "Die atomare Masseneinheit in kg ist als ein Zwölftel der Masse eines Kohlenstoff-12-Atoms definiert."
)]
pub struct UEndpoint;

impl ConstantEndpoint for UEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(1.660_539_069e-27)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?mup
#[derive(ConstantMetadata)]
#[name("physics:mu_p")]
#[description(Language::English, "Magnetic moment of the proton in J/T.")]
#[description(Language::German, "Magnetisches Moment des Protons in J/T.")]
pub struct MuPEndpoint;

impl ConstantEndpoint for MuPEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(1.410_606_795e-26)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?muem
#[derive(ConstantMetadata)]
#[name("physics:mu_e")]
#[description(Language::English, "Magnetic moment of the electron in J/T.")]
#[description(Language::German, "Magnetisches Moment des Elektrons in J/T.")]
pub struct MuEEndpoint;

impl ConstantEndpoint for MuEEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(-9.284_764_69e-24)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?munn
#[derive(ConstantMetadata)]
#[name("physics:mu_n")]
#[description(Language::English, "Magnetic moment of the neutron in J/T.")]
#[description(Language::German, "Magnetisches Moment des Neutrons in J/T.")]
pub struct MuNNEndpoint;

impl ConstantEndpoint for MuNNEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(-9.662_37e-27)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?mumum
#[derive(ConstantMetadata)]
#[name("physics:mu_mu")]
#[description(Language::English, "Magnetic moment of the muon in J/T.")]
#[description(Language::German, "Magnetisches Moment des Myons in J/T.")]
pub struct MuMuEndpoint;

impl ConstantEndpoint for MuMuEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(-4.490_448e-26)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?f
#[derive(ConstantMetadata)]
#[name("physics:F")]
#[description(
    Language::English,
    "The Faraday constant in C/mol is the electric charge of one mole of electrons."
)]
#[description(
    Language::German,
    "Die Faraday-Konstante in C/mol ist die elektrische Ladung eines Mols einfach geladener Ionen."
)]
pub struct FEndpoint;

impl ConstantEndpoint for FEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(96_485.332_12)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?e
#[derive(ConstantMetadata)]
#[name("physics:e")]
#[description(
    Language::English,
    "The elementary charge in Colomb of an electron."
)]
#[description(
    Language::German,
    "Die Elementarladung in Colomb eines Elektrons."
)]
pub struct EEndpoint;

impl ConstantEndpoint for EEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(1.602_176_634e-19)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?na
#[derive(ConstantMetadata)]
#[name("physics:N_A")]
#[description(
    Language::English,
    "The Avogadro constant in 1/mol indicates how many particles are contained in one mole."
)]
#[description(
    Language::German,
    "Die Avogadro-Konstante in 1/mol gibt an, wie viele Teilchen in einem Mol enthalten sind."
)]
pub struct NAEndpoint;

impl ConstantEndpoint for NAEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(6.022_140_76e23)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?k
#[derive(ConstantMetadata)]
#[name("physics:k")]
#[description(
    Language::English,
    "The Boltzmann constant in J/K indicates the scaling between energy and temperature."
)]
#[description(
    Language::German,
    "Die Boltzmann-Konstante in J/K gibt die Skalierung zwischen Energie und Temperatur an."
)]
pub struct KEndpoint;

impl ConstantEndpoint for KEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(1.380_649e-23)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?mvolstd
#[derive(ConstantMetadata)]
#[name("physics:V_m")]
#[description(
    Language::English,
    "Molar volume of an ideal gas in m^3/mol at standard conditions (273.15 K, 101.325 kPa)."
)]
#[description(
    Language::German,
    "Molares Volumen eines idealen Gases in m^3/mol unter Normalbedingungen (273.15 K, 101.325 kPa)."
)]
pub struct VMEndpoint;

impl ConstantEndpoint for VMEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(22.413_969_54e-3)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?r
#[derive(ConstantMetadata)]
#[name("physics:R")]
#[description(
    Language::English,
    "The molar gas constant in J/(mol K) occurs in the thermal equation of state of ideal gases."
)]
#[description(
    Language::German,
    "Die molare Gaskonstante in J/(mol K) tritt in der thermischen Zustandsgleichung idealer Gase auf."
)]
pub struct REndpoint;

impl ConstantEndpoint for REndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(8.314_462_618)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?c
#[derive(ConstantMetadata)]
#[name("physics:c")]
#[description(Language::English, "Speed of light in m/s in vacuum.")]
#[description(Language::German, "Lichtgeschwindigkeit in m/s in Vakuum.")]
pub struct CEndpoint;

impl ConstantEndpoint for CEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(299_792_458.)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?c11strc
#[derive(ConstantMetadata)]
#[name("physics:c_1")]
#[description(
    Language::English,
    "First radiation constant in W m^2 used in Planck's law of black-body radiation."
)]
#[description(
    Language::German,
    "Erste Strahlungskonstante in W m^2, verwendet im Planckschen Strahlungsgesetz."
)]
pub struct C1Endpoint;

impl ConstantEndpoint for C1Endpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(3.741_771_852e-16)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?c22ndrc
#[derive(ConstantMetadata)]
#[name("physics:c_2")]
#[description(
    Language::English,
    "Second radiation constant in m K used in Planck's law of black-body radiation."
)]
#[description(
    Language::German,
    "Zweite Strahlungskonstante in m K, verwendet im Planckschen Strahlungsgesetz."
)]
pub struct C2Endpoint;

impl ConstantEndpoint for C2Endpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(1.438_776_877e-2)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?sigma
#[derive(ConstantMetadata)]
#[name("physics:sigma")]
#[description(
    Language::English,
    "Stefan-Boltzmann constant in W/(m^2 K^4) used to calculate black-body radiation power."
)]
#[description(
    Language::German,
    "Stefan-Boltzmann-Konstante in W/(m^2 K^4) zur Berechnung der Strahlungsleistung eines Schwarzen Körpers."
)]
pub struct SigmaEndpoint;

impl ConstantEndpoint for SigmaEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(5.670_374_419e-8)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?ep0
#[derive(ConstantMetadata)]
#[name("physics:epsilon_0")]
#[description(
    Language::English,
    "The vacuum electric permittivity in (A s)/(V m) gives the ratio of electric flux density to electric field strength in vacuum."
)]
#[description(
    Language::German,
    "Die elektrische Feldkonstante in (A s)/(V m) gibt das Verhältnis der elektrischen Flussdichte zur elektrischen Feldstärke im Vakuum an."
)]
pub struct Epsilon0Endpoint;

impl ConstantEndpoint for Epsilon0Endpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(8.854_187_82e-12)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?mu0
#[derive(ConstantMetadata)]
#[name("physics:mu_0")]
#[description(
    Language::English,
    "The vacuum magnetic permeability in N/(A^2) gives the ratio of the magnetic flux density to the magnetic field strength in vacuum."
)]
#[description(
    Language::German,
    "Die magnetische Feldkonstante in N/(A^2) gibt das Verhältnis der magnetischen Flussdichte zur magnetischen Feldstärke im Vakuum an."
)]
pub struct Mu0Endpoint;

impl ConstantEndpoint for Mu0Endpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(1.256_637_061e-6)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?flxquhs2e
#[derive(ConstantMetadata)]
#[name("physics:Phi_0")]
#[description(
    Language::English,
    "Magnetic flux quantum in Wb, the quantum of magnetic flux in superconductivity."
)]
#[description(
    Language::German,
    "Magnetisches Flussquantum in Wb, die kleinste Einheit des magnetischen Flusses in der Supraleitung."
)]
pub struct Phi0Endpoint;

impl ConstantEndpoint for Phi0Endpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(2.067_833_848e-15)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?gn
#[derive(ConstantMetadata)]
#[name("physics:g")]
#[description(
    Language::English,
    "Standard acceleration due to gravity on Earth in m/s^2."
)]
#[description(
    Language::German,
    "Normale Erdbeschleunigung in m/s^2 an der Erdoberfläche."
)]
pub struct GNEndpoint;

impl ConstantEndpoint for GNEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(9.806_65)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?conqu2e2sh
#[derive(ConstantMetadata)]
#[name("physics:G_0")]
#[description(
    Language::English,
    "Conductance quantum in S, the fundamental unit of electrical conductance."
)]
#[description(
    Language::German,
    "Leitfähigkeitsquantum in S, die fundamentale Einheit der elektrischen Leitfähigkeit."
)]
pub struct G0Endpoint;

impl ConstantEndpoint for G0Endpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(7.748_091_729e-5)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?z0
#[derive(ConstantMetadata)]
#[name("physics:Z_0")]
#[description(
    Language::English,
    "Characteristic impedance of vacuum in ohms, relating electric and magnetic fields of electromagnetic waves."
)]
#[description(
    Language::German,
    "Charakteristische Impedanz des Vakuums in Ohm, beschreibt das Verhältnis von elektrischem und magnetischem Feld elektromagnetischer Wellen."
)]
pub struct Z0Endpoint;

impl ConstantEndpoint for Z0Endpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(376.730_313_4)
    }
}

#[derive(ConstantMetadata)]
#[name("physics:t")]
#[description(
    Language::English,
    "Standard temperature in K (0 °C), commonly used in standard conditions (STP)."
)]
#[description(
    Language::German,
    "Standardtemperatur in K (0 °C), wird häufig für Normbedingungen (STP) verwendet."
)]
pub struct TEndpoint;

impl ConstantEndpoint for TEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(273.15)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?bg
#[derive(ConstantMetadata)]
#[name("physics:G")]
#[description(
    Language::English,
    "The Newtonian constant of gravitation in m^3/(kg s^2) gives the strength of the gravitational force between two bodies as a function of their distance and masses."
)]
#[description(
    Language::German,
    "Die Gravitationskonstante in m^3/(kg s^2) gibt die Stärke der Gravitationskraft zwischen zwei Körpern in Abhängigkeit von ihrem Abstand und ihren Massen berechnen."
)]
pub struct GEndpoint;

impl ConstantEndpoint for GEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(6.674e-11)
    }
}

// source: https://physics.nist.gov/cgi-bin/cuu/Value?stdatm
#[derive(ConstantMetadata)]
#[name("physics:atm")]
#[description(
    Language::English,
    "Standard atmospheric pressure in Pa used as a reference pressure."
)]
#[description(
    Language::German,
    "Standardatmosphärendruck in Pa, verwendet als Referenzdruck."
)]
pub struct ATMEndpoint;

impl ConstantEndpoint for ATMEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(101_325e0)
    }
}
