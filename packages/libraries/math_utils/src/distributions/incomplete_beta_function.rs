use core::f64;
use std::f64::consts::{FRAC_2_SQRT_PI, FRAC_PI_4, LN_2};

use thiserror::Error;

use crate::distributions::incomplete_beta_function::BetaError::{
    BothAAndBZero, Internal, NanInput, NegativeAOrB, XAndAZero, XOutOfRange,
    XPlusYNotOne, YAndBZero, YOutOfRange,
};

/// Computes the regularized incomplete beta function `I_x(a, b)`.
///
/// This returns the first value `w` computed by [`bratio`], where
///
/// `w = I_x(a, b)`.
///
/// The complementary value `1 - I_x(a, b)` is computed internally as well,
/// but discarded by this function. Use
/// [`calculate_incomplete_beta_function_complementary`] if the complementary
/// value is needed directly.
///
/// # Errors
///
/// Returns [`BetaError`] if the input parameters are outside the domain
/// accepted by `bratio`, for example if `a` or `b` is negative, if `x` is
/// outside `[0, 1]`, or if one of the boundary cases is undefined.
#[allow(dead_code)]
pub fn calculate_incomplete_beta_function(
    x: f64,
    a: f64,
    b: f64,
) -> Result<f64, BetaError> {
    let y = 1.0 - x;
    let BratioResult { w, .. } = bratio(a, b, x, y)?;

    Ok(w)
}

/// Computes the complementary regularized incomplete beta function
/// `1 - I_x(a, b)`.
///
/// This returns the second value `w1` computed by [`bratio`], where
///
/// `w1 = 1 - I_x(a, b)`.
///
/// Computing the complementary value directly can be preferable to subtracting
/// the result of [`calculate_incomplete_beta_function`] from `1.0`, because
/// `bratio` may choose a numerically more stable path for the requested tail.
///
/// # Errors
///
/// Returns [`BetaError`] if the input parameters are outside the domain
/// accepted by `bratio`, for example if `a` or `b` is negative, if `x` is
/// outside `[0, 1]`, or if one of the boundary cases is undefined.
pub fn calculate_complementary_incomplete_beta_function(
    x: f64,
    a: f64,
    b: f64,
) -> Result<f64, BetaError> {
    let y = 1.0 - x;
    let BratioResult { w1, .. } = bratio(a, b, x, y)?;

    Ok(w1)
}

#[derive(Error, Debug, PartialEq)]
pub enum BetaError {
    #[error("bratio: a or b is negative")]
    NegativeAOrB,
    #[error("bratio: a = b = 0")]
    BothAAndBZero,
    #[error("bratio: x < 0 or x > 1")]
    XOutOfRange,
    #[error("bratio: y < 0 or y > 1")]
    YOutOfRange,
    #[error("bratio: x + y != 1")]
    XPlusYNotOne,
    #[error("bratio: x = a = 0")]
    XAndAZero,
    #[error("bratio: y = b = 0")]
    YAndBZero,
    #[error("bratio: NaN in a, b, x, or y")]
    NanInput,
    #[error("{0}")]
    Internal(&'static str),
}

struct BratioResult {
    w: f64,
    w1: f64,
}

impl BratioResult {
    fn new(w: f64, w1: f64) -> Self {
        Self { w, w1 }
    }

    fn from_swap(is_swap: bool, w: f64, w1: f64) -> Self {
        if is_swap {
            Self::new(w1, w)
        } else {
            Self::new(w, w1)
        }
    }
}

/*
   Reference:
   Algorithm 708, Collected Algorithms from ACM.
   This work published in Transactions on Mathematical Software.
   vol. 18, no. 3, September, 1992, pp. 360-373.

   Original code:
   Written by Alfred H. Morris
   Naval Surface Warfare Center
   Dahlgren, Virginia

   Source: https://doi.org/10.1145/131766.131776

   The following algorithm is based heavily on the above work
   and was carefully transpiled into Rust.
   Naming is adopted.
*/

/// Evaluation of the incomplete beta function I_x(a, b).
/// Computes the values w = I_x(a, b) and w1 = 1 - I_x(a, b).
///
/// It is assumed that a and b are nonnegative, ant that x <= x
/// and y = 1 - x. bratio assigns w and w1 the values
///
/// w = I_x(a, b) and w1 = 1 - I_x(a, b)
fn bratio(a: f64, b: f64, x: f64, y: f64) -> Result<BratioResult, BetaError> {
    let mut eps = f64::EPSILON;

    if a.is_nan() || b.is_nan() || x.is_nan() || y.is_nan() {
        return Err(NanInput);
    }
    if a < 0.0 || b < 0.0 {
        return Err(NegativeAOrB);
    }
    if a == 0.0 && b == 0.0 {
        return Err(BothAAndBZero);
    }
    if !(0.0..=1.0).contains(&x) {
        return Err(XOutOfRange);
    }
    if !(0.0..=1.0).contains(&y) {
        return Err(YOutOfRange);
    }

    let z = ((x + y) - 0.5) - 0.5;
    if z.abs() > 3.0 * eps {
        return Err(XPlusYNotOne);
    }

    if x == 0.0 {
        if a == 0.0 {
            return Err(XAndAZero);
        }
        return Ok(BratioResult::new(0.0, 1.0));
    }

    if y == 0.0 {
        if b == 0.0 {
            return Err(YAndBZero);
        }
        return Ok(BratioResult::new(1.0, 0.0));
    }

    if a == 0.0 {
        return Ok(BratioResult::new(1.0, 0.0));
    }
    if b == 0.0 {
        return Ok(BratioResult::new(0.0, 1.0));
    }

    eps = eps.max(1e-15);
    if a.max(b) < 1e-3 * eps {
        // Procedure for a and b < 1.e-3 * eps
        let w = b / (a + b);
        let w1 = a / (a + b);
        return Ok(BratioResult::new(w, w1));
    }

    if a.min(b) <= 1.0 {
        // Procedure for a0 <= 1 or b0 <= 1
        let is_swap = x > 0.5;
        let (a0, b0, x0, y0) =
            if is_swap { (b, a, y, x) } else { (a, b, x, y) };

        if b0 < eps.min(eps * a0) {
            let w = fpser(a0, b0, x0, eps);
            let w1 = 0.5 + (0.5 - w);
            return Ok(BratioResult::from_swap(is_swap, w, w1));
        }

        if a0 < eps.min(eps * b0) && b0 * x0 <= 1.0 {
            let w1 = apser(a0, b0, x0, eps);
            let w = 0.5 + (0.5 - w1);
            return Ok(BratioResult::from_swap(is_swap, w, w1));
        }

        if a0.max(b0) > 1.0 {
            if b0 <= 1.0 {
                let w = bpser(a0, b0, x0, eps);
                let w1 = 0.5 + (0.5 - w);
                return Ok(BratioResult::from_swap(is_swap, w, w1));
            }

            if x0 >= 0.29 {
                // changes from original x0 >= 0.3 to R's x0 >= 0.29
                let w1 = bpser(b0, a0, y0, eps);
                let w = 0.5 + (0.5 - w1);
                return Ok(BratioResult::from_swap(is_swap, w, w1));
            }

            if x0 < 0.1 && (x0 * b0).powf(a0) <= 0.7 {
                let w = bpser(a0, b0, x0, eps);
                let w1 = 0.5 + (0.5 - w);
                return Ok(BratioResult::from_swap(is_swap, w, w1));
            }

            if b0 > 15.0 {
                let w1 = bgrat(b0, a0, y0, x0, 0.0, 15.0 * eps)?;
                let w = 0.5 + (0.5 - w1);
                return Ok(BratioResult::from_swap(is_swap, w, w1));
            }
        } else {
            // min(a0, b0) <= 1.0 and max(a0, b0) <= 1.0
            if a0 >= b0.min(0.2) {
                let w = bpser(a0, b0, x0, eps);
                let w1 = 0.5 + (0.5 - w);
                return Ok(BratioResult::from_swap(is_swap, w, w1));
            }
            if x0.powf(a0) <= 0.9 {
                let w = bpser(a0, b0, x0, eps);
                let w1 = 0.5 + (0.5 - w);
                return Ok(BratioResult::from_swap(is_swap, w, w1));
            }
            if x0 >= 0.3 {
                let w1 = bpser(b0, a0, y0, eps);
                let w = 0.5 + (0.5 - w1);
                return Ok(BratioResult::from_swap(is_swap, w, w1));
            }
        }

        let n = 20;
        let mut w1 = bup(b0, a0, y0, x0, n, eps);
        let b0 = b0 + n as f64;
        w1 = bgrat(b0, a0, y0, x0, w1, 15.0 * eps)?;
        let w = 0.5 + (0.5 - w1);
        return Ok(BratioResult::from_swap(is_swap, w, w1));
    }

    // Procedure for a0 > 1 and b0 > 1
    let mut lambda = if (a + b).is_finite() {
        if a > b {
            (a + b) * y - b
        } else {
            a - (a + b) * x
        }
    } else {
        a * y - b * x
    };

    let is_swap = lambda < 0.0;

    let (a0, b0, x0, y0) = if is_swap { (b, a, y, x) } else { (a, b, x, y) };

    if is_swap {
        lambda = -lambda;
    }

    if b0 < 40.0 {
        if b0 * x0 <= 0.7 {
            let w = bpser(a0, b0, x0, eps);
            let w1 = 0.5 + (0.5 - w);
            return Ok(BratioResult::from_swap(is_swap, w, w1));
        }

        let mut n = b0.trunc() as usize;
        let mut b0 = b0 - n as f64;
        if b0 == 0.0 {
            n -= 1;
            b0 = 1.0;
        }

        let mut w = bup(b0, a0, y0, x0, n, eps);
        if x0 <= 0.7 {
            w += bpser(a0, b0, x0, eps);
            let w1 = 0.5 + (0.5 - w);
            return Ok(BratioResult::from_swap(is_swap, w, w1));
        }

        let mut a0 = a0;
        if a0 <= 15.0 {
            n = 20;
            w += bup(a0, b0, x0, y0, n, eps);
            a0 += n as f64;
        }

        w = bgrat(a0, b0, x0, y0, w, 15.0 * eps)?;
        let w1 = 0.5 + (0.5 - w);
        return Ok(BratioResult::from_swap(is_swap, w, w1));
    }

    if a0 <= b0 && (a0 <= 100.0 || lambda > 0.03 * a0) {
        let w = bfrac(a0, b0, x0, y0, lambda, 15.0 * eps);
        let w1 = 0.5 + (0.5 - w);
        return Ok(BratioResult::from_swap(is_swap, w, w1));
    }
    if b0 <= 100.0 || lambda > 0.03 * b0 {
        let w = bfrac(a0, b0, x0, y0, lambda, 15.0 * eps);
        let w1 = 0.5 + (0.5 - w);
        return Ok(BratioResult::from_swap(is_swap, w, w1));
    }

    let w = basym(a0, b0, lambda, 100.0 * eps);
    let w1 = 0.5 + (0.5 - w);
    Ok(BratioResult::from_swap(is_swap, w, w1))
}

/// Evaluation of I_x(a, b) for b < min(eps, eps * a) and x <= 0.5.
fn fpser(a: f64, b: f64, x: f64, eps: f64) -> f64 {
    let mut value = 1.0;
    if a > 1e-3 * eps {
        value = 0.0;
        let t = a * x.ln();
        if t < exparg(1) {
            return value;
        }
        value = t.exp();
    }

    // Note that 1 / B(A,B) = B
    value *= b / a;
    let tol = eps / a;
    let mut an = a + 1.0;
    let mut t = x;
    let mut s = t / an;

    loop {
        an += 1.0;
        t *= x;
        let c = t / an;
        s += c;

        if c.abs() <= tol {
            break;
        }
    }

    value * (1.0 + a * s)
}

/// apser yields the incomplete beta ratio I_{1 - x}(b, a) for
/// a <= min(eps, eps * b), b * x <= 1, and x <= 0.5. Used when
/// a is very small. Use only if above inequalities are satisfied.
fn apser(a: f64, b: f64, x: f64, eps: f64) -> f64 {
    const G: f64 = 0.577215664901533;

    let bx = b * x;
    let mut t = x - bx;

    let c = if b * eps > 2e-2 {
        bx.ln() + G + t
    } else {
        x.ln() + psi(b) + G + t
    };

    let tol = 5.0 * eps * c.abs();
    let mut j = 1.0;
    let mut s = 0.0;

    loop {
        j += 1.0;
        t *= x - bx / j;
        let aj = t / j;
        s += aj;

        if aj.abs() <= tol {
            break;
        }
    }

    -a * (c + s)
}

/// Power series expansion for evaluating I_x(a, b) when b <= 1
/// or b * x <= 0.7
fn bpser(a: f64, b: f64, x: f64, eps: f64) -> f64 {
    let mut value = 0.0;
    if x == 0.0 {
        return value;
    }

    // Compute the factor x ^ a / (a * beta(a, b))
    let a0 = a.min(b);
    if a0 >= 1.0 {
        let z = a * x.ln() - betaln(a, b);
        value = z.exp() / a;
        if value == 0.0 || a <= 0.1 * eps {
            return value;
        }
    } else {
        let mut b0 = a.max(b);

        // Procedure for a0 < 1 and b0 >= 8
        if b0 >= 8.0 {
            let u = gamln1(a0) + algdiv(a0, b0);
            let z = a * x.ln() - u;
            value = (a0 / a) * z.exp();
            if value == 0.0 || a <= 0.1 * eps {
                return value;
            }
        } else if b0 > 1.0 {
            // Procedure for a0 < 1 and 1 < b0 < 8
            let m = (b0 - 1.0).trunc() as usize;
            let u = if m >= 1 {
                let mut c = 1.0;
                for _i in 1..=m {
                    b0 -= 1.0;
                    c *= b0 / (a0 + b0);
                }

                c.ln() + gamln1(a0)
            } else {
                gamln1(a0)
            };

            let z = a * x.ln() - u;
            b0 -= 1.0;
            let apb = a0 + b0;
            let t = if apb > 1.0 {
                let u = a0 + b0 - 1.0;

                (1.0 + gam1(u)) / apb
            } else {
                1.0 + gam1(apb)
            };
            value = (z.exp() * (a0 / a) * (1.0 + gam1(b0))) / t;
            if value == 0.0 || a <= 0.1 * eps {
                return value;
            }
        } else {
            // Procedure for a0 < 1 and b0 < 1
            value = x.powf(a);
            if value == 0.0 {
                return value;
            }

            let apb = a + b;
            let z = if apb > 1.0 {
                let u = a + b - 1.0;

                (1.0 + gam1(u)) / apb
            } else {
                1.0 + gam1(apb)
            };
            let c = ((1.0 + gam1(a)) * (1.0 + gam1(b))) / z;
            value = value * c * (b / apb);
            if value == 0.0 || a <= 0.1 * eps {
                return value;
            }
        }
    }

    // Computes the series
    let mut sum = 0.0;
    let mut n = 0.0;
    let mut c = 1.0;
    let tol = eps / a;

    loop {
        n += 1.0;
        c = c * (0.5 + (0.5 - b / n)) * x;
        let w = c / (a + n);
        sum += w;

        if w.abs() <= tol {
            break;
        }
    }

    if a * sum > -1.0 {
        value * (1.0 + a * sum)
    } else {
        0.0
    }
}

/// Evaluation of I_x(a, b) - I_x(a * n, b) where n is a positive integer.
/// eps is the tolerance used.
fn bup(a: f64, b: f64, x: f64, y: f64, n: usize, eps: f64) -> f64 {
    // Obtain the scaling factor exp(-mu) and exp(mu) * (x ^ a * y ^ b / beta(a, b)) / a
    let apb = a + b;
    let ap1 = a + 1.0;

    let mut mu = 0;
    let mut d = 1.0;

    if n != 1 && a >= 1.0 && apb >= 1.1 * ap1 {
        mu = exparg(1).abs().trunc() as i32;
        let k = exparg(0).trunc() as i32;
        if k < mu {
            mu = k;
        }
        d = (-mu as f64).exp();
    }

    let value = brcmp1(mu as f64, a, b, x, y) / a;
    if n == 1 || value == 0.0 {
        return value;
    }

    let nm1 = n - 1;
    let mut w = d;

    // Let k be the index of the maximum term
    let mut k = 0;
    if b > 1.0 {
        if y > 1e-4 {
            let r = ((b - 1.0) * x) / y - a;
            if r >= 1.0 {
                let r_int = r.trunc() as usize;

                k = if r_int < nm1 { r_int } else { nm1 };
            }
        } else {
            k = nm1;
        }

        // Add the increasing terms of the series
        for i in 0..k {
            let i_f = i as f64;
            d *= ((apb + i_f) / (ap1 + i_f)) * x;
            w += d;
        }
        if k == nm1 {
            return value * w;
        }
    }

    // Add the remaining terms of the series
    for i in k..nm1 {
        let i_f = i as f64;
        d *= ((apb + i_f) / (ap1 + i_f)) * x;
        w += d;
        if d <= eps * w {
            break;
        }
    }

    value * w
}

/// Continued fraction expansion for I_x(a, b) when a, b > 1.
/// It is assumed that lambda = (a + b) * y - b
fn bfrac(a: f64, b: f64, x: f64, y: f64, lambda: f64, eps: f64) -> f64 {
    let value = brcomp(a, b, x, y);
    if value == 0.0 {
        return value;
    }

    let c = 1.0 + lambda;
    let c0 = b / a;
    let c1 = 1.0 + 1.0 / a;
    let yp1 = y + 1.0;

    let mut n = 0.0;
    let mut p = 1.0;
    let mut s = a + 1.0;
    let mut an = 0.0;
    let mut bn = 1.0;
    let mut anp1 = 1.0;
    let mut bnp1 = c / c1;
    let mut r = c1 / c;
    let mut r0;

    // Continued fraction calculation
    loop {
        n += 1.0;
        let t = n / a;
        let w = n * (b - n) * x;
        let e1 = a / s;
        let alpha = p * (p + c0) * e1 * e1 * (w * x);
        let e2 = (1.0 + t) / (c1 + t + t);
        let beta = n + w / s + e2 * (c + n * yp1);
        p = 1.0 + t;
        s += 2.0;

        // Update an, bn, anp1, and bnp1
        let t = alpha * an + beta * anp1;
        an = anp1;
        anp1 = t;
        let t = alpha * bn + beta * bnp1;
        bn = bnp1;
        bnp1 = t;

        r0 = r;
        r = anp1 / bnp1;
        if (r - r0).abs() <= eps * r {
            break;
        }

        // Rescale an, bn, anp1, and bnp1
        an /= bnp1;
        bn /= bnp1;
        anp1 = r;
        bnp1 = 1.0;

        let n_int = n.trunc() as usize;
        if n_int >= 1_000 {
            break;
        }
    }

    value * r
}

/// Evaluating of x^a * y^b / beta(a, b)
fn brcomp(a: f64, b: f64, x: f64, y: f64) -> f64 {
    const CONST: f64 = 0.398942280401433e0; // 0.398942280401433 = 1 / sqrt(2 * pi)

    if x == 0.0 || y == 0.0 {
        return 0.0;
    }

    let a0 = a.min(b);
    if a0 < 8.0 {
        let (lnx, lny) = if x <= 0.375 {
            (x.ln(), alnrel(-x))
        } else if y <= 0.375 {
            (alnrel(-y), y.ln())
        } else {
            (x.ln(), y.ln())
        };

        let mut z = a * lnx + b * lny;
        if a0 >= 1.0 {
            z -= betaln(a, b);
            return z.exp();
        }

        // Procedure for a < 1 or b < 1
        let mut b0 = a.max(b);
        if b0 >= 8.0 {
            // Algorithm for b0 >= 8
            let u = gamln1(a0) + algdiv(a0, b0);
            return a0 * (z - u).exp();
        }

        if b0 <= 1.0 {
            // Algorithm for b0 <= 1
            let value = z.exp();
            if value == 0.0 {
                return value;
            }

            let apb = a + b;
            let z = if apb > 1.0 {
                let u = a + b - 1.0;

                (1.0 + gam1(u)) / apb
            } else {
                1.0 + gam1(apb)
            };

            let c = ((1.0 + gam1(a)) * (1.0 + gam1(b))) / z;
            return (value * (a0 * c)) / (1.0 + a0 / b0);
        }

        // Algorithm for 1 < b0 < 8
        let n = (b0 - 1.0).trunc() as i32;
        let u = if n >= 1 {
            let mut c = 1.0;
            for _i in 0..n {
                b0 -= 1.0;
                c *= b0 / (a0 + b0);
            }

            c.ln() + gamln1(a0)
        } else {
            gamln1(a0)
        };

        z -= u;
        b0 -= 1.0;
        let apb = a0 + b0;
        let t = if apb > 1.0 {
            let u = a0 + b0 - 1.0;

            (1.0 + gam1(u)) / apb
        } else {
            1.0 + gam1(apb)
        };

        return (a0 * z.exp() * (1.0 + gam1(b0))) / t;
    }

    // Procedure for a >= 8 and b >= 8
    let h = if a > b { b / a } else { a / b };
    let x0 = if a > b {
        1.0 / (1.0 + h)
    } else {
        h / (1.0 + h)
    };
    let y0 = if a > b {
        h / (1.0 + h)
    } else {
        1.0 / (1.0 + h)
    };
    let lambda = if a > b {
        (a + b) * y - b
    } else {
        a - (a + b) * x
    };

    let e1 = -lambda / a;
    let u = if e1.abs() > 0.6 {
        e1 - (x / x0).ln()
    } else {
        rlog1(e1)
    };

    let e2 = lambda / b;
    let v = if e2.abs() > 0.6 {
        e2 - (y / y0).ln()
    } else {
        rlog1(e2)
    };

    let z = (-(a * u + b * v)).exp();
    CONST * (b * x0).sqrt() * z * (-bcorr(a, b)).exp()
}

/// Evaluation of exp(mu) * (x^a * y^b / beta(a, b))
fn brcmp1(mu: f64, a: f64, b: f64, x: f64, y: f64) -> f64 {
    const CONST: f64 = 0.398942280401433e0; // 0.398942280401433 = 1 / sqrt(2 * pi)

    let a0 = a.min(b);
    if a0 < 8.0 {
        let (lnx, lny) = if x <= 0.375 {
            (x.ln(), alnrel(-x))
        } else if y <= 0.375 {
            (alnrel(-y), y.ln())
        } else {
            (x.ln(), y.ln())
        };

        let mut z = a * lnx + b * lny;
        if a0 >= 1.0 {
            z -= betaln(a, b);
            return esum(mu, z);
        }

        // Procedure for a < 1 or b < 1
        let mut b0 = a.max(b);
        if b0 >= 8.0 {
            // Algorithm for b0 >= 8
            let u = gamln1(a0) + algdiv(a0, b0);
            return a0 * esum(mu, z - u);
        }

        if b0 <= 1.0 {
            // Algorithm for b0 <= 1
            let value = esum(mu, z);
            if value == 0.0 {
                return value;
            }

            let apb = a + b;
            let z = if apb > 1.0 {
                let u = a + b - 1.0;

                (1.0 + gam1(u)) / apb
            } else {
                1.0 + gam1(apb)
            };

            let c = ((1.0 + gam1(a)) * (1.0 + gam1(b))) / z;
            return (value * (a0 * c)) / (1.0 + a0 / b0);
        }

        // Algorithm for 1 < b0 < 8
        let n = (b0 - 1.0).trunc() as usize;
        let u = if n >= 1 {
            let mut c = 1.0;
            for _i in 0..n {
                b0 -= 1.0;
                c *= b0 / (a0 + b0);
            }

            c.ln() + gamln1(a0)
        } else {
            gamln1(a0)
        };

        z -= u;
        b0 -= 1.0;
        let apb = a0 + b0;
        let t = if apb > 1.0 {
            let u = a0 + b0 - 1.0;

            (1.0 + gam1(u)) / apb
        } else {
            1.0 + gam1(apb)
        };

        return (a0 * esum(mu, z) * (1.0 + gam1(b0))) / t;
    }

    // Procedure for a >= 8 and b >= 8
    let h = if a > b { b / a } else { a / b };
    let x0 = if a > b {
        1.0 / (1.0 + h)
    } else {
        h / (1.0 + h)
    };
    let y0 = if a > b {
        h / (1.0 + h)
    } else {
        1.0 / (1.0 + h)
    };
    let lambda = if a > b {
        (a + b) * y - b
    } else {
        a - (a + b) * x
    };

    let e1 = -lambda / a;
    let u = if e1.abs() > 0.6 {
        e1 - (x / x0).ln()
    } else {
        rlog1(e1)
    };

    let e2 = lambda / b;
    let v = if e2.abs() > 0.6 {
        e2 - (y / y0).ln()
    } else {
        rlog1(e2)
    };

    let z = esum(mu, -(a * u + b * v));
    CONST * (b * x0).sqrt() * z * (-bcorr(a, b)).exp()
}

/// Asymptotic expansion for I_x(a, b) when a is larger than b.
/// The result of the expansion is added to w. It is assumed
/// that a >= 15 and b <= 1. eps is the tolerance used.
/// ierr is a variable that reports the status of the results. (Replaced with throw in this implementation)
fn bgrat(
    a: f64,
    b: f64,
    x: f64,
    y: f64,
    w: f64,
    eps: f64,
) -> Result<f64, BetaError> {
    let bm1 = (b - 0.5) - 0.5;
    let nu = a + 0.5 * bm1;

    let lnx = if y > 0.375 { x.ln() } else { alnrel(-y) };
    let z = -nu * lnx;

    if b * z == 0.0 {
        return Err(Internal("bgrat: b*z == 0 underflow"));
    }

    // Computation of the expansion set r = exp(-z) * z^b / gamma(b)
    let mut r = b * (1.0 + gam1(b)) * (b * z.ln()).exp();
    r = r * (a * lnx).exp() * (0.5 * bm1 * lnx).exp();
    let mut u = algdiv(b, a) + b * nu.ln();
    u = r * (-u).exp();
    if u == 0.0 {
        return Err(Internal("bgrat: log(u) underflow"));
    }

    let (_p, q) = grat1(b, z, r, eps);

    let v = 0.25 * (1.0 / nu).powi(2);
    let t2 = 0.25 * lnx * lnx;
    let l = w / u;

    let mut j = q / r;
    let mut sum = j;
    let mut t = 1.0;
    let mut cn = 1.0;
    let mut n2 = 0.0;

    const ARRAY_LEN: usize = 30;
    let mut c = [0.0_f64; ARRAY_LEN];
    let mut d = [0.0_f64; ARRAY_LEN];

    for n in 0..ARRAY_LEN {
        let bp2n = b + n2;
        j = (bp2n * (bp2n + 1.0) * j + (z + bp2n + 1.0) * t) * v;
        n2 += 2.0;
        t *= t2;
        cn /= n2 * (n2 + 1.0);
        c[n] = cn;

        let mut s = 0.0;

        if n != 0 {
            let mut coef = b - ((n + 1) as f64);

            for i in 0..n {
                s += coef * c[i] * d[n - i - 1]; //original: D[N - I - 1]; alt: D[NM1 - I]
                coef += b;
            }
        }

        d[n] = bm1 * cn + s / ((n + 1) as f64);
        let dj = d[n] * j;
        sum += dj;

        if sum <= 0.0 {
            return Err(Internal("bgrat: sum <= 0"));
        }

        if dj.abs() <= eps * (sum + l) {
            break;
        }
    }

    // Add the results to w
    let w = w + u * sum;

    Ok(w)
}

/// Evaluation of the incomplete Gamma ratio funcions
/// P(a, x) and Q(a, x)
///
/// It is assumed that a <= 1. eps is the tolerance to be used.
/// The input argument r has the value e^(-x) * x^a / Gamma(a)
fn grat1(a: f64, x: f64, r: f64, eps: f64) -> (f64, f64) {
    if a * x == 0.0 {
        return if x <= a { (0.0, 1.0) } else { (1.0, 0.0) };
    }
    if a == 0.5 {
        if x >= 0.25 {
            let q = erfc1(0, x.sqrt());
            let p = 0.5 + (0.5 - q);
            return (p, q);
        }
        let p = erf(x.sqrt());
        let q = 0.5 + (0.5 - p);
        return (p, q);
    }

    if x < 1.1 {
        // Taylor series for P(a, x) / x^a
        let mut an = 3.0;
        let mut c = x;
        let mut sum = x / (a + 3.0);
        let tol = (0.1 * eps) / (a + 1.0);

        loop {
            an += 1.0;
            c = -c * (x / an);
            let t = c / (a + an);
            sum += t;

            if t.abs() <= tol {
                break;
            }
        }

        let j = a * x * ((sum / 6.0 - 0.5 / (a + 2.0)) * x + 1.0 / (a + 1.0));

        let z = a * x.ln();
        let h = gam1(a);
        let g = 1.0 + h;

        if (x < 0.25 && z > -0.13394) || (x >= 0.25 && a < x / 2.59) {
            let l = rexp(z);
            let w = 0.5 + (0.5 + l);

            let q = (w * j - l) * g - h;
            if q < 0.0 {
                return (1.0, 0.0);
            }

            let p = 0.5 + (0.5 - q);
            return (p, q);
        } else {
            let w = z.exp();
            let p = w * g * (0.5 + (0.5 - j));
            let q = 0.5 + (0.5 - p);
            return (p, q);
        }
    }

    //    Continued fraction expansion
    let mut a2nm1 = 1.0;
    let mut a2n = 1.0;
    let mut b2nm1 = x;
    let mut b2n = x + (1.0 - a);
    let mut c = 1.0;
    let mut an0;

    loop {
        a2nm1 = x * a2n + c * a2nm1;
        b2nm1 = x * b2n + c * b2nm1;
        let am0 = a2nm1 / b2nm1;
        c += 1.0;
        let cma = c - a;
        a2n = a2nm1 + cma * a2n;
        b2n = b2nm1 + cma * b2n;
        an0 = a2n / b2n;

        if (an0 - am0).abs() <= eps * an0 {
            break;
        }
    }

    let q = r * an0;
    let p = 0.5 + (0.5 - q);
    (p, q)
}

/// Asymptotic expansion for I_x(a, b) for large a and b.
/// Lambda = (a + b) * y - b and eps is the tolerance used.
/// It is assumed that lambda is nonnegative and that
/// a and b are greater than or equal to 15.
fn basym(a: f64, b: f64, lambda: f64, eps: f64) -> f64 {
    // num is the maximum value that n can take in the do loop
    // ending at statement 50 (original label). It is required that num be even.
    // The arrays a0, b0, c, d have Dimension num + 1
    const NUM: usize = 20;
    const E0: f64 = FRAC_2_SQRT_PI; // 1.12837916709551 = 2 / sqrt(pi)
    const E1: f64 = 0.353553390593274e0; // 0.353553390593274 = 2 ^(-3 / 2)

    let mut a0 = [0.0_f64; NUM + 1];
    let mut b0 = [0.0_f64; NUM + 1];
    let mut c = [0.0_f64; NUM + 1];
    let mut d = [0.0_f64; NUM + 1];

    let (h, r0, r1, w0) = if a >= b {
        let h = b / a;
        let r0 = 1.0 / (1.0 + h);
        let r1 = (b - a) / a;
        let w0 = 1.0 / (b * (1.0 + h)).sqrt();

        (h, r0, r1, w0)
    } else {
        let h = a / b;
        let r0 = 1.0 / (1.0 + h);
        let r1 = (b - a) / b;
        let w0 = 1.0 / (a * (1.0 + h)).sqrt();

        (h, r0, r1, w0)
    };

    let f = a * rlog1(-lambda / a) + b * rlog1(lambda / b);
    let t = (-f).exp();
    if t == 0.0 {
        return 0.0;
    }

    let z0 = f.sqrt();
    let z = 0.5 * (z0 / E1);
    let z2 = f + f;

    a0[0] = (2.0 / 3.0) * r1;
    c[0] = -0.5 * a0[0];
    d[0] = -c[0];

    let h2 = h * h;
    let mut hh = 1.0;
    let mut s = 1.0;

    let mut j0 = (0.5 / E0) * erfc1(1, z0);
    let mut j1 = E1;
    let mut znm1 = z;
    let mut zn = z2;
    let mut w = w0;
    let mut sum = j0 + d[0] * w0 * j1;

    for n in (2..=NUM).step_by(2) {
        hh *= h2;
        a0[n - 1] = (2.0 * r0 * (1.0 + h * hh)) / (n + 2) as f64;
        s += hh;
        a0[n] = (2.0 * r1 * s) / (n + 3) as f64;
        let np1 = n + 1;

        for i in n..=np1 {
            let r = -0.5 * (i + 1) as f64;
            b0[0] = r * a0[0];

            for m in 2..=i {
                let mut bsum = 0.0;
                let mm1 = m - 1;

                for j in 1..=mm1 {
                    let mmj = m - j;
                    bsum +=
                        (j as f64 * r - mmj as f64) * a0[j - 1] * b0[mmj - 1];
                }

                b0[m - 1] = r * a0[m - 1] + bsum / m as f64;
            }

            let im1 = i - 1;
            c[im1] = b0[im1] / (i + 1) as f64;
            let mut dsum = 0.0;

            for j in 1..=im1 {
                let imj = i - j;
                dsum += d[imj - 1] * c[j - 1];
            }

            d[im1] = -(dsum + c[im1]);
        }

        j0 = E1 * znm1 + (n - 1) as f64 * j0;
        j1 = E1 * zn + n as f64 * j1;
        znm1 *= z2;
        zn *= z2;
        w *= w0;
        let t0 = d[n - 1] * w * j0;
        w *= w0;
        let t1 = d[n] * w * j1;
        sum += t0 + t1;

        if t0.abs() + t1.abs() <= eps * sum {
            break;
        }
    }

    let u = (-bcorr(a, b)).exp();
    E0 * t * u * sum
}

/// If l = 0 then exparg(l) = the largest positive w for which
/// exp(w) can be computed.
///
/// If l is nonzero then exparg(l) = the largest negative w for
/// which the computed value of exp(w) is nonzero.
///
/// Note... only an approximate value for exparg(l) is needed.
fn exparg(l: i32) -> f64 {
    // B = IPMPAR(4) = returns the base, nowadays base = 2
    const LNB: f64 = LN_2; // ln(2) 0.69314718055995e0

    let m = if l == 0 {
        1023 // IPMPAR(7) = the largest exponent (here double)
    } else {
        -1023 // IPMPAR(6) - 1 = the smallest exponent - 1 (here double)
    };

    0.99999 * ((m as f64) * LNB)
}

/// Evaluation of exp(mu + x)
fn esum(mu: f64, x: f64) -> f64 {
    if x > 0.0 {
        let w = mu + x;
        if mu <= 0.0 && w >= 0.0 {
            return w.exp();
        }
        return mu.exp() * x.exp();
    }

    if mu >= 0.0 {
        let w = mu + x;
        if w <= 0.0 {
            return w.exp();
        }
    }

    mu.exp() * x.exp()
}

/// Evaluation of the function exp(x) - 1
fn rexp(x: f64) -> f64 {
    const P1: f64 = 0.914041914819518e-9;
    const P2: f64 = 0.238082361044469e-1;
    const Q1: f64 = -0.499999999085958e0;
    const Q2: f64 = 0.107141568980644e0;
    const Q3: f64 = -0.119041179760821e-1;
    const Q4: f64 = 0.595130811860248e-3;

    if x.abs() <= 0.15 {
        return x
            * (((P2 * x + P1) * x + 1.0)
                / ((((Q4 * x + Q3) * x + Q2) * x + Q1) * x + 1.0));
    }

    let w = x.exp();
    if x <= 0.0 {
        return (w - 0.5) - 0.5;
    }

    w * (0.5 + (0.5 - 1.0 / w))
}

/// Evaluation of the function ln(1 + a)
fn alnrel(a: f64) -> f64 {
    const P1: f64 = -0.129418923021993e1;
    const P2: f64 = 0.405303492862024e0;
    const P3: f64 = -0.178874546012214e-1;
    const Q1: f64 = -0.162752256355323e1;
    const Q2: f64 = 0.747811014037616e0;
    const Q3: f64 = -0.845104217945565e-1;

    if a.abs() <= 0.375 {
        let t = a / (a + 2.0);
        let t2 = t * t;
        let w = (((P3 * t2 + P2) * t2 + P1) * t2 + 1.0)
            / (((Q3 * t2 + Q2) * t2 + Q1) * t2 + 1.0);
        return 2.0 * t * w;
    }

    let x = 1.0 + a;

    x.ln()
}

/// Evaluation of the function x - ln(1 + x)
fn rlog1(x: f64) -> f64 {
    const A: f64 = 0.566749439387324e-1;
    const B: f64 = 0.456512608815524e-1;
    const P0: f64 = 0.333333333333333e0;
    const P1: f64 = -0.224696413112536e0;
    const P2: f64 = 0.620886815375787e-2;
    const Q1: f64 = -0.127408923933623e1;
    const Q2: f64 = 0.354508718369557e0;

    if !(-0.39..=0.57).contains(&x) {
        let w = (x + 0.5) + 0.5;
        return x - w.ln();
    }

    // Argument reduction
    let (h, w1) = if x < -0.18 {
        let mut h = x + 0.3;
        h /= 0.7;
        let w1 = A - h * 0.3;

        (h, w1)
    } else if x > 0.18 {
        let h = 0.75 * x - 0.25;
        let w1 = B + h / 3.0;

        (h, w1)
    } else {
        let h = x;
        let w1 = 0.0;

        (h, w1)
    };

    // Series expansion
    let r = h / (h + 2.0);
    let t = r * r;
    let w = ((P2 * t + P1) * t + P0) / ((Q2 * t + Q1) * t + 1.0);

    2.0 * t * (1.0 / (1.0 - r) - r * w) + w1
}

/// Evaluation of the real Error function
fn erf(x: f64) -> f64 {
    const C: f64 = 0.564189583547756;

    const A: [f64; 5] = [
        0.77105849500132e-4,
        -0.133733772997339e-2,
        0.323076579225834e-1,
        0.479137145607681e-1,
        0.128379167095513,
    ];
    const B: [f64; 3] = [
        0.301048631703895e-2,
        0.538971687740286e-1,
        0.375795757275549,
    ];

    const P: [f64; 8] = [
        -1.36864857382717e-7,
        5.64195517478974e-1,
        7.21175825088309,
        4.31622272220567e1,
        1.5298928504694e2,
        3.39320816734344e2,
        4.51918953711873e2,
        3.00459261020162e2,
    ];
    const Q: [f64; 8] = [
        1.00000000000000e0,
        1.27827273196294e1,
        7.70001529352295e1,
        2.77585444743988e2,
        6.38980264465631e2,
        9.3135409485061e2,
        7.90950925327898e2,
        3.00459260956983e2,
    ];

    const R: [f64; 5] = [
        2.10144126479064,
        2.62370141675169e1,
        2.13688200555087e1,
        4.6580782871847,
        2.82094791773523e-1,
    ];
    const S: [f64; 4] = [
        9.4153775055546e1,
        1.8711481179959e2,
        9.90191814623914e1,
        1.80124575948747e1,
    ];

    let ax = x.abs();
    if ax <= 0.5 {
        let t = x * x;
        let top = (((A[0] * t + A[1]) * t + A[2]) * t + A[3]) * t + A[4] + 1.0;
        let bot = ((B[0] * t + B[1]) * t + B[2]) * t + 1.0;

        return x * (top / bot);
    }

    if ax <= 4.0 {
        let top = ((((((P[0] * ax + P[1]) * ax + P[2]) * ax + P[3]) * ax
            + P[4])
            * ax
            + P[5])
            * ax
            + P[6])
            * ax
            + P[7];
        let bot = ((((((Q[0] * ax + Q[1]) * ax + Q[2]) * ax + Q[3]) * ax
            + Q[4])
            * ax
            + Q[5])
            * ax
            + Q[6])
            * ax
            + Q[7];
        let value = 0.5 + (0.5 - ((-x * x).exp() * top) / bot);
        return if x < 0.0 { -value } else { value };
    }

    if ax >= 5.8 {
        return if x >= 0.0 { 1.0 } else { -1.0 };
    }

    let x2 = x * x;
    let t = 1.0 / x2;
    let top = (((R[0] * t + R[1]) * t + R[2]) * t + R[3]) * t + R[4];
    let bot = (((S[0] * t + S[1]) * t + S[2]) * t + S[3]) * t + 1.0;
    let mut value = (C - top / (x2 * bot)) / ax;
    value = 0.5 + (0.5 - (-x2).exp() * value);
    if x < 0.0 { -value } else { value }
}

/// Evaluation of the complementary Error function
///
/// erfc1(ind, x) = erfc(x)                if ind = 0
///
/// erfc1(ind, x) = exp(x * x) * erfc(x)    otherwise
fn erfc1(ind: usize, x: f64) -> f64 {
    const C: f64 = 0.564189583547756;

    const A: [f64; 5] = [
        0.77105849500132e-4,
        -0.133733772997339e-2,
        0.323076579225834e-1,
        0.479137145607681e-1,
        0.128379167095513,
    ];
    const B: [f64; 3] = [
        0.301048631703895e-2,
        0.538971687740286e-1,
        0.375795757275549,
    ];

    const P: [f64; 8] = [
        -1.36864857382717e-7,
        5.64195517478974e-1,
        7.21175825088309,
        4.31622272220567e1,
        1.5298928504694e2,
        3.39320816734344e2,
        4.51918953711873e2,
        3.00459261020162e2,
    ];
    const Q: [f64; 8] = [
        1.00000000000000e0,
        1.27827273196294e1,
        7.70001529352295e1,
        2.77585444743988e2,
        6.38980264465631e2,
        9.3135409485061e2,
        7.90950925327898e2,
        3.00459260956983e2,
    ];

    const R: [f64; 5] = [
        2.10144126479064,
        2.62370141675169e1,
        2.13688200555087e1,
        4.6580782871847,
        2.82094791773523e-1,
    ];
    const S: [f64; 4] = [
        9.4153775055546e1,
        1.8711481179959e2,
        9.90191814623914e1,
        1.80124575948747e1,
    ];

    // abs(x) <= 0.5
    let ax = x.abs();
    if ax <= 0.5 {
        let t = x * x;
        let top = (((A[0] * t + A[1]) * t + A[2]) * t + A[3]) * t + A[4] + 1.0;
        let bot = ((B[0] * t + B[1]) * t + B[2]) * t + 1.0;
        let value = 0.5 + (0.5 - x * (top / bot));

        return if ind != 0 { t.exp() * value } else { value };
    }

    // 0.5 < abs(x) <= 4
    let mut value = if ax <= 4.0 {
        let top = ((((((P[0] * ax + P[1]) * ax + P[2]) * ax + P[3]) * ax
            + P[4])
            * ax
            + P[5])
            * ax
            + P[6])
            * ax
            + P[7];
        let bot = ((((((Q[0] * ax + Q[1]) * ax + Q[2]) * ax + Q[3]) * ax
            + Q[4])
            * ax
            + Q[5])
            * ax
            + Q[6])
            * ax
            + Q[7];

        top / bot
    } else {
        // abs(x) > 4
        if x <= -5.6 {
            // Limit value for large negative x
            return if ind != 0 { 2.0 * (x * x).exp() } else { 2.0 };
        }
        if ind == 0 && (x > 100.0 || x * x > -exparg(1)) {
            // Limit value for large positive x when ind = 0
            return 0.0;
        }

        let t = (1.0 / x).powi(2);
        let top = (((R[0] * t + R[1]) * t + R[2]) * t + R[3]) * t + R[4];
        let bot = (((S[0] * t + S[1]) * t + S[2]) * t + S[3]) * t + 1.0;

        (C - (t * top) / bot) / ax
    };

    // Final assembly
    if ind != 0 {
        return if x < 0.0 {
            2.0 * (x * x).exp() - value
        } else {
            value
        };
    }

    let w = x * x;
    let t = w; // original: float (32 bit precision)
    let e = w - t;

    value *= (0.5 + (0.5 - e)) * (-t).exp();
    if x < 0.0 { 2.0 - value } else { value }
}

/// Computation of 1/Gamma(a + 1) - 1 for -0.5 <= a <= 1.5
fn gam1(a: f64) -> f64 {
    const P: [f64; 7] = [
        0.577215664901533e0,
        -0.409078193005776e0,
        -0.230975380857675e0,
        0.597275330452234e-1,
        0.766968181649490e-2,
        -0.514889771323592e-2,
        0.589597428611429e-3,
    ];
    const Q: [f64; 5] = [
        0.100000000000000e1,
        0.427569613095214e0,
        0.158451672430138e0,
        0.261132021441447e-1,
        0.423244297896961e-2,
    ];
    const R: [f64; 9] = [
        -0.422784335098468e0,
        -0.771330383816272e0,
        -0.244757765222226e0,
        0.118378989872749e0,
        0.930357293360349e-3,
        -0.118290993445146e-1,
        0.223047661158249e-2,
        0.266505979058923e-3,
        -0.132674909766242e-3,
    ];

    const S1: f64 = 0.273076135303957e0;
    const S2: f64 = 0.559398236957378e-1;

    let d = a - 0.5;
    let t = if d > 0.0 { d - 0.5 } else { a };

    if t == 0.0 {
        return 0.0;
    }

    if t > 0.0 {
        let top = (((((P[6] * t + P[5]) * t + P[4]) * t + P[3]) * t + P[2])
            * t
            + P[1])
            * t
            + P[0];
        let bot = (((Q[4] * t + Q[3]) * t + Q[2]) * t + Q[1]) * t + 1.0;
        let w = top / bot;

        if d > 0.0 {
            return (t / a) * ((w - 0.5) - 0.5);
        }
        return a * w;
    }

    // t < 0
    let top = (((((((R[8] * t + R[7]) * t + R[6]) * t + R[5]) * t + R[4])
        * t
        + R[3])
        * t
        + R[2])
        * t
        + R[1])
        * t
        + R[0];
    let bot = (S2 * t + S1) * t + 1.0;
    let w = top / bot;

    if d > 0.0 {
        return (t * w) / a;
    }
    a * ((w + 0.5) + 0.5)
}

/// Evaluation of ln(Gamma(1 + a)) for -0.2 <= a <= 1.25
fn gamln1(a: f64) -> f64 {
    const P0: f64 = 0.577215664901533e0;
    const P1: f64 = 0.844203922187225e0;
    const P2: f64 = -0.168860593646662e0;
    const P3: f64 = -0.780427615533591e0;
    const P4: f64 = -0.402055799310489e0;
    const P5: f64 = -0.673562214325671e-1;
    const P6: f64 = -0.271935708322958e-2;

    const Q1: f64 = 0.288743195473681e1;
    const Q2: f64 = 0.312755088914843e1;
    const Q3: f64 = 0.156875193295039e1;
    const Q4: f64 = 0.361951990101499e0;
    const Q5: f64 = 0.325038868253937e-1;
    const Q6: f64 = 0.667465618796164e-3;

    const R0: f64 = 0.422784335098467e0;
    const R1: f64 = 0.848044614534529e0;
    const R2: f64 = 0.565221050691933e0;
    const R3: f64 = 0.156513060486551e0;
    const R4: f64 = 0.17050248402265e-1;
    const R5: f64 = 0.497958207639485e-3;

    const S1: f64 = 0.124313399877507e1;
    const S2: f64 = 0.548042109832463e0;
    const S3: f64 = 0.101552187439830e0;
    const S4: f64 = 0.713309612391000e-2;
    const S5: f64 = 0.116165475989616e-3;

    if a < 0.6 {
        let w = ((((((P6 * a + P5) * a + P4) * a + P3) * a + P2) * a + P1) * a
            + P0)
            / ((((((Q6 * a + Q5) * a + Q4) * a + Q3) * a + Q2) * a + Q1) * a
                + 1.0);
        return -a * w;
    }

    let x = (a - 0.5) - 0.5;
    let w = (((((R5 * x + R4) * x + R3) * x + R2) * x + R1) * x + R0)
        / (((((S5 * x + S4) * x + S3) * x + S2) * x + S1) * x + 1.0);
    x * w
}

/// Evaluation of the Digamma function
///
/// psi(xx) is assigned the value 0 when the Digamma function cannot
/// be computed.
///
/// The main computation involves evaluation ot the rational Chebyshev
/// approximations published in Math. Comp. 27, 123-127 (1973) by
/// Cody, Strecok and Thacher.
///
/// The original PSI function was written at Argonne National Laboratory for the Funpack
/// package of special function subroutines. PSI was modified by
/// A. H. Morris (NSWC).
fn psi(xx: f64) -> f64 {
    const DX0: f64 = 1.461632144968362341262659542325721325; // zero of Psi to extended precision

    // Coefficients for rational approximation of
    // psi(x) / (x - x0), 0.5 <= x <= 3.0
    const P1: [f64; 7] = [
        0.895385022981970e-2,
        0.477762828042627e1,
        0.142441585084029e3,
        0.118645200713425e4,
        0.363351846806499e4,
        0.413810161269013e4,
        0.130560269827897e4,
    ];
    const Q1: [f64; 6] = [
        0.448452573429826e2,
        0.520752771467162e3,
        0.221000799247830e4,
        0.364127349079381e4,
        0.190831076596300e4,
        0.691091682714533e-5,
    ];

    // Coefficients for rational approximation of
    // psi(x) - ln(x) + 1 / (2 * x), x > 3.0
    const P2: [f64; 4] = [
        -0.212940445131011e1,
        -0.701677227766759e1,
        -0.448616543918019e1,
        -0.648157123766197e0,
    ];
    const Q2: [f64; 4] = [
        0.322703493791143e2,
        0.892920700481861e2,
        0.546117738103215e2,
        0.777788548522962e1,
    ];

    /*
    MACHINE DEPENDENT CONSTANTS ...

    XMAX1 = The smallest positive floating point
            constant with entirely integer representation. Also used
            as negative lower bound on acceptable negative
            arguments and as the positive argument beyond which
            psi may be represented as ln(x).

    XSMALL = Absolute argument below which
            pi*cot(pi*x) may be represented by 1/x.
    */
    const XMAX1: f64 = (i32::MAX as f64).min(1.0 / f64::EPSILON); //Originally: AMIN1(IPMPAR(3), 1.0/SPMPAR(1))
    const XSMALL: f64 = 1.0e-9;

    let mut x = xx;
    let mut aug = 0.0;
    if x < 0.5 {
        // x < 0.5, use reflection formula
        // psi(1 - x) = psi(x) + pi * contan(pi * x)
        if x.abs() <= XSMALL {
            if x == 0.0 {
                // Error return
                return 0.0;
            }

            // 0 < abs(x) <= XSMALL. Use 1 / x as a substitute
            // for pi * contan(pi * x)
            aug = -1.0 / x;
        } else {
            // Reduction of argument for cotan
            let mut w = -x;
            let mut sgn = FRAC_PI_4;
            if w <= 0.0 {
                w = -w;
                sgn = -sgn;
            }

            // Make an error exit if x <= -XMAX1
            if w >= XMAX1 {
                // Error return
                return 0.0;
            }

            let mut nq = w.trunc() as i32; // int
            w -= nq as f64; // double
            nq = (w * 4.0).trunc() as i32; // int
            w = 4.0 * (w - (nq as f64) * 0.25); // double

            /*
                Variable w is now related to the fractional part of 4.0 * x.
                Adjust argument to correspons to values in first
                quadrant and determine sign.
            */

            let n = nq / 2;
            if n + n != nq {
                w = 1.0 - w;
            }

            let z = FRAC_PI_4 * w;
            let mut m = n / 2;
            if m + m != n {
                sgn = -sgn;
            }

            // Determine final value for -pi * contan(pi * x)
            let n = (nq + 1) / 2;
            m = n / 2;
            m = m + m;

            aug = if m == n {
                // Check for singularity
                if z == 0.0 {
                    // Error return
                    return 0.0;
                }

                // Use cos/sin as a substitute for cotan,
                // and sin/cos as a substitute for tan
                sgn * ((z.cos() / z.sin()) * 4.0)
            } else {
                sgn * ((z.sin() / z.cos()) * 4.0)
            };
        }

        x = 1.0 - x;
    }

    if x <= 3.0 {
        // 0.5 <= x <= 3.0
        let mut den = x;
        let mut upper = P1[0] * x;

        for i in 1..=5 {
            den = (den + Q1[i - 1]) * x;
            upper = (upper + P1[i]) * x;
        }

        den = (upper + P1[6]) / (den + Q1[5]);
        let xmx0 = x - DX0;

        return den * xmx0 + aug;
    }

    // If x >= xmax1, psi = ln(x)
    if x < XMAX1 {
        // 3.0 < x < xmax1
        let w = 1.0 / (x * x);
        let mut den = w;
        let mut upper = P2[0] * w;

        for i in 1..=3 {
            den = (den + Q2[i - 1]) * w;
            upper = (upper + P2[i]) * w;
        }

        aug += upper / (den + Q2[3]) - 0.5 / x;
    }

    aug + x.ln()
}

/// Evaluation of the logarithm of the Beta function.
fn betaln(a0: f64, b0: f64) -> f64 {
    const E: f64 = 0.918938533204673; // 0.5 * ln(2 * pi)

    let mut a = a0.min(b0);
    let mut b = a0.max(b0);

    if a >= 8.0 {
        // Procedure when a >= 8
        let w = bcorr(a, b);
        let h = a / b;
        let c = h / (1.0 + h);
        let u = -(a - 0.5) * c.ln();
        let v = b * alnrel(h);

        return if u <= v {
            (((-0.5 * b.ln() + E) + w) - u) - v
        } else {
            (((-0.5 * b.ln() + E) + w) - v) - u
        };
    }

    if a < 1.0 {
        // Procedure when a < 1
        return if b >= 8.0 {
            gamln(a) + algdiv(a, b)
        } else {
            gamln(a) + (gamln(b) - gamln(a + b))
        };
    }

    // Procedure when 1 <= a < 8
    let mut w;

    if a > 2.0 {
        let n = a - 1.0;
        let n_int = n.trunc() as usize;

        if b > 1000.0 {
            // Reduction of a when b > 1000
            w = 1.0;
            for _i in 1..=n_int {
                a -= 1.0;
                w *= a / (1.0 + a / b);
            }

            return w.ln() - n * b.ln() + (gamln(a) + algdiv(a, b));
        }

        // Reduction of a when b <= 1000
        w = 1.0;
        for _i in 1..=n_int {
            a -= 1.0;
            let h = a / b;
            w *= h / (1.0 + h);
        }

        w = w.ln();
        if b >= 8.0 {
            return w + gamln(a) + algdiv(a, b);
        }
    } else {
        // Procedure when 1 <= a <= 2
        if b <= 2.0 {
            return gamln(a) + gamln(b) - gsumln(a, b);
        }
        w = 0.0;
        if b >= 8.0 {
            return gamln(a) + algdiv(a, b);
        }
    }

    // Reduction of b when b < 8
    let n = b - 1.0;
    let n_int = n.trunc() as usize;
    let mut z = 1.0;
    for _i in 1..=n_int {
        b -= 1.0;
        z *= b / (a + b);
    }

    w + z.ln() + (gamln(a) + (gamln(b) - gsumln(a, b)))
}

/// Evaluation of the function ln(Gamma(a + b))
/// for 1 <= a <= 2 and 1 <= b <= 2.
fn gsumln(a: f64, b: f64) -> f64 {
    let x = a + b - 2.0;
    if x <= 0.25 {
        return gamln1(1.0 + x);
    }
    if x <= 1.25 {
        return gamln1(x) + alnrel(x);
    }
    gamln1(x - 1.0) + (x * (1.0 + x)).ln()
}

/// Evaluation of del(a0) + del(b0) - del(a0 + b0) where
/// ln(Gamma(a)) = (a - 0.5) * ln(a) - a + 0.5 * ln(2 * pi) + del(a).
/// It is assumed that a0 >= 8 and b0 >= 8.
fn bcorr(a0: f64, b0: f64) -> f64 {
    const C0: f64 = 0.833333333333333e-1;
    const C1: f64 = -0.277777777760991e-2;
    const C2: f64 = 0.793650666825390e-3;
    const C3: f64 = -0.595202931351870e-3;
    const C4: f64 = 0.837308034031215e-3;
    const C5: f64 = -0.165322962780713e-2;

    let a = a0.min(b0);
    let b = a0.max(b0);

    let h = a / b;
    let c = h / (1.0 + h);
    let x = 1.0 / (1.0 + h);
    let x2 = x * x;

    // Set sn = (1 - x^n) / (1 - x)
    let s3 = 1.0 + (x + x2);
    let s5 = 1.0 + (x + x2 * s3);
    let s7 = 1.0 + (x + x2 * s5);
    let s9 = 1.0 + (x + x2 * s7);
    let s11 = 1.0 + (x + x2 * s9);

    // Set w = del(b) - del(a + b)
    let t = (1.0 / b).powi(2);
    let mut w = ((((C5 * s11 * t + C4 * s9) * t + C3 * s7) * t + C2 * s5) * t
        + C1 * s3)
        * t
        + C0;
    w *= c / b;

    // Compute del(a) + w
    let t = (1.0 / a).powi(2);
    (((((C5 * t + C4) * t + C3) * t + C2) * t + C1) * t + C0) / a + w
}

/// Computation of ln(Gamma(b)/Gamma(a + b)) when b >= 8
///
/// In this algorithm, del(x) is the function defined by
/// ln(Gamma(x)) = (x - 0.5) * ln(x) - x + 0.5 * ln(2 * pi) + del(x).
fn algdiv(a: f64, b: f64) -> f64 {
    const C0: f64 = 0.833333333333333e-1;
    const C1: f64 = -0.277777777760991e-2;
    const C2: f64 = 0.793650666825390e-3;
    const C3: f64 = -0.595202931351870e-3;
    const C4: f64 = 0.837308034031215e-3;
    const C5: f64 = -0.165322962780713e-2;

    let (c, x, d) = if a <= b {
        let h = a / b;

        let c = h / (1.0 + h);
        let x = 1.0 / (1.0 + h);
        let d = b + (a - 0.5);

        (c, x, d)
    } else {
        let h = b / a;

        let c = 1.0 / (1.0 + h);
        let x = h / (1.0 + h);
        let d = a + (b - 0.5);

        (c, x, d)
    };

    // Set sn = (1 - x^n) / (1 - x)
    let x2 = x * x;
    let s3 = 1.0 + (x + x2);
    let s5 = 1.0 + (x + x2 * s3);
    let s7 = 1.0 + (x + x2 * s5);
    let s9 = 1.0 + (x + x2 * s7);
    let s11 = 1.0 + (x + x2 * s9);

    // Set w = del(b) - del(a + b)
    let t = (1.0 / b).powi(2);
    let mut w = ((((C5 * s11 * t + C4 * s9) * t + C3 * s7) * t + C2 * s5) * t
        + C1 * s3)
        * t
        + C0;
    w *= c / b;

    // Combine the results
    let u = d * alnrel(a / b);
    let v = a * (b.ln() - 1.0);

    if u <= v { (w - u) - v } else { (w - v) - u }
}

/// Evaluation of ln(Gamma(a)) for positive a
fn gamln(a: f64) -> f64 {
    const D: f64 = 0.418938533204673; // 0.5 * (ln(2 * pi) - 1)

    const C0: f64 = 0.833333333333333e-1;
    const C1: f64 = -0.277777777760991e-2;
    const C2: f64 = 0.793650666825390e-3;
    const C3: f64 = -0.595202931351870e-3;
    const C4: f64 = 0.837308034031215e-3;
    const C5: f64 = -0.165322962780713e-2;

    if a <= 0.8 {
        return gamln1(a) - a.ln();
    }

    if a <= 2.25 {
        let t = a - 0.5 - 0.5;
        return gamln1(t);
    }

    if a < 10.0 {
        let n = (a - 1.25).trunc() as usize;
        let mut t = a;
        let mut w = 1.0;

        for _i in 1..=n {
            t -= 1.0;
            w *= t;
        }

        return gamln1(t - 1.0) + w.ln();
    }

    let t = (1.0 / a).powi(2);
    let w = (((((C5 * t + C4) * t + C3) * t + C2) * t + C1) * t + C0) / a;
    D + w + (a - 0.5) * (a.ln() - 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_bratio_eq(
        name: &str,
        x: f64,
        a: f64,
        b: f64,
        expected: (f64, f64),
    ) {
        let y = 1.0 - x;
        let result = bratio(a, b, x, y).unwrap();

        assert_eq!((result.w, result.w1), expected, "{name}");
    }

    fn assert_bratio_error(
        name: &str,
        x: f64,
        a: f64,
        b: f64,
        expected: BetaError,
    ) {
        let y = 1.0 - x;
        let error = match bratio(a, b, x, y) {
            Ok(_) => panic!("{name}: expected error {expected:?}, got Ok"),
            Err(error) => error,
        };

        assert_eq!(error, expected, "{name}");
    }

    // incomplete beta function throw tests
    #[test]
    fn throw_01_negative_a() {
        assert_bratio_error(
            "bratio: a or b is negative",
            0.2,
            -3.0,
            2.0,
            BetaError::NegativeAOrB,
        );
    }

    #[test]
    fn throw_02_negative_b() {
        assert_bratio_error(
            "bratio: a or b is negative",
            0.2,
            3.0,
            -2.0,
            BetaError::NegativeAOrB,
        );
    }

    #[test]
    fn throw_03_a_and_b_zero() {
        assert_bratio_error(
            "bratio: a = b = 0",
            0.2,
            0.0,
            0.0,
            BetaError::BothAAndBZero,
        );
    }

    #[test]
    fn throw_04_negative_x() {
        assert_bratio_error(
            "bratio: x < 0 o x > 1",
            -0.2,
            3.0,
            2.0,
            BetaError::XOutOfRange,
        );
    }

    #[test]
    fn throw_05_x_greater_than_one() {
        assert_bratio_error(
            "bratio: x < 0 or x > 1",
            1.2,
            3.0,
            2.0,
            BetaError::XOutOfRange,
        );
    }

    #[test]
    fn throw_06_x_and_a_zero() {
        assert_bratio_error(
            "bratio: x = a = 0",
            0.0,
            0.0,
            2.0,
            BetaError::XAndAZero,
        );
    }

    #[test]
    fn throw_07_y_and_b_zero() {
        assert_bratio_error(
            "bratio: y = b = 0",
            1.0,
            3.0,
            0.0,
            BetaError::YAndBZero,
        );
    }

    // incomplete beta function trivial cases
    #[test]
    fn trivial_01() {
        assert_bratio_eq(
            "I_{0}(3, 2); 1 - I_{0}(3, 2)",
            0.0,
            3.0,
            2.0,
            (
                0.0, // sagemath: 1.0
                1.0, //sagemath: 0.0
            ),
        );
    }

    #[test]
    fn trivial_02() {
        assert_bratio_eq(
            "I_{1}(3, 2); 1 - I_{1}(3, 2)",
            1.0,
            3.0,
            2.0,
            (
                1.0, // sagemath: 1.0
                0.0, // sagemath: 0.0
            ),
        );
    }

    #[test]
    fn trivial_03() {
        assert_bratio_eq(
            "I_{0.2}(3, 0); 1 - 0I_{0.2}(3, 0)",
            0.2,
            3.0,
            0.0,
            (
                0.0, // sagemath: NaN
                1.0, // sagemath: NaN
            ),
        );
    }

    #[test]
    fn trivial_04() {
        assert_bratio_eq(
            "I_{0.2}(0, 2); 1 - I_{0.2}(0, 2)",
            0.2,
            0.0,
            2.0,
            (
                1.0, // sagemath: NaN
                0.0, // sagemath: NaN
            ),
        );
    }

    // bratio test cases
    #[test]
    fn bratio_00() {
        assert_bratio_eq(
            "I_{0.2}(1e-20, 1e-21); 1 - I_{0.2}(1e-20, 1e-21)",
            0.2,
            1e-20,
            1e-21,
            (
                0.0909090909090909, // sagemath: 0.09090909090909065
                0.9090909090909091, // sagemath: 0.909090909090909
            ),
        );
    }

    #[test]
    fn bratio_01() {
        assert_bratio_eq(
            "I_{0.2}(3, 1e-17); 1 - I_{0.2}(3, 1e-17)",
            0.2,
            3.0,
            1e-17,
            (
                3.143551314209757e-20, // sagemath: 3.143551314209749e-20
                1.0,                   // sagemath: 1.00000000000000
            ),
        );
    }

    #[test]
    fn bratio_02() {
        assert_bratio_eq(
            "I_{0.2}(1e-19, 0.0003); 1 - I_{0.2}(1e-19, 0.0003)",
            0.2,
            1e-19,
            0.0003,
            (
                0.9999999999999997,    // sagemath: 0.9999999999999991
                3.334719205091276e-16, // sagemath: 8.88178419700125e-16
            ),
        );
    }

    #[test]
    fn bratio_03() {
        assert_bratio_eq(
            "I_{0.2}(8.7, 0.6); 1 - I_{0.2}(8.7, 0.6)",
            0.2,
            8.7,
            0.6,
            (
                2.504047825925081e-7, // sagemath: 2.50404782592508e-07
                0.9999997495952174,   // sagemath: 0.999999749595217
            ),
        );
    }

    #[test]
    fn bratio_04() {
        assert_bratio_eq(
            "I_{0.2}(1.7, 0.6); 1 - I_{0.2}(1.7, 0.6)",
            0.2,
            1.7,
            0.6,
            (
                0.034729557265054514, // sagemath: 0.03472955726505452
                0.9652704427349454,   // sagemath: 0.965270442734945
            ),
        );
    }

    #[test]
    fn bratio_05() {
        assert_bratio_eq(
            "I_{0.2}(3.7, 0.6); 1 - I_{0.2}(3.7, 0.6)",
            0.2,
            3.7,
            0.6,
            (
                0.001070573575819432, // sagemath: 0.00107057357581943
                0.9989294264241806,   // sagemath: 0.998929426424181
            ),
        );
    }

    #[test]
    fn bratio_06() {
        assert_bratio_eq(
            "I_{0.3}(0.9, 1.7); 1 - I_{0.3}(0.9, 1.7)",
            0.3,
            0.9,
            1.7,
            (
                0.49669572816874397, // sagemath: 0.4966957281687434
                0.503304271831256,   // sagemath: 0.503304271831257
            ),
        );
    }

    #[test]
    fn bratio_07() {
        assert_bratio_eq(
            "I_{0.01}(0.9, 1.7); 1 - I_{0.01}(0.9, 1.7)",
            0.01,
            0.9,
            1.7,
            (
                0.025841490768663027, // sagemath: 0.025841490768663006
                0.974158509231337,    // sagemath: 0.9741585092313370
            ),
        );
    }

    #[test]
    fn bratio_08() {
        assert_bratio_eq(
            "I_{0.2}(0.9, 15.5); 1 - I_{0.2}(0.9, 15.5)",
            0.2,
            0.9,
            15.5,
            (
                0.974298915074856,   // sagemath: 0.974298915074856
                0.02570108492514396, // sagemath: 0.0257010849251440
            ),
        );
    }

    #[test]
    fn bratio_09() {
        assert_bratio_eq(
            "I_{0.2}(0.5, 15.5); 1 - I_{0.2}(0.5, 15.5)",
            0.2,
            0.5,
            15.5,
            (
                0.9909293561788433,  // sagemath: 0.9909293561788434
                0.00907064382115669, // sagemath: 0.00907064382115658
            ),
        );
    }

    #[test]
    fn bratio_10() {
        assert_bratio_eq(
            "I_{0.2}(0.9, 8.7); 1 - I_{0.2}(0.9, 8.7)",
            0.2,
            0.9,
            8.7,
            (
                0.8774785948777811, // sagemath: 0.8774785948777811
                0.1225214051222189, // sagemath: 0.122521405122219
            ),
        );
    }

    #[test]
    fn bratio_11() {
        assert_bratio_eq(
            "I_{0.2}(0.9, 7); 1 - I_{0.2}(0.9, 7)",
            0.2,
            0.9,
            7.0,
            (
                0.8181890351695966,  // sagemath: 0.818189035169597
                0.18181096483040338, // sagemath: 0.181810964830403
            ),
        );
    }

    #[test]
    fn bratio_12() {
        assert_bratio_eq(
            "I_{0.2}(0.9, 6.9); 1 - I_{0.2}(0.9, 6.9)",
            0.2,
            0.9,
            6.9,
            (
                0.8139029530611517,  // sagemath: 0.813902953061152
                0.18609704693884838, // sagemath: 0.186097046938848
            ),
        );
    }

    #[test]
    fn bratio_13() {
        assert_bratio_eq(
            "I_{0.2}(0.9, 0.4); 1 - I_{0.2}(0.9, 0.4)",
            0.2,
            0.9,
            0.4,
            (
                0.10512446812879857, // sagemath: 0.10512446812879858
                0.8948755318712014,  // sagemath: 0.8948755318712014
            ),
        );
    }

    #[test]
    fn bratio_14() {
        assert_bratio_eq(
            "I_{0.2}(0.1, 0.7); 1 - I_{0.2}(0.1, 0.7)",
            0.2,
            0.1,
            0.7,
            (
                0.8073274503338682, // sagemath: 0.8073274503338692
                0.1926725496661318, // sagemath: 0.1926725496661318
            ),
        );
    }

    #[test]
    fn bratio_15() {
        assert_bratio_eq(
            "I_{0.35}(0.1, 0.7); 1 - I_{0.35}(0.1, 0.7)",
            0.35,
            0.1,
            0.7,
            (
                0.8581064018974903,  // sagemath: 0.8581064018974913
                0.14189359810250968, // sagemath: 0.141893598102509
            ),
        );
    }

    #[test]
    fn bratio_16() {
        assert_bratio_eq(
            "I_{0.2}(0.01, 0.999); 1 - I_{0.2}(0.01, 0.999)",
            0.2,
            0.01,
            0.999,
            (
                0.9840204187494235,   // sagemath: 0.9840204187494239
                0.015979581250576497, // sagemath: 0.0159795812505761
            ),
        );
    }

    #[test]
    fn bratio_17() {
        assert_bratio_eq(
            "I_{0.2}(0.01, 0.7); 1 - I_{0.2}(0.01, 0.7)",
            0.2,
            0.01,
            0.7,
            (
                0.978410014890881,    // sagemath: 0.9784100148908809
                0.021589985109119084, // sagemath: 0.0215899851091191
            ),
        );
    }

    #[test]
    fn bratio_18() {
        assert_bratio_eq(
            "I_{0.01}(0.01, 0.7); 1 - I_{0.01}(0.01, 0.7)",
            0.01,
            0.01,
            0.7,
            (
                0.9489575726627992,  // sagemath: 0.9489575726627996
                0.05104242733720081, // sagemath: 0.0510424273372004
            ),
        );
    }

    #[test]
    fn bratio_19() {
        assert_bratio_eq(
            "I_{0.02}(30, 20); 1 - I_{0.02}(30, 20)",
            0.02,
            30.0,
            20.0,
            (
                1.3963885552371685e-38, // sagemath: 1.3963885552371275e-38
                1.0,                    // sagemath: 1.000000000000000
            ),
        );
    }

    #[test]
    fn bratio_20() {
        assert_bratio_eq(
            "I_{0.2}(1.7, 2); 1 - I_{0.2}(1.7, 2)",
            0.2,
            1.7,
            2.0,
            (
                0.15298998272779685, // sagemath: 0.152989982727797
                0.8470100172722032,  // sagemath: 0.847010017272203
            ),
        );
    }

    #[test]
    fn bratio_21() {
        assert_bratio_eq(
            "I_{0.02}(1.7, 6); 1 - I_{0.02}(1.7, 6)",
            0.02,
            1.7,
            6.0,
            (
                0.01814584484038807, // sagemath: 0.018145844840388045
                0.9818541551596119,  // sagemath: 0.9818541551596120
            ),
        );
    }

    #[test]
    fn bratio_22() {
        assert_bratio_eq(
            "I_{0.2}(30, 20); 1 - I_{0.2}(30, 20)",
            0.2,
            30.0,
            20.0,
            (
                3.436158271932095e-10, // sagemath: 3.4361582719319687e-10
                0.9999999996563842,    // sagemath: 0.9999999996563842
            ),
        );
    }

    #[test]
    fn bratio_23() {
        assert_bratio_eq(
            "I_{0.75}(14, 4); 1 - I_{0.75}(14, 4)",
            0.75,
            14.0,
            4.0,
            (
                0.35301809501834186, //  sagemath: 0.353018095018344
                0.6469819049816581,  // sagemath: 0.646981904981656
            ),
        );
    }

    #[test]
    fn bratio_24() {
        assert_bratio_eq(
            "I_{0.75}(35, 10); 1 - I_{0.75}(35, 10)",
            0.75,
            35.0,
            10.0,
            (
                0.30862539084006774, // sagemath: 0.30862539084006324
                0.6913746091599322,  // sagemath: 0.6913746091599368
            ),
        );
    }

    #[test]
    fn bratio_25() {
        assert_bratio_eq(
            "I_{0.6}(50, 45); 1 - I_{0.6}(50, 45)",
            0.6,
            50.0,
            45.0,
            (
                0.9259884904645385,  // sagemath: 0.925988490464538
                0.07401150953546155, // sagemath: 0.0740115095354620
            ),
        );
    }

    #[test]
    fn bratio_26() {
        assert_bratio_eq(
            "I_{0.99}(41, 7); 1 - I_{0.99}(41, 45)",
            0.99,
            41.0,
            7.0,
            (
                0.9999995570577934,    // sagemath: 0.9999995570577934
                4.4294220655994265e-7, // sagemath: 4.42942206579922e-7
            ),
        );
    }

    #[test]
    fn bratio_27() {
        assert_bratio_eq(
            "I_{0.2}(105, 130); 1 - I_{0.2}(105, 130)",
            0.2,
            105.0,
            130.0,
            (
                7.720561154039475e-18, // sagemath: 7.720561154039562e-18
                1.0,                   // sagemath: 1.00000000000000
            ),
        );
    }

    #[test]
    fn bratio_28() {
        assert_bratio_eq(
            "I_{0.2}(50, 45); 1 - I_{0.2}(50, 45)",
            0.2,
            50.0,
            45.0,
            (
                1.0479945433367573e-12, // sagemath: 1.0479945433367593e-12
                0.999999999998952,
            ), // sagemath: 0.9999999999989520
        );
    }

    #[test]
    fn bratio_29() {
        assert_bratio_eq(
            "I_{0.6}(105, 130); 1 - I_{0.6}(105, 130)",
            0.6,
            105.0,
            130.0,
            (
                0.9999988878041558,    // sagemath: 0.9999988878041558
                1.1121958441800532e-6, // sagemath: 1.11219584419953e-6
            ),
        );
    }

    #[test]
    fn bratio_30() {
        assert_bratio_eq(
            "I_{0.2}(1000, 4001); 1 - I_{0.2}(1000, 4001)",
            0.2,
            1000.0,
            4001.0,
            (
                0.505641740641465,  // sagemath: 0.5056417406424282
                0.4943582593585349, // sagemath: 0.494358259357572
            ),
        );
    }

    #[test]
    fn bratio_31() {
        assert_bratio_eq(
            "I_{0.4}(247.6, 368.9); 1 - I_{0.4}(247.6, 368.9)",
            0.4,
            247.6,
            368.9,
            (
                0.46940376969197334, // sagemath: 0.46940376969211417
                0.5305962303080267,  // sagemath: 0.5305962303078858
            ),
        );
    }

    #[test]
    fn bratio_32() {
        assert_bratio_eq(
            "I_{0.4}(24704.6, 36889.9); 1 - I_{0.4}(24704.6, 36889.9)",
            0.4,
            24704.6,
            36889.9,
            (
                0.29157571565188733, // sagemath: 0.29157571564818835
                0.7084242843481127,  // sagemath: 0.7084242843518117
            ),
        );
    }

    #[test]
    fn bratio_33() {
        assert_bratio_eq(
            "I_{0.87}(10, 1); 1 - I_{0.87}(10, 1)",
            0.87,
            10.0,
            1.0,
            (
                0.2484234141914357, // sagemath: 0.2484234141914352
                0.7515765858085643, // sagemath: 0.751576585808565
            ),
        );
    }

    #[test]
    fn bratio_34() {
        assert_bratio_eq(
            "I_{0.7}(4, 8); 1 - I_{0.7}(4, 8)",
            0.7,
            4.0,
            8.0,
            (
                0.9957091060000001,   // sagemath: 0.9957091060000001
                0.004290894000000007, // sagemath: 0.00429089399999988
            ),
        );
    }
}
