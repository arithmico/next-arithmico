mod combinations_permutations;
mod factorize;
mod fibonacci;
mod gcd;
mod lcm;

pub use combinations_permutations::{
    calculate_combinations, calculate_permutations,
};
pub use factorize::factorize;
pub use fibonacci::fibonacci;
pub use gcd::greatest_common_divisor;
pub use lcm::least_common_multiple;
