use core::f64;

use crate::DistributionError::{self, NonNegativeStandardDeviation};

/// Computes the quantile (inverse cumulative distribution function, inverse CDF)
/// of a normal (Gaussian) distribution with given mean and standard deviation.
///
/// This function returns the value `x` such that:
///
/// P(X <= x) = p,  where X ~ N(μ, σ²)
///
/// Internally, the probability `p` is mapped to the standard normal distribution
/// using an approximation of the inverse CDF (`ppnd16`), and then scaled:
///
/// x = μ + σ * Φ^{-1}(p)
///
/// where Φ⁻¹ is the inverse CDF of the standard normal distribution.
///
/// # Arguments
/// - `p`: Probability in the interval [0, 1]
/// - `mean`: The mean (μ) of the distribution
/// - `standard_deviation`: The standard deviation (σ), should be > 0
///
/// # Returns
/// - The quantile corresponding to probability `p`
///
/// # Special Cases
/// - If `standard_deviation <= 0.0`, returns `Err`
/// - If `p <= 0.0`, returns `-∞`
/// - If `p >= 1.0`, returns `+∞`
///
/// # Notes
/// - This implementation uses the AS241 algorithm (`ppnd16`), which provides
///   high accuracy (~1e-16 relative error) across the full domain.
pub fn calculate_quantile_of_normal_cdf(
    p: f64,
    mean: f64,
    standard_deviation: f64,
) -> Result<f64, DistributionError> {
    if standard_deviation <= 0.0 {
        return Err(NonNegativeStandardDeviation);
    }

    if p <= 0.0 {
        return Ok(f64::NEG_INFINITY);
    }
    if p >= 1.0 {
        return Ok(f64::INFINITY);
    }

    let result = mean + standard_deviation * ppnd16(p);

    Ok(result)
}

/*
    Reference:
    Wichura, M.J. (1988).
    Algorithm AS 241: The Percentage Points of the Normal Distribution.
    Applied Statistics, vol. 37, no. 3, 477-484.

    Original code: http://lib.stat.cmu.edu/apstat/241
    We are using PPND16 for double precision.
*/

const SPLIT1: f64 = 0.425;
const SPLIT2: f64 = 5.0;
const CONST1: f64 = 0.180625;
const CONST2: f64 = 1.6;

// Coefficients for P close to 0.5
const A0: f64 = 3.38713_28727_96366_6080E0;
const A1: f64 = 1.33141_66789_17843_7745E+2;
const A2: f64 = 1.97159_09503_06551_4427E+3;
const A3: f64 = 1.37316_93765_50946_1125E+4;
const A4: f64 = 4.59219_53931_54987_1457E+4;
const A5: f64 = 6.72657_70927_00870_0853E+4;
const A6: f64 = 3.34305_75583_58812_8105E+4;
const A7: f64 = 2.50908_09287_30122_6727E+3;

const B1: f64 = 4.23133_30701_60091_1252E+1;
const B2: f64 = 6.87187_00749_20579_0830E+2;
const B3: f64 = 5.39419_60214_24751_1077E+3;
const B4: f64 = 2.12137_94301_58659_5867E+4;
const B5: f64 = 3.93078_95800_09271_0610E+4;
const B6: f64 = 2.87290_85735_72194_2674E+4;
const B7: f64 = 5.22649_52788_52854_5610E+3;

// Coefficients for P not close to 0, 0.5 or 1.
const C0: f64 = 1.42343_71107_49683_57734E0;
const C1: f64 = 4.63033_78461_56545_29590E0;
const C2: f64 = 5.76949_72214_60691_40550E0;
const C3: f64 = 3.64784_83247_63204_60504E0;
const C4: f64 = 1.27045_82524_52368_38258E0;
const C5: f64 = 2.41780_72517_74506_11770E-1;
const C6: f64 = 2.27238_44989_26918_45833E-2;
const C7: f64 = 7.74545_01427_83414_07640E-4;

const D1: f64 = 2.05319_16266_37758_82187E0;
const D2: f64 = 1.67638_48301_83803_84940E0;
const D3: f64 = 6.89767_33498_51000_04550E-1;
const D4: f64 = 1.48103_97642_74800_74590E-1;
const D5: f64 = 1.51986_66563_61645_71966E-2;
const D6: f64 = 5.47593_80849_95344_94600E-4;
const D7: f64 = 1.05075_00716_44416_84324E-9;

// Coefficients for P near 0 or 1.
const E0: f64 = 6.65790_46435_01103_77720E0;
const E1: f64 = 5.46378_49111_64114_36990E0;
const E2: f64 = 1.78482_65399_17291_33580E0;
const E3: f64 = 2.96560_57182_85048_91230E-1;
const E4: f64 = 2.65321_89526_57612_30930E-2;
const E5: f64 = 1.24266_09473_88078_43860E-3;
const E6: f64 = 2.71155_55687_43487_57815E-5;
const E7: f64 = 2.01033_43992_92288_13265E-7;

const F1: f64 = 5.99832_20655_58879_37690E-1;
const F2: f64 = 1.36929_88092_27358_05310E-1;
const F3: f64 = 1.48753_61290_85061_48525E-2;
const F4: f64 = 7.86869_13114_56132_59100E-4;
const F5: f64 = 1.84631_83175_10054_68180E-5;
const F6: f64 = 1.42151_17583_16445_88870E-7;
const F7: f64 = 2.04426_31033_89939_78564E-15;

/// Produces the normal deviate Z corresponding to a given lower
/// tail area of P; Z is accurate to about 1 part in 10**16.
fn ppnd16(p: f64) -> f64 {
    let q = p - 0.5;

    // |q| <= 0.425
    if q.abs() <= SPLIT1 {
        // 0.075 <= p <= 0.925
        let r = CONST1 - q * q;

        return q
            * (((((((A7 * r + A6) * r + A5) * r + A4) * r + A3) * r + A2)
                * r
                + A1)
                * r
                + A0)
            / (((((((B7 * r + B6) * r + B5) * r + B4) * r + B3) * r + B2)
                * r
                + B1)
                * r
                + 1.0);
    }

    // closer than 0.075 from {0,1} boundary
    let mut r = if q < 0.0 { p } else { 1.0 - p }; // r = min(p, 1-p) < 0.075

    // p <= 0.0 (=> -inf) || p >= 1.0 (=> inf)
    if r <= 0.0 {
        return 0.0;
    }

    // r = sqrt(-log(r))  <==>  min(p, 1-p) = exp(-r^2)
    r = (-r.ln()).sqrt();

    // r <= 5
    let t = if r <= SPLIT2 {
        // <==> min(p,1-p) >= exp(-25) ~= 1.3888e-11
        let r = r - CONST2;

        (((((((C7 * r + C6) * r + C5) * r + C4) * r + C3) * r + C2) * r + C1)
            * r
            + C0)
            / (((((((D7 * r + D6) * r + D5) * r + D4) * r + D3) * r + D2) * r
                + D1)
                * r
                + 1.0)
    } else {
        // very close to  0 or 1
        let r = r - SPLIT2;

        (((((((E7 * r + E6) * r + E5) * r + E4) * r + E3) * r + E2) * r + E1)
            * r
            + E0)
            / (((((((F7 * r + F6) * r + F5) * r + F4) * r + F3) * r + F2) * r
                + F1)
                * r
                + 1.0)
    };

    if q < 0.0 { -t } else { t }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverse_standard_normal_cdf_0_5() {
        assert_eq!(
            calculate_quantile_of_normal_cdf(0.5, 0.0, 1.0).unwrap(),
            0.0
        )
    }

    #[test]
    fn inverse_standard_normal_cdf_0_23() {
        assert_eq!(
            calculate_quantile_of_normal_cdf(0.23, 0.0, 1.0).unwrap(),
            -0.7388468491852137
        )
    }

    #[test]
    fn inverse_normal_cdf_0_78() {
        assert_eq!(
            calculate_quantile_of_normal_cdf(0.78, 2., 5.).unwrap(),
            5.860966070943425
        )
    }

    #[test]
    fn inverse_normal_cdf_0_9999() {
        assert_eq!(
            calculate_quantile_of_normal_cdf(0.9999, 2., 5.).unwrap(),
            20.59508242727854
        )
    }

    #[test]
    fn inverse_normal_cdf_10_power_minus_9() {
        assert_eq!(
            calculate_quantile_of_normal_cdf(10e-9, 2., 5.).unwrap(),
            -26.06000622087395
        )
    }
}
