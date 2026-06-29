/// Computes the cumulative distribution function (CDF) of a normal (Gaussian)
/// distribution with given mean and standard deviation.
///
/// The CDF is defined as:
///
/// Φ(x; μ, σ) = P(X ≤ x)
///
/// where X ~ N(μ, σ²).
///
/// Internally, the input is standardized to the standard normal distribution:
/// and then evaluated using a numerically stable implementation of the
/// standard normal CDF (ndtr), based on the Cephes library.
///
/// # Arguments
/// - x: The value at which to evaluate the CDF
/// - mean: The mean (μ) of the distribution
/// - standard_deviation: The standard deviation (σ), must be > 0
///
/// # Returns
/// The probability P(X ≤ x)
///
/// # Panics / Notes
/// - For standard_deviation < 0.0 NaN returns.
///
/// # Accuracy
/// This implementation inherits the numerical stability and precision
/// of the Cephes ndtr function, which provides high accuracy across
/// the full range of f64 values, including extreme tails.
pub fn calculate_normal_cdf(x: f64, mean: f64, standard_deviation: f64) -> f64 {
    if standard_deviation < 0.0 {
        return f64::NAN;
    }

    let z = (x - mean) / standard_deviation;

    ndtr(z)
}

// This is Rust transpilation of ndtr.c:
//
// Cephes Math Library Release 2.9:  November, 2000
// Copyright 1984, 1987, 1988, 1992, 2000 by Stephen L. Moshier
//
// Original web site: https://www.netlib.org/cephes/
// Code mirror: https://github.com/jeremybarnes/cephes/blob/master/cprob/ndtr.c

use core::f64;
use std::f64::consts::FRAC_1_SQRT_2;

const P: [f64; 9] = [
    2.46196981473530512524E-10,
    5.64189564831068821977E-1,
    7.46321056442269912687E0,
    4.86371970985681366614E1,
    1.96520832956077098242E2,
    5.26445194995477358631E2,
    9.34528527171957607540E2,
    1.02755188689515710272E3,
    5.57535335369399327526E2,
];
const Q: [f64; 8] = [
    /* 1.00000000000000000000E0,*/
    1.32281951154744992508E1,
    8.67072140885989742329E1,
    3.54937778887819891062E2,
    9.75708501743205489753E2,
    1.82390916687909736289E3,
    2.24633760818710981792E3,
    1.65666309194161350182E3,
    5.57535340817727675546E2,
];
const R: [f64; 6] = [
    5.64189583547755073984E-1,
    1.27536670759978104416E0,
    5.01905042251180477414E0,
    6.16021097993053585195E0,
    7.40974269950448939160E0,
    2.97886665372100240670E0,
];
const S: [f64; 6] = [
    /* 1.00000000000000000000E0,*/
    2.26052863220117276590E0,
    9.39603524938001434673E0,
    1.20489539808096656605E1,
    1.70814450747565897222E1,
    9.60896809063285878198E0,
    3.36907645100081516050E0,
];
const T: [f64; 5] = [
    9.60497373987051638749E0,
    9.00260197203842689217E1,
    2.23200534594684319226E3,
    7.00332514112805075473E3,
    5.55923013010394962768E4,
];
const U: [f64; 5] = [
    /* 1.00000000000000000000E0,*/
    3.35617141647503099647E1,
    5.21357949780152679795E2,
    4.59432382970980127987E3,
    2.26290000613890934246E4,
    4.92673942608635921086E4,
];

const UTHRESH: f64 = 37.519379347;
const MAXLOG: f64 = 7.09782712893383996732E2; // MAXLOG ≈ ln(f64::MAX), see const.c

/// This is a direct Rust port of the Cephes ndtr implementation,
/// which evaluates:
///
/// Φ(a) = 1/2 * (1 + erf(a / sqrt(2)))
///
/// The implementation uses different numerical strategies depending
/// on the magnitude of the input to ensure stability and precision:
///
/// - For small |x|: direct evaluation via erf
/// - For large |x|: evaluation via erfc (complementary error function)
/// - For extreme values: scaled evaluation using erfce and expx2
///
/// This avoids catastrophic cancellation and underflow in the tails.
///
/// # Arguments
/// - a: input value
///
/// # Returns
/// The value of the standard normal CDF at a.
fn ndtr(a: f64) -> f64 {
    let x = a * FRAC_1_SQRT_2;
    let z = x.abs();

    /* if( z < SQRTH ) */
    let y = if z < 1.0 {
        0.5 + 0.5 * erf(x)
    } else {
        let mut y = 0.5 * erfce(z);

        let exp_term = expx2(a, -1); // ≈ exp(-a^2)
        y = y * exp_term.sqrt(); // ergibt exp(-x^2)

        if x > 0.0 {
            y = 1.0 - y;
        }

        y
    };

    return y;
}

/// Computes the complementary error function:
///
/// erfc(x) = 1 - erf(x)
///
/// This implementation is numerically stable for large values of x,
/// where direct evaluation of erf(x) would lose precision.
///
/// Uses rational polynomial approximations and exponential scaling.
///
/// # Arguments
/// - a: input value
///
/// # Returns
/// The value of erfc(a)
fn erfc(a: f64) -> f64 {
    let x = if a < 0.0 { -a } else { a };

    if x < 1.0 {
        return 1.0 - erf(a);
    }

    let z = -a * a;

    if z < -MAXLOG {
        // underflow
        return if a < 0.0 { 2.0 } else { 0.0 };
    }

    let z = expx2(a, -1);

    let (p, q) = if x < 8.0 {
        let p = polevl(x, &P);
        let q = p1evl(x, &Q);

        (p, q)
    } else {
        let p = polevl(x, &R);
        let q = p1evl(x, &S);

        (p, q)
    };
    let mut y = (z * p) / q;

    if a < 0.0 {
        y = 2.0 - y;
    }

    if y == 0.0 {
        return if a < 0.0 { 2.0 } else { 0.0 };
    }

    return y;
}

/// Exponentially scaled erfc function
/// exp(x^2) erfc(x)
/// valid for x > 1.
/// Use with ndtr and expx2.
fn erfce(x: f64) -> f64 {
    let (p, q) = if x < 8.0 {
        let p = polevl(x, &P);
        let q = p1evl(x, &Q);

        (p, q)
    } else {
        let p = polevl(x, &R);
        let q = p1evl(x, &S);

        (p, q)
    };
    return p / q;
}

/// Uses a polynomial approximation for |x| <= 1,
/// and defers to erfc for larger values to maintain precision.
///
/// # Arguments
/// - x: input value
///
/// # Returns
/// erf(x)
fn erf(x: f64) -> f64 {
    if x.abs() > 1.0 {
        return 1.0 - erfc(x);
    }
    let z = x * x;
    let y = x * polevl(z, &T) / p1evl(z, &U);
    return y;
}

/// This function avoids overflow and underflow by splitting
/// the argument into large and small components.
///
/// Used internally for stable evaluation of Gaussian tails.
///
/// # Arguments
/// - x: input value
/// - sign: +1 or -1 (controls exponent sign)
///
/// # Returns
/// exp(sign * x^2)
fn expx2(x: f64, sign: i32) -> f64 {
    // since we have no DEC architecture, const as follows
    const M: f64 = 128.0;
    const MINV: f64 = 0.0078125; // 1 / M

    let mut x = x.abs();
    if sign < 0 {
        x = -x;
    }

    /* Represent x as an exact multiple of M plus a residual.
    M is a power of 2 chosen so that exp(m * m) does not overflow
    or underflow and so that |x - m| is small.  */
    let m = MINV * (M * x + 0.5).floor();
    let f = x - m;

    /* x^2 = m^2 + 2mf + f^2 */
    let mut u = m * m;
    let mut u1 = 2. * m * f + f * f;

    if sign < 0 {
        u = -u;
        u1 = -u1;
    }

    if (u + u1) > MAXLOG {
        return f64::INFINITY;
    }

    /* u is exact, u1 is small.  */
    u = u.exp() * u1.exp();
    return u;
}

/// Evaluates a polynomial using Horner's method.
///
/// Computes:
///
/// coef[0]*x^n + coef[1]*x^(n-1) + ... + coef[n]
///
/// # Arguments
/// - x: evaluation point
/// - coef: polynomial coefficients
///
/// # Returns
/// polynomial value
fn polevl(x: f64, coef: &[f64]) -> f64 {
    let mut ans = coef[0];

    for c in &coef[1..] {
        ans = ans * x + c;
    }

    ans
}

/// Evaluates a polynomial with leading coefficient 1:
///
/// x^n + coef[0]*x^(n-1) + ... + coef[n-1]
///
/// # Arguments
/// - x: evaluation point
/// - coef: coefficients
///
/// # Returns
/// polynomial value
fn p1evl(x: f64, coef: &[f64]) -> f64 {
    let mut ans = x + coef[0];

    for c in &coef[1..] {
        ans = ans * x + c;
    }

    ans
}
