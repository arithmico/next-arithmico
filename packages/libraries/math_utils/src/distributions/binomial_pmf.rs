/// Computes the probability mass function (PMF) of the binomial distribution.
///
/// This is a thin wrapper around the internal `dbinom` implementation,
/// which uses a numerically stable saddlepoint approximation (Loader).
///
/// # Arguments
///
/// * `n` - Number of trials (n >= 0)
/// * `p` - Success probability (0 <= p <= 1)
/// * `k` - Number of successes (0 <= k <= n)
///
/// # Returns
///
/// The probability P(X = k) for X ~ Binomial(n, p).
///
/// # Notes
///
/// Internally converts integer inputs to `f64` to match the numerical backend.
pub fn calculate_binomial_pmf(n: usize, p: f64, k: usize) -> f64 {
    let n = n as f64;
    let k = k as f64;

    if !(0.0..=1.0).contains(&p) {
        return f64::NAN;
    }

    if k < 0.0 || n < 0.0 || k > n {
        return 0.0;
    }

    dbinom(k, n, p)
}

/*
    Reference:
    Catherine Loader.
    Fast and Accurate Computation of Binomial Probabilities. (2002)

    Source: https://www.r-project.org/doc/reports/CLoader-dbinom-2002.pdf
*/

use std::f64::consts::TAU;

//const PI2: f64 = 6.283185307179586476925286;
const S0: f64 = 0.083333333333333333333; /* 1/12 */
const S1: f64 = 0.00277777777777777777778; /* 1/360 */
const S2: f64 = 0.00079365079365079365079365; /* 1/1260 */
const S3: f64 = 0.000595238095238095238095238; /* 1/1680 */
const S4: f64 = 0.0008417508417508417508417508; /* 1/1188 */

const SFE: [f64; 16] = [
    0.0,
    0.081061466795327258219670264,
    0.041340695955409294093822081,
    0.0276779256849983391487892927,
    0.020790672103765093111522771,
    0.0166446911898211921631948653,
    0.013876128823070747998745727,
    0.0118967099458917700950557241,
    0.010411265261972096497478567,
    0.0092554621827127329177286366,
    0.008330563433362871256469318,
    0.0075736754879518407949720242,
    0.006942840107209529865664152,
    0.0064089941880042070684396310,
    0.005951370112758847735624416,
    0.0055547335519628013710386899,
];

/// stirlerr(n) = log(n!) - log( sqrt(2*pi*n)*(n/e)^n )
fn stirlerr(n: f64) -> f64 {
    let n_int = n as usize;
    if n_int < 16 {
        return SFE[n_int];
    }

    let n = n_int as f64;
    let nn = n;
    let nn = nn * nn;
    if n > 500. {
        return (S0 - S1 / nn) / n;
    }
    if n > 80. {
        return (S0 - (S1 - S2 / nn) / nn) / n;
    }
    if n > 35. {
        return (S0 - (S1 - (S2 - S3 / nn) / nn) / nn) / n;
    }

    return (S0 - (S1 - (S2 - (S3 - S4 / nn) / nn) / nn) / nn) / n;
}

/// Evaluate the deviance term
/// bd0(x,np) = x log(x/np) + np - x
fn bd0(x: f64, np: f64) -> f64 {
    if (x - np).abs() < 0.1 * (x + np) {
        let mut s = (x - np) * (x - np) / (x + np);
        let v = (x - np) / (x + np);
        let mut ej = 2. * x * v;

        // for(j=1; ;j++)
        for j in 1.. {
            ej *= v * v;
            let s1 = s + ej / ((2 * j + 1) as f64);
            if s1 == s {
                return s1;
            }
            s = s1;
        }
    }
    return x * (x / np).ln() + np - x;
}

fn dbinom(x: f64, n: f64, p: f64) -> f64 {
    if p == 0.0 {
        return if x == 0.0 { 1.0 } else { 0.0 };
    }
    if p == 1.0 {
        return if x == n { 1.0 } else { 0.0 };
    }
    if x == 0.0 {
        return (n * (1. - p).ln()).exp();
    }
    if x == n {
        return (n * p.ln()).exp();
    }

    let lc = stirlerr(n)
        - stirlerr(x)
        - stirlerr(n - x)
        - bd0(x, n * p)
        - bd0(n - x, n * (1.0 - p));
    
    return lc.exp() * (n / (TAU * x * (n - x))).sqrt();
}

#[allow(dead_code)]
fn dpois(x: f64, lb: f64) -> f64 {
    if lb == 0.0 {
        return if x == 0.0 { 1.0 } else { 0.0 };
    }
    if x == 0.0 {
        return (-lb).exp();
    }
    return (-stirlerr(x) - bd0(x, lb)).exp() / (TAU * x).sqrt();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binomial_pdf() {
        assert_eq!(calculate_binomial_pmf(10, 0.87, 9), 0.37120740051593837)
    }
}
