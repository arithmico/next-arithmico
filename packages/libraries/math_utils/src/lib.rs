#![allow(clippy::excessive_precision)]
#![allow(clippy::expect_used)]
#![allow(clippy::unwrap_used)]

mod algebra;
mod descriptive_statistics;
mod discrete_mathematics;
mod distributions;
mod f64_extension;
mod numerical_analysis;
mod translation_provider;

pub use algebra::*;
pub use descriptive_statistics::*;
pub use discrete_mathematics::*;
pub use distributions::*;
pub use f64_extension::F64Extension;
pub use numerical_analysis::*;
