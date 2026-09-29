use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnclosingZeroError {
    #[error("The interval boundaries must be finite.")]
    InfiniteIntervalBoundaries,
    #[error("The condition a < b must be met.")]
    AGreaterOrEqualThanB,
    #[error("The interval contains no zero resulting from a sign change.")]
    IntervalNotEnclosingZero,
}

/// Finds a zero of a continuous function within the interval [a, b].
///
/// The interval must be finite, satisfy a < b, and either contain a zero at
/// one boundary or enclose one through a sign change.
///
/// Returns the exact or approximated zero, or an [EnclosingZeroError] if the
/// interval is invalid or does not enclose a zero.
pub fn enclose_zero<F>(f: &F, a: f64, b: f64) -> Result<f64, EnclosingZeroError>
where
    F: Fn(f64) -> f64,
{
    if !a.is_finite() || !b.is_finite() {
        return Err(EnclosingZeroError::InfiniteIntervalBoundaries);
    }
    if a >= b {
        return Err(EnclosingZeroError::AGreaterOrEqualThanB);
    }

    let fa = f(a);
    let fb = f(b);

    if !fa.is_finite() || !fb.is_finite() {
        return Err(EnclosingZeroError::InfiniteIntervalBoundaries);
    }

    if fa == 0.0 {
        return Ok(a);
    }
    if fb == 0.0 {
        return Ok(b);
    }

    if fa.signum() == fb.signum() {
        return Err(EnclosingZeroError::IntervalNotEnclosingZero);
    }

    let root = rroot(f, a, b, 1000, rmp()).root;

    Ok(root)
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct RootResult {
    root: f64,
    a: f64,
    b: f64,
    iterations: usize,
}

#[derive(Debug, Clone, Copy)]
struct Bracket {
    a: f64,
    b: f64,
    fa: f64,
    fb: f64,
    d: f64,
    fd: f64,
    tol: f64,
}

/*
    Reference:
    Algorithm ENCLOFX
    G. E. Alefeld, F. A. Potra, snd Yixun Shi --- JULY, 1994
    Companion Paper: Enclosing Zeros of Continous Functions
    This Implements the Algorithm 4.2 of the Companion paper.

    Algorithm 748, Collected Algorithms from ACM.
    This work published in Transactions on Mathematical Software,
    vol. 21, no. 3, September, 1995, p. 327-344.

    source: https://doi.org/10.1145/210089.210111
*/

/// Finds either an exact solution or an approximate solution of the
/// equation f(x) = 0 in the interval [a, b], at the beginning of each
/// iteration, the current enclosing interval is recorded as [a0, b0].
/// The first iteration is simply a secant step. Starting with the
/// {second iteration, three steps are taken in each iteration. First
/// two steps are either quardratic interpolation or cubic inverse
/// interpolation. The third step is a double-size secant step. If the
/// diameter of the enclosing interval obtained after those three steps
/// is larger than 0.5 * (b0 * a0), then an additional bisection step will
/// be taken.
///
/// * `nprob` -- Integer. Indicating the problem to be solved;
/// * `neps` -- Integer. Used to determine the termination criterion;
/// * `eps` -- Double precision. Used in the termination criterion;
/// * `a`, `b` -- Double precision. Input as the initial interval and
///   output as the enclosing interval at the termination;
fn rroot<F>(func: &F, a: f64, b: f64, neps: i32, eps: f64) -> RootResult
where
    F: Fn(f64) -> f64,
{
    let mut a = a;
    let mut b = b;

    const MU: f64 = 0.5;

    // Initialization, set the number of iteration as 0. Call subroutine
    // "func" to obtain the initial function values f(a) and f(b). Set
    // dumb values for the variables "e" and "fe".

    let mut itnum = 0;
    let mut fa = func(a);
    let mut fb = func(b);
    let mut e = 1.0e5;
    let mut fe = 1.0e5;
    let mut d = 0.0;
    let mut fd = 0.0;

    // Iteration starts. The enclosing interval before executing the
    // iteration is recorded as [a0, b0].
    loop {
        let a0 = a;
        let b0 = b;

        // Updates the number of iteration.
        itnum += 1;

        // Calculates the termination criterion. Stops the procedure if the
        // criterion is satisfied.
        let mut tol = if fb.abs() <= fa.abs() {
            tole(b, neps, eps)
        } else {
            tole(a, neps, eps)
        };
        if (b - a) <= tol {
            break;
        }

        // For the first iteration, secant step is taken.
        if itnum == 1 {
            let c = a - (fa / (fb - fa)) * (b - a);

            // Call subroutine "brackt" to get a shrinked enclosing interval as
            // well as to update the termination criterion. Stop the procedure
            // if the criterion is satisfied or the exact solution is obtained.
            Bracket {
                a,
                b,
                fa,
                fb,
                d,
                fd,
                tol,
            } = brackt(func, a, b, c, fa, fb, tol, neps, eps);

            if (fa == 0.0) || ((b - a) <= tol) {
                break;
            }
            continue;
        }

        // Starting with the second iteration, in the first two steps, either
        // quardratic interpolation is used by calling the subroutine "NEWQUA"
        // or the cubic inverse interpolation is used by calling the subroutine
        // "PZERO". In the following, if "PROF" is not equal to 0, then the
        // four function values "FA", "FB", "FD" and "FE" are distinct, and
        // hence "PZERO" will be called.

        let prof = (fa - fb)
            * (fa - fd)
            * (fa - fe)
            * (fb - fd)
            * (fb - fe)
            * (fd - fe);
        let c = if (itnum == 2) || (prof == 0.0) {
            newqua(a, b, d, fa, fb, fd, 2)
        } else {
            let c = pzero(a, b, d, e, fa, fb, fd, fe);

            if (c - a) * (c - b) >= 0.0 {
                newqua(a, b, d, fa, fb, fd, 2)
            } else {
                c
            }
        };

        e = d;
        fe = fd;

        // Call subroutine "BRACKT" to get a shrinked enclosing interval
        // well as to update the termination criterion. Stop the procedure
        // if the criterion is satisfied or the exact solution is obtained.
        Bracket {
            a,
            b,
            fa,
            fb,
            d,
            fd,
            tol,
        } = brackt(func, a, b, c, fa, fb, tol, neps, eps);

        if (fa == 0.0) || ((b - a) <= tol) {
            break;
        }

        let prof = (fa - fb)
            * (fa - fd)
            * (fa - fe)
            * (fb - fd)
            * (fb - fe)
            * (fd - fe);
        let c = if prof == 0.0 {
            newqua(a, b, d, fa, fb, fd, 3)
        } else {
            let c = pzero(a, b, d, e, fa, fb, fd, fe);

            if (c - a) * (c - b) >= 0.0 {
                newqua(a, b, d, fa, fb, fd, 3)
            } else {
                c
            }
        };

        // Call subroutine "BRACKT" to get a shrinked enclosing interval as
        // well as to update the termination criterion. Stop the procedure
        // if the criterion is satisfied or the exact solution is obtained.
        Bracket {
            a,
            b,
            fa,
            fb,
            d,
            fd,
            tol,
        } = brackt(func, a, b, c, fa, fb, tol, neps, eps);

        if (fa == 0.0) || ((b - a) <= tol) {
            break;
        }
        e = d;
        fe = fd;

        // Takes the double-size secant step.
        let (u, fu) = if fa.abs() < fb.abs() {
            (a, fa)
        } else {
            (b, fb)
        };
        let mut c = u - 2.0 * (fu / (fb - fa)) * (b - a);
        if (c - u).abs() > (0.5 * (b - a)) {
            c = a + 0.5 * (b - a)
        }

        // Call subroutine "BRACKT" to get a shrinked enclosing interval as
        // well as to update the termination criterion. Stop the procedure
        // if the criterion is satisfied or the exact solution is obtained.
        Bracket {
            a,
            b,
            fa,
            fb,
            d,
            fd,
            tol,
        } = brackt(func, a, b, c, fa, fb, tol, neps, eps);

        if (fa == 0.0) || ((b - a) <= tol) {
            break;
        }

        // Determines whether an additional bisection step is needed. And takes
        // it if necessary.
        if (b - a) < (MU * (b0 - a0)) {
            continue;
        }
        e = d;
        fe = fd;

        // Call subroutine "BRACKT" to get a shrinked enclosing interval as
        // well as to update the termination criterion. Stop the procedure
        // if the criterion is satisfied or the exact solution is obtained.
        Bracket {
            a,
            b,
            fa,
            fb,
            d,
            fd,
            tol,
        } = brackt(func, a, b, a + 0.5 * (b - a), fa, fb, tol, neps, eps);

        if (fa == 0.0) || ((b - a) <= tol) {
            break;
        }
        continue;
        // Terminates the procedure and return the "ROOT".
    }

    RootResult {
        root: a,
        a,
        b,
        iterations: itnum,
    }
}

/// Given current enclosing interval [a, b] and a number c in (a, b), if
/// f(c) = 0 then sets the output a = c. Otherwise determines the new
/// enclosing interval: [a, b] = [a, c] or [a, b] = [c, b]. Also updates the
/// termination criterion corresponding to the new enclosing interval.
///
/// * `nprob` -- Integer. Indicating the problem to be solved;
/// * `a`, `b` -- Double precision. [A, B] is input as the current
///   enclosing interval and output as the shrinked new
///   enclosing interval;
/// * `c` -- Double precision. Used to determine the new enclosing
///   interval;
/// * `d` -- Double precision. Output: If the new enclosing interval
///   is [A, C] then D = B, otherwise D = A;
/// * `fa`, `fb`, `fd` -- Double precision. FA = F(A), FB = F(B), and FD = F(D);
/// * `tol` -- Double precision. Input as the current termination
///   criterion and output as the updated termination
///   criterion according to the new enclosing interval;
/// * `neps` -- Integer. Used to determine the termination criterion;
/// * `eps` -- Double precision. Used in the termination criterion.
#[allow(clippy::too_many_arguments)]
fn brackt<F>(
    func: &F,
    a: f64,
    b: f64,
    c: f64,
    fa: f64,
    fb: f64,
    tol: f64,
    neps: i32,
    eps: f64,
) -> Bracket
where
    F: Fn(f64) -> f64,
{
    let mut a = a;
    let mut b = b;
    let mut c = c;
    let mut fa = fa;
    let mut fb = fb;

    // Adjust c if (b - a) is very small or if c is very close to a or b.
    let tol = 0.7 * tol;
    c = if (b - a) <= 2.0 * tol {
        a + 0.5 * (b - a)
    } else if c <= a + tol {
        a + tol
    } else if c >= b - tol {
        b - tol
    } else {
        c
    };

    // Call subroutine "FUNC" to obtain f(c)
    let fc = func(c);

    let mut d = 0.0;
    let mut fd = 0.0;

    // If f(c) = 0, then set a = c and return. This will terminate the
    // procedure in subroutine "RROOT" and give the exact solution of
    // the equation f(x) = 0.
    if fc == 0.0 {
        return Bracket {
            a: c,
            b,
            fa: 0.0,
            fb,
            d,
            fd,
            tol,
        };
    }

    // If f(c) is not zero, then determine the new enclosing interval.
    if isign(fa) * isign(fc) < 0 {
        d = b;
        fd = fb;
        b = c;
        fb = fc;
    } else {
        d = a;
        fd = fa;
        a = c;
        fa = fc;
    };

    // Update the termination criterion according to the new enclosing
    // interval.
    let tol = if fb.abs() <= fa.abs() {
        tole(b, neps, eps)
    } else {
        tole(a, neps, eps)
    };

    // End of the subroutine.
    Bracket {
        a,
        b,
        fa,
        fb,
        d,
        fd,
        tol,
    }
}

/// Indicates the sign of the variable "x".
///
/// x      -- Double precision.
/// isign  -- Integer.
fn isign(x: f64) -> i8 {
    if x > 0.0 {
        1
    } else if x == 0.0 {
        0
    } else {
        -1
    }
}

/// Determines the termination criterion.
///
/// * `b` -- Double precision.
/// * `neps` -- Integer.
/// * `eps` -- Double precision.
/// * `tol` -- Double precision. Output as the termination criterion.
///   tol = 2 * (2 * eps * |B| + 10 - neps),  if neps is not 1000;
///   and tol = 2 * (2 * eps * |B|),              if neps = 1000;
fn tole(b: f64, neps: i32, eps: f64) -> f64 {
    let mut tol = if neps == 1000 {
        0.0
    } else {
        10.0_f64.powi(-neps)
    };

    tol += 2.0 * b.abs() * eps;

    2.0 * tol
}

/// Uses k Newton steps to approximate the zero in (a, b) of the
/// quadratic polynomial interpolating f(x) at a, b, and d. Safeguard
/// is used to avoid overflow.
///
/// * `a`, `b`, `d`, `fa`, `fb`, `fd` -- Double precision. d lies outside the interval
///   [a, b]. fa = f(a), fb = f(b), and fd = f(d). f(a)f(b) < 0.
/// * `c` -- Double precision. Output as the approximate zero
///   in (a, b) of the quadratic polynomial.
/// * `k` -- Integer. Input indicating the number of Newton
fn newqua(a: f64, b: f64, d: f64, fa: f64, fb: f64, fd: f64, k: isize) -> f64 {
    // Initialization. Find the coefficients of the quadratic polynomial.
    let mut ierror = 0;
    let a0 = fa;
    let a1 = (fb - fa) / (b - a);
    let a2 = ((fd - fb) / (d - b) - a1) / (d - a);

    let mut c;

    // Safeguard to avoid overflow.
    loop {
        if (a2 == 0.0) || (ierror == 1) {
            return a - a0 / a1;
        }

        // Determine the starting point of Newton steps.
        c = if isign(a2) * isign(fa) > 0 { a } else { b };

        // Start the safeguarded Newton steps.
        for _i in 1..=k {
            if ierror == 0 {
                let pc = a0 + (a1 + a2 * (c - b)) * (c - a);
                let pdc = a1 + a2 * ((2.0 * c) - (a + b));
                if pdc == 0.0 {
                    ierror = 1;
                } else {
                    c -= pc / pdc;
                }
            }
        }
        if ierror != 1 {
            break;
        }
    }

    c
}

/// Uses cubic inverse interpolation of f(x) at a, b, d and e to
/// get an approximate root of f(x). This procedure is a slight
/// modification of Aitken-Neville Algorithm for interpolation
/// described by Stoer and Bulirsch in "Intro. to Numerical Analysis"
/// Springer-Verlag, New York (1980).
/// * `a`, `b`, `d`, `e`, `fa`, `fb`, `fd`, `fe` -- Double precision. d and e lie outside
///   the interval [a, b], fa = f(a), fb = f(b),
///   fd = f(d), and fe = f(e).
#[allow(clippy::too_many_arguments)]
fn pzero(
    a: f64,
    b: f64,
    d: f64,
    e: f64,
    fa: f64,
    fb: f64,
    fd: f64,
    fe: f64,
) -> f64 {
    let q11 = (d - e) * fd / (fe - fd);
    let q21 = (b - d) * fb / (fd - fb);
    let q31 = (a - b) * fa / (fb - fa);
    let d21 = (b - d) * fd / (fd - fb);
    let d31 = (a - b) * fb / (fb - fa);
    let q22 = (d21 - q11) * fb / (fe - fb);
    let q32 = (d31 - q21) * fa / (fd - fa);
    let d32 = (d31 - q21) * fd / (fd - fa);
    let q33 = (d32 - q22) * fa / (fe - fa);

    // Calculate the output c.
    let c = q31 + q32 + q33;

    a + c
}

/// Calculates the relative machine precision (RMP).
///
/// REL -- Double precision. Output of RMP.
fn rmp() -> f64 {
    let beta = 2.0;
    let mut a = 1.0;

    loop {
        let b = 1.0 + a;
        if b > 1.0 {
            a /= beta;
        } else {
            return a * beta;
        }
    }
}

/// Regression tests for the Rust port of ACM TOMS Algorithm 748.
///
/// The test data is based on the original Fortran 77 driver and reproduces:
///
/// - the initial intervals defined by `INIT`;
/// - the test functions defined by `FUNC`;
/// - all 154 `(NPROB, N)` combinations from `testdata`;
/// - the historical reference roots listed in `testout`;
/// - the two example problems from `exdrive.f`.
///
/// Each Fortran test function is adapted to the generic solver interface
/// `Fn(f64) -> f64` by capturing `NPROB` and `N` in a Rust closure:
///
/// `ignore
/// let f = move |x| func(case.nprob, x, case.n);
/// let root = enclose_zero(f, a, b);
/// `
///
/// The expected values are the roots printed by the original Fortran test
/// run. They are regression references for the port and are not necessarily
/// the mathematically exact roots.
///
/// Because the Fortran output contains a limited number of decimal digits,
/// the tests compare the computed and expected roots using a small absolute
/// and relative tolerance instead of requiring bitwise-identical `f64`
/// values.
///
/// Case 26 is intentionally compared against the historical Fortran output.
/// Although its function has an exact zero at `x = 0`, the recorded reference
/// value is the approximate result produced by the original Algorithm 748
/// execution.
#[cfg(test)]
mod tests {
    use super::*;
    use core::f64;
    use std::f64::consts::{FRAC_PI_2, PI};

    #[derive(Debug, Clone, Copy)]
    struct TestCase {
        nprob: usize,
        n: i32,
        expected_root: f64,
    }

    fn init(nprob: usize) -> (f64, f64) {
        let (a, b) = match nprob {
            1 => (FRAC_PI_2, PI),
            2 => (1.0 + 1.0e-9, 2.0 * 2.0 - 1.0e-9),
            3 => (2.0 * 2.0 + 1.0e-9, 3.0 * 3.0 - 1.0e-9),
            4 => (3.0 * 3.0 + 1.0e-9, 4.0 * 4.0 - 1.0e-9),
            5 => (4.0 * 4.0 + 1.0e-9, 5.0 * 5.0 - 1.0e-9),
            6 => (5.0 * 5.0 + 1.0e-9, 6.0 * 6.0 - 1.0e-9),
            7 => (6.0 * 6.0 + 1.0e-9, 7.0 * 7.0 - 1.0e-9),
            8 => (7.0 * 7.0 + 1.0e-9, 8.0 * 8.0 - 1.0e-9),
            9 => (8.0 * 8.0 + 1.0e-9, 9.0 * 9.0 - 1.0e-9),
            10 => (9.0 * 9.0 + 1.0e-9, 10.0 * 10.0 - 1.0e-9),
            11 => (10.0 * 10.0 + 1.0e-9, 11.0 * 11.0 - 1.0e-9),
            12..=14 => (-9.0, 31.0),
            15 | 16 => (0.0, 5.0),
            17 => (-0.95, 4.05),
            18 => (0.0, 1.5),
            19..=23 => (0.0, 1.0),
            24 => (1.0e-2, 1.0),
            25 => (1.0, 100.0),
            26 => (-1.0, 4.0),
            27 => (-10_000.0, FRAC_PI_2),
            28 => (-10_000.0, 1.0e-4),
            _ => panic!("unsupported NPROB: {}", nprob),
        };

        (a, b)
    }

    fn func(nprob: usize, x: f64, n: i32) -> f64 {
        let dn = n as f64;

        

        match nprob {
            1 => x.sin() - x / 2.0,
            2..=11 => {
                let mut value: f64 = 0.0;
                for i in 1..=20 {
                    let di = i as f64;
                    value += ((2.0 * di - 5.0).powi(2)) / (x - di * di).powi(3);
                }

                -2.0 * value
            }
            12 => -40.0 * x * (-x).exp(),
            13 => -100.0 * x * (-2.0 * x).exp(),
            14 => -200.0 * x * (-3.0 * x).exp(),
            15 => x.powi(n) - 0.2,
            16 | 17 => x.powi(n) - 1.0,
            18 => x.sin() - 0.5,
            19 => 2.0 * x * (-dn).exp() - 2.0 * (-dn * x).exp() + 1.0,
            20 => (1.0 + (1.0 - dn).powi(2)) * x - (1.0 - dn * x).powi(2),
            21 => x.powi(2) - (1.0 - x).powi(n),
            22 => (1.0 + (1.0 - dn).powi(4)) * x - (1.0 - dn * x).powi(4),
            23 => (x - 1.0) * (-dn * x).exp() + x.powi(n),
            24 => (dn * x - 1.0) / ((dn - 1.0) * x),
            25 => x.powf(1.0 / dn) - dn.powf(1.0 / dn),
            26 => {
                if x == 0.0 {
                    0.0
                } else {
                    x / (1.0 / (x * x)).exp()
                }
            }
            27 => {
                if x >= 0.0 {
                    (x / 1.5 + x.sin() - 1.0) * dn / 20.0
                } else {
                    -dn / 20.0
                }
            }
            28 => {
                if x >= 1.0e-3 * 2.0 / (dn + 1.0) {
                    1.0f64.exp() - 1.859
                } else if x >= 0.0 {
                    ((dn + 1.0) * 0.5 * x * 1.0e3).exp() - 1.859
                } else {
                    -0.859
                }
            }
            _ => panic!("unsupported NPROB: {}", nprob),
        }
    }

    #[allow(clippy::approx_constant)]
    fn original_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                nprob: 1,
                n: 1,
                expected_root: 1.8954942670340,
            },
            TestCase {
                nprob: 2,
                n: 1,
                expected_root: 3.0229153472731,
            },
            TestCase {
                nprob: 3,
                n: 1,
                expected_root: 6.6837535608081,
            },
            TestCase {
                nprob: 4,
                n: 1,
                expected_root: 11.238701655002,
            },
            TestCase {
                nprob: 5,
                n: 1,
                expected_root: 19.676000080623,
            },
            TestCase {
                nprob: 6,
                n: 1,
                expected_root: 29.828227326505,
            },
            TestCase {
                nprob: 7,
                n: 1,
                expected_root: 41.906116195289,
            },
            TestCase {
                nprob: 8,
                n: 1,
                expected_root: 55.953595800143,
            },
            TestCase {
                nprob: 9,
                n: 1,
                expected_root: 71.985665586588,
            },
            TestCase {
                nprob: 10,
                n: 1,
                expected_root: 90.008868539167,
            },
            TestCase {
                nprob: 11,
                n: 1,
                expected_root: 110.02653274833,
            },
            TestCase {
                nprob: 12,
                n: 1,
                expected_root: 0.0,
            },
            TestCase {
                nprob: 13,
                n: 1,
                expected_root: 0.0,
            },
            TestCase {
                nprob: 14,
                n: 1,
                expected_root: 0.0,
            },
            TestCase {
                nprob: 15,
                n: 4,
                expected_root: 0.66874030497642,
            },
            TestCase {
                nprob: 15,
                n: 6,
                expected_root: 0.76472449133173,
            },
            TestCase {
                nprob: 15,
                n: 8,
                expected_root: 0.81776543395794,
            },
            TestCase {
                nprob: 15,
                n: 10,
                expected_root: 0.85133992252078,
            },
            TestCase {
                nprob: 15,
                n: 12,
                expected_root: 0.87448527222117,
            },
            TestCase {
                nprob: 16,
                n: 4,
                expected_root: 1.0000000000000,
            },
            TestCase {
                nprob: 16,
                n: 6,
                expected_root: 1.0000000000000,
            },
            TestCase {
                nprob: 16,
                n: 8,
                expected_root: 1.0000000000000,
            },
            TestCase {
                nprob: 16,
                n: 10,
                expected_root: 1.0000000000000,
            },
            TestCase {
                nprob: 16,
                n: 12,
                expected_root: 1.0000000000000,
            },
            TestCase {
                nprob: 17,
                n: 8,
                expected_root: 1.0000000000000,
            },
            TestCase {
                nprob: 17,
                n: 10,
                expected_root: 1.0000000000000,
            },
            TestCase {
                nprob: 17,
                n: 12,
                expected_root: 1.0000000000000,
            },
            TestCase {
                nprob: 17,
                n: 14,
                expected_root: 1.0000000000000,
            },
            TestCase {
                nprob: 18,
                n: 1,
                expected_root: 0.52359877559830,
            },
            TestCase {
                nprob: 19,
                n: 1,
                expected_root: 0.42247770964124,
            },
            TestCase {
                nprob: 19,
                n: 2,
                expected_root: 0.30669941048320,
            },
            TestCase {
                nprob: 19,
                n: 3,
                expected_root: 0.22370545765466,
            },
            TestCase {
                nprob: 19,
                n: 4,
                expected_root: 0.17171914751951,
            },
            TestCase {
                nprob: 19,
                n: 5,
                expected_root: 0.13825715505682,
            },
            TestCase {
                nprob: 19,
                n: 20,
                expected_root: 3.4657359020854e-02,
            },
            TestCase {
                nprob: 19,
                n: 40,
                expected_root: 1.7328679513999e-02,
            },
            TestCase {
                nprob: 19,
                n: 60,
                expected_root: 1.1552453009332e-02,
            },
            TestCase {
                nprob: 19,
                n: 80,
                expected_root: 8.6643397569993e-03,
            },
            TestCase {
                nprob: 19,
                n: 100,
                expected_root: 6.9314718055995e-03,
            },
            TestCase {
                nprob: 20,
                n: 5,
                expected_root: 3.8402551840622e-02,
            },
            TestCase {
                nprob: 20,
                n: 10,
                expected_root: 9.9000099980005e-03,
            },
            TestCase {
                nprob: 20,
                n: 20,
                expected_root: 2.4937500390620e-03,
            },
            TestCase {
                nprob: 21,
                n: 2,
                expected_root: 0.50000000000000,
            },
            TestCase {
                nprob: 21,
                n: 5,
                expected_root: 0.34595481584824,
            },
            TestCase {
                nprob: 21,
                n: 10,
                expected_root: 0.24512233375331,
            },
            TestCase {
                nprob: 21,
                n: 15,
                expected_root: 0.19554762353657,
            },
            TestCase {
                nprob: 21,
                n: 20,
                expected_root: 0.16492095727644,
            },
            TestCase {
                nprob: 22,
                n: 1,
                expected_root: 0.27550804099948,
            },
            TestCase {
                nprob: 22,
                n: 2,
                expected_root: 0.13775402049974,
            },
            TestCase {
                nprob: 22,
                n: 4,
                expected_root: 1.0305283778156e-02,
            },
            TestCase {
                nprob: 22,
                n: 5,
                expected_root: 3.6171081789041e-03,
            },
            TestCase {
                nprob: 22,
                n: 8,
                expected_root: 4.1087291849640e-04,
            },
            TestCase {
                nprob: 22,
                n: 15,
                expected_root: 2.5989575892908e-05,
            },
            TestCase {
                nprob: 22,
                n: 20,
                expected_root: 7.6685951221853e-06,
            },
            TestCase {
                nprob: 23,
                n: 1,
                expected_root: 0.40105813754155,
            },
            TestCase {
                nprob: 23,
                n: 5,
                expected_root: 0.51615351875793,
            },
            TestCase {
                nprob: 23,
                n: 10,
                expected_root: 0.53952222690842,
            },
            TestCase {
                nprob: 23,
                n: 15,
                expected_root: 0.54818229434066,
            },
            TestCase {
                nprob: 23,
                n: 20,
                expected_root: 0.55270466667849,
            },
            TestCase {
                nprob: 24,
                n: 2,
                expected_root: 0.50000000000000,
            },
            TestCase {
                nprob: 24,
                n: 5,
                expected_root: 0.20000000000000,
            },
            TestCase {
                nprob: 24,
                n: 15,
                expected_root: 6.6666666666667e-02,
            },
            TestCase {
                nprob: 24,
                n: 20,
                expected_root: 5.0000000000000e-02,
            },
            TestCase {
                nprob: 25,
                n: 2,
                expected_root: 2.0000000000000,
            },
            TestCase {
                nprob: 25,
                n: 3,
                expected_root: 3.0000000000000,
            },
            TestCase {
                nprob: 25,
                n: 4,
                expected_root: 4.0000000000000,
            },
            TestCase {
                nprob: 25,
                n: 5,
                expected_root: 5.0000000000000,
            },
            TestCase {
                nprob: 25,
                n: 6,
                expected_root: 6.0000000000000,
            },
            TestCase {
                nprob: 25,
                n: 7,
                expected_root: 7.0000000000000,
            },
            TestCase {
                nprob: 25,
                n: 9,
                expected_root: 9.0000000000000,
            },
            TestCase {
                nprob: 25,
                n: 11,
                expected_root: 11.000000000000,
            },
            TestCase {
                nprob: 25,
                n: 13,
                expected_root: 13.000000000000,
            },
            TestCase {
                nprob: 25,
                n: 15,
                expected_root: 15.000000000000,
            },
            TestCase {
                nprob: 25,
                n: 17,
                expected_root: 17.000000000000,
            },
            TestCase {
                nprob: 25,
                n: 19,
                expected_root: 19.000000000000,
            },
            TestCase {
                nprob: 25,
                n: 21,
                expected_root: 21.000000000000,
            },
            TestCase {
                nprob: 25,
                n: 23,
                expected_root: 23.000000000000,
            },
            TestCase {
                nprob: 25,
                n: 25,
                expected_root: 25.000000000000,
            },
            TestCase {
                nprob: 25,
                n: 27,
                expected_root: 27.000000000000,
            },
            TestCase {
                nprob: 25,
                n: 29,
                expected_root: 29.000000000000,
            },
            TestCase {
                nprob: 25,
                n: 31,
                expected_root: 31.000000000000,
            },
            TestCase {
                nprob: 25,
                n: 33,
                expected_root: 33.000000000000,
            },
            TestCase {
                nprob: 26,
                n: 1,
                expected_root: 2.2317679157465e-02,
            },
            TestCase {
                nprob: 27,
                n: 1,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 2,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 3,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 4,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 5,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 6,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 7,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 8,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 9,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 10,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 11,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 12,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 13,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 14,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 15,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 16,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 17,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 18,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 19,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 20,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 21,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 22,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 23,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 24,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 25,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 26,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 27,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 28,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 29,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 30,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 31,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 32,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 33,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 34,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 35,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 36,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 37,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 38,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 39,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 27,
                n: 40,
                expected_root: 0.62380651896161,
            },
            TestCase {
                nprob: 28,
                n: 20,
                expected_root: 5.9051305594220e-05,
            },
            TestCase {
                nprob: 28,
                n: 21,
                expected_root: 5.6367155339937e-05,
            },
            TestCase {
                nprob: 28,
                n: 22,
                expected_root: 5.3916409455592e-05,
            },
            TestCase {
                nprob: 28,
                n: 23,
                expected_root: 5.1669892394942e-05,
            },
            TestCase {
                nprob: 28,
                n: 24,
                expected_root: 4.9603096699145e-05,
            },
            TestCase {
                nprob: 28,
                n: 25,
                expected_root: 4.7695285287639e-05,
            },
            TestCase {
                nprob: 28,
                n: 26,
                expected_root: 4.5928793239949e-05,
            },
            TestCase {
                nprob: 28,
                n: 27,
                expected_root: 4.4288479195665e-05,
            },
            TestCase {
                nprob: 28,
                n: 28,
                expected_root: 4.2761290257883e-05,
            },
            TestCase {
                nprob: 28,
                n: 29,
                expected_root: 4.1335913915954e-05,
            },
            TestCase {
                nprob: 28,
                n: 30,
                expected_root: 4.0002497338020e-05,
            },
            TestCase {
                nprob: 28,
                n: 31,
                expected_root: 3.8752419296207e-05,
            },
            TestCase {
                nprob: 28,
                n: 32,
                expected_root: 3.7578103559958e-05,
            },
            TestCase {
                nprob: 28,
                n: 33,
                expected_root: 3.6472865219959e-05,
            },
            TestCase {
                nprob: 28,
                n: 34,
                expected_root: 3.5430783356532e-05,
            },
            TestCase {
                nprob: 28,
                n: 35,
                expected_root: 3.4446594929961e-05,
            },
            TestCase {
                nprob: 28,
                n: 36,
                expected_root: 3.3515605877800e-05,
            },
            TestCase {
                nprob: 28,
                n: 37,
                expected_root: 3.2633616249437e-05,
            },
            TestCase {
                nprob: 28,
                n: 38,
                expected_root: 3.1796856858426e-05,
            },
            TestCase {
                nprob: 28,
                n: 39,
                expected_root: 3.1001935436965e-05,
            },
            TestCase {
                nprob: 28,
                n: 40,
                expected_root: 3.0245790670210e-05,
            },
            TestCase {
                nprob: 28,
                n: 100,
                expected_root: 1.2277994232462e-05,
            },
            TestCase {
                nprob: 28,
                n: 200,
                expected_root: 6.1695393904409e-06,
            },
            TestCase {
                nprob: 28,
                n: 300,
                expected_root: 4.1198585298293e-06,
            },
            TestCase {
                nprob: 28,
                n: 400,
                expected_root: 3.0924623877272e-06,
            },
            TestCase {
                nprob: 28,
                n: 500,
                expected_root: 2.4752044261050e-06,
            },
            TestCase {
                nprob: 28,
                n: 600,
                expected_root: 2.0633567678513e-06,
            },
            TestCase {
                nprob: 28,
                n: 700,
                expected_root: 1.7690120078154e-06,
            },
            TestCase {
                nprob: 28,
                n: 800,
                expected_root: 1.5481615698859e-06,
            },
            TestCase {
                nprob: 28,
                n: 900,
                expected_root: 1.3763345366022e-06,
            },
            TestCase {
                nprob: 28,
                n: 1000,
                expected_root: 1.2388385788997e-06,
            },
        ]
    }

    #[test]
    fn dataset_has_all_154_original_cases() {
        assert_eq!(original_cases().len(), 154);
    }

    #[test]
    fn reproduces_original_outputs() {
        for case in original_cases() {
            let (a, b) = init(case.nprob);
            let f = move |x| func(case.nprob, x, case.n);
            let root = enclose_zero(&f, a, b).unwrap();
            let err = (root - case.expected_root).abs();
            assert!(
                err <= 1e-13_f64.max(1e-13 * case.expected_root.abs()),
                "{case:?}: got {:.17e}, expected {:.17e}, err={err:e}",
                root,
                case.expected_root
            );
        }
    }

    #[test]
    fn exdrive_problem_1() {
        let root =
            enclose_zero(&|x| x * x - (1.0 - x).powi(5), 0.0, 1.0).unwrap();
        assert!((root - 0.34595481584824).abs() < 1e-14);
    }
    #[test]
    fn exdrive_problem_2() {
        let root = enclose_zero(
            &|x| x.powf(1.0 / 5.0) - 5.0_f64.powf(1.0 / 5.0),
            1.0,
            100.0,
        )
        .unwrap();
        assert!((root - 5.0).abs() < 1e-14);
    }
}
