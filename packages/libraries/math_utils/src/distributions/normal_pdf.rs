use std::{
    f64::{self, consts::TAU},
    ops::{Div, Mul, Sub},
};

/// Calculates the probability density function of the normal distribution.
/// The normal probability density function is defined as:
///
/// `f(x; μ, σ) = 1 / sqrt(2πσ^2) * exp(-1/2 * ((x - μ) / σ)^2)`
///
/// where:
///
/// - `x` is the value at which the density is evaluated,
/// - `mean` is the mean `μ`,
/// - `standard_deviation` is the standard deviation `σ`.
///
/// Returns `Err` if `standard_deviation <= 0.0`.
pub fn calculate_normal_pdf(
    x: f64,
    mean: f64,
    standard_deviation: f64,
) -> Result<f64, String> {
    if standard_deviation <= 0.0 {
        return Err(String::from(
            "Standard deviation must not be smaller than zero.",
        ));
    }

    Ok(
        (-x.sub(mean).div(standard_deviation).powi(2).mul(0.5)).exp()
            / (standard_deviation * TAU.sqrt()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_normal_pdf_0() {
        assert_eq!(
            calculate_normal_pdf(0.0, 0.0, 1.0).unwrap(),
            0.3989422804014327 // sage: 0.3989422804014327 python: 0.3989422804014326779399461
        )
    }

    #[test]
    fn standard_normal_pdf_1() {
        assert_eq!(
            calculate_normal_pdf(1.0, 0.0, 1.0).unwrap(),
            0.24197072451914337 // sage: 0.24197072451914337
        )
    }

    #[test]
    fn standard_normal_pdf_0_23() {
        assert_eq!(
            calculate_normal_pdf(0.23, 0.0, 1.0).unwrap(),
            0.3885285853158359 // sage: 0.3885285853158359
        )
    }

    #[test]
    fn standard_normal_pdf_minus_0_23() {
        assert_eq!(
            calculate_normal_pdf(-0.23, 0.0, 1.0).unwrap(),
            0.3885285853158359 // sage: 0.3885285853158359
        )
    }

    #[test]
    fn normal_pdf3() {
        assert_eq!(
            calculate_normal_pdf(3.0, 2.0, 5.0).unwrap(),
            0.07820853879509118
        )
    }
}
