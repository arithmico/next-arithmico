#![allow(dead_code)]

use std::f64::consts::{
    self, EULER_GAMMA, FRAC_1_PI, FRAC_2_SQRT_PI, FRAC_PI_2, FRAC_PI_4, TAU,
};

use crate::DistributionError;

/// Computes the regularized lower incomplete gamma function P(a, x).
pub fn calculate_regularized_lower_incomplete_gamma(
    a: f64,
    x: f64,
) -> Result<f64, DistributionError> {
    let output = cdf_gam_c(1, a, x);

    if output.ierr != 0 {
        return Err(DistributionError::GammaError);
    }

    Ok(output.p)
}

/// Computes the regularized upper incomplete gamma function Q(a, x).
pub fn calculate_regularized_upper_incomplete_gamma(
    a: f64,
    x: f64,
) -> Result<f64, DistributionError> {
    let output = cdf_gam_c(1, a, x);

    if output.ierr != 0 {
        return Err(DistributionError::GammaError);
    }

    Ok(output.q)
}

/*
    References:

    A. Gil, J. Segura, N. M. Temme.
    GammaCHI: A package for the inversion and computation of the gamma and
    chi-square cumulative distribution functions (central and noncentral).
    New version announcement.
    Computer Physics Communications, 267 (2021), 108083.
    <https://doi.org/10.1016/j.cpc.2021.108083>

    Corresponding code:
    A. Gil, J. Segura, N. M. Temme.
    GammaCHI: A package for the inversion and computation of the gamma and
    chi-square cumulative distribution functions (central and noncentral).
    New version announcement.
    License: GPL v2
    <https://doi.org/10.17632/d2kwwvsyny.1>

    This is a revised version of:

    A. Gil, J. Segura, N. M. Temme.
    GammaCHI: A package for the inversion and computation of the gamma and
    chi-square cumulative distribution functions (central and noncentral).
    Computer Physics Communications, 191 (2015), 132-139.
    <https://doi.org/10.1016/j.cpc.2015.01.004>

    The algorithms for the computation and inversion of the incomplete gamma
    function ratios are described in:

    A. Gil, J. Segura, N. M. Temme.
    Efficient and Accurate Algorithms for the Computation and Inversion of the
    Incomplete Gamma Function Ratios.
    SIAM Journal on Scientific Computing, 34(6) (2012), A2965-A2981.
    <https://doi.org/10.1137/120872553>

    The work builds on earlier algorithms for the incomplete gamma function,
    including:

    A. R. DiDonato, A. H. Morris, Jr.
    Computation of the Incomplete Gamma Function Ratios and Their Inverse.
    ACM Transactions on Mathematical Software, 12(4) (1986), 377-393.
    <https://doi.org/10.1145/22721.23109>

    The corresponding FORTRAN implementation was published as:

    A. R. DiDonato, A. H. Morris, Jr.
    Algorithm 654: FORTRAN Subroutines for Computing the Incomplete Gamma
    Function Ratios and Their Inverse.
    ACM Transactions on Mathematical Software, 13(3) (1987), 318-319.
    <https://doi.org/10.1145/29380.214348>
*/

// Isomorphic Rust port of gammaCHI.f90 (GammaCHI 2021).
// Original routine names, order, and comments are retained as far as Rust syntax permits.

const LN_SQRT_TWO_PI: f64 = 0.9189385332046727418; // log(sqrt(2*pi))
const SQRT_TWO_PI: f64 = 2.5066282746310005024; // sqrt(2*pi)
const EUL_MASC: f64 = EULER_GAMMA; // Euler-Mascheroni constant = 0.5772156649015328606
const PI_HALF: f64 = FRAC_PI_2; // pi/2 = 1.5707963267948966192
const PI_QUART: f64 = FRAC_PI_4; // pi/4 = 0.78539816339744830962
const PI_THREE_QUART: f64 = 2.3561944901923449288; // 3pi/4
const SQRT_PI: f64 = 1.7724538509055160272; // sqrt(pi)
const LN_PI: f64 = 1.1447298858494001741; // log(pi)
const PI: f64 = consts::PI; // pi = 3.1415926535897932385
const ONE_PI: f64 = FRAC_1_PI; // 1/pi = 0.31830988618379067153
const SQRT_2: f64 = consts::SQRT_2; // sqrt(2) = 1.4142135623730950488
const SQRT_3: f64 = 1.7320508075688772935; // sqrt(3)
const SQRT_2_PI: f64 = 0.7978845608028653559; // sqrt(2/pi)
const TWO_PI: f64 = TAU; // 2*pi = 6.2831853071795864769
const ONE_OVER_SQRT_PI: f64 = 0.5641895835477562869; // 1/sqrt(pi)
const TWO_OVER_SQRT_PI: f64 = FRAC_2_SQRT_PI; // 2/sqrt(pi) = 1.1283791670955125739
const ONE_THIRD: f64 = 0.33333333333333333; // 1/3
const TWO_THIRD: f64 = 0.6666666666666666666667; // 2/3
const ONE_SIX: f64 = 0.166666666666666666666666666667; // 1/6
const TWO_EXP_1_4: f64 = 1.18920711500272106671749997; // 2**(1/4)
const EPSS: f64 = 1.0e-15; // demanded accuracy 
const UNDFL: f64 = f64::MIN_POSITIVE; // underflow limit = TINY(0.0_r8)
const DWARF: f64 = f64::MIN_POSITIVE * 1000.0; // safe underflow limit = TINY(0.0_r8) * 1000.0
const GIANT: f64 = f64::MAX / 1000.0; // safe overflow limit = HUGE(0.0_r8) / 1000.0
const MACHTOL: f64 = f64::EPSILON; // machine-epsilon = EPSILON(0.0_r8)
const EXP_LOW: f64 = -300.0;
const EXP_HIGH: f64 = 300.0;

//---------------------------------------------------
// Authors:
//  Amparo Gil    (U. Cantabria, Santander, Spain)
//                 e-mail: amparo.gil@unican.es
//  Javier Segura (U. Cantabria, Santander, Spain)
//                 e-mail: javier.segura@unican.es
//  Nico M. Temme (CWI, Amsterdam, The Netherlands)
//                 e-mail: nico.temme@cwi.nl
// ------------------------------------------------------------------
//  (2021) New version of "GammaCHI: a package for the inversion
//         and computation of the gamma and chi-square distribution
//         functions (central and noncentral)".
//         Comput. Phys. Commun. 191(2015) 132-139
// -----------------------------------------------------------------
// FORTRAN: PUBLIC  :: cdfgamC, invcdfgamC, cdfgamNC, invcdfgamNC, errorfunction, inverfc, gamma, loggam, gamstar, quotgamm

struct CdfOutput {
    p: f64,
    q: f64,
    ierr: i32,
}

pub enum InverseCdfError {
    /// ierr = 1 in `invcdfgamNC`
    InvalidNoncentralityParameter,
    /// ierr = 2 in `invcdfgamNC`
    EvaluationFailed { estimate: f64 },
    /// ierr = 3 in `invcdfgamNC`
    NoConvergence { estimate: f64 },
    /// ierr = 4 in `invcdfgamNC`, ierr = 3 in `invcdfgamC`
    OutOfRange,
    /// ierr = 1 in `invcdfgamC`
    OverflowUnderflow { estimate: f64 },
    /// ierr = 2 in `invcdfgamC`
    LossOfAccuracy { estimate: f64 },
}

/// Calculation of the central gamma (or chi-square distribution)
/// functions P(a,x) and Q(a,x).
///
/// The parameters a and x are related to the input arguments
/// ai and xi as follows:
///
/// `a=ai, x=xi` for central gamma distributions.
///
/// `a=ai/2, x=xi/2` for central chi-square distributions.
/// The parameter ai corresponds to the degrees of freedom
/// of the distribution function.
///
/// The parameter xi should be zero or a positive real number.
/// The parameter ai should be a positive real number.
/// The admissible value should be greater than 1.e-300 (safe underflow)
/// for ai or smaller than 1e+300 (safe overflow) both for ai and xi.
///
/// The aimed relative accuracy is close to 1.0e-13 in
/// most cases.
///
/// # Inputs
///
/// - `ichi`: flag for the choice of gamma or
///   chi-square distribution function:
///   - ichi=1, computes the gamma distribution function.
///   - ichi=2, computes the central chi-square
///     distribution function.
/// - `ai`: argument of the functions
/// - `xi`: argument of the functions
///
/// # Outputs
///
/// - `p`: function P(a,x)
/// - `q`: function Q(a,x)
/// - `ierr`: error flag
///   - ierr=0, computation succesful
///   - ierr=1, underflow problems. The function values
///     (P(a,x) and Q(a,x)) are set to zero or one
///     depending on the parameter region. Relative
///     accuracy is not guaranteed for the smallest value.
///   - ierr=2, any of the arguments of the function is
///     out of range. The function values
///     (P(a,x) and Q(a,x)) are set to zero.
///
/// # Authors
///
/// Amparo Gil (U. Cantabria, Santander, Spain)
/// e-mail: amparo.gil@unican.es
///
/// Javier Segura (U. Cantabria, Santander, Spain)
/// e-mail: javier.segura@unican.es
///
/// Nico M. Temme (CWI, Amsterdam, The Netherlands)
/// e-mail: nico.temme@cwi.nl
///
/// # References
///
/// 1. A. Gil, J. Segura and N.M. Temme, GammaCHI: a package
///    for the inversion and computation of the gamma and
///    chi-square distribution functions (central and noncentral).
///    Computer Physics Commun. 191(2015) 132-139
/// 2. "Efficient and accurate algorithms for the computation
///    and inversion of the incomplete gamma function ratios",
///    A. Gil, J. Segura and N.M. Temme, SIAM J Sci Comput (2012)
fn cdf_gam_c(ichi: i32, ai: f64, xi: f64) -> CdfOutput {
    let asmall = DWARF;
    let alarg = GIANT;
    let xlarg = GIANT;
    let (a, x) = if ichi == 2 {
        // chi-square distribution variables
        (ai * 0.5, xi * 0.5)
    } else {
        (ai, xi)
    };

    if x < 0.0 {
        return CdfOutput {
            p: 0.0,
            q: 0.0,
            ierr: 2,
        };
    }
    if a < asmall {
        return CdfOutput {
            p: 0.0,
            q: 0.0,
            ierr: 2,
        };
    }
    if a > alarg {
        return CdfOutput {
            p: 0.0,
            q: 0.0,
            ierr: 2,
        };
    }
    if x > xlarg {
        return CdfOutput {
            p: 0.0,
            q: 0.0,
            ierr: 2,
        };
    }

    if x < UNDFL {
        if x == 0.0 {
            return CdfOutput {
                p: 0.0,
                q: 1.0,
                ierr: 0,
            };
        } else {
            #[allow(clippy::approx_constant)]
            let alf = -0.6931 / UNDFL.ln();

            if a > alf {
                return CdfOutput {
                    p: 0.0,
                    q: 1.0,
                    ierr: 1,
                };
            } else {
                return CdfOutput {
                    p: 1.0,
                    q: 0.0,
                    ierr: 1,
                };
            }
        }
    }

    let lnx = x.ln();
    if a > alfa(x) {
        let dp = dompart(a, x, false);
        if dp <= 0.0 {
            CdfOutput {
                p: 0.0,
                q: 1.0,
                ierr: 1,
            }
        } else {
            let p = if (x < 0.34 * a) || (a < 16.5) {
                ptaylor(a, x, dp)
            } else {
                pqasymp(a, x, dp, true)
            };
            let q = 1.0 - p;

            CdfOutput { p, q, ierr: 0 }
        }
    } else {
        if a < -DWARF / lnx {
            CdfOutput {
                p: 1.0,
                q: 0.0,
                ierr: 1,
            }
        } else {
            if x < 1.0 {
                let dp = dompart(a, x, true);
                if dp <= 0.0 {
                    CdfOutput {
                        p: 1.0,
                        q: 0.0,
                        ierr: 1,
                    }
                } else {
                    let q = qtaylor(a, x, dp);
                    let p = 1.0 - q;

                    CdfOutput { p, q, ierr: 0 }
                }
            } else {
                let dp = dompart(a, x, false);
                if dp <= 0.0 {
                    CdfOutput {
                        p: 1.0,
                        q: 0.0,
                        ierr: 1,
                    }
                } else {
                    let q = if (x > 1.5 * a) || (a < 12.0) {
                        qfraction(a, x, dp)
                    } else {
                        pqasymp(a, x, dp, false)
                    };
                    let p = 1.0 - q;

                    CdfOutput { p, q, ierr: 0 }
                }
            }
        }
    }
}

/// Inversion routine for the central gamma distribution.
///
/// This routine computes xr in the equations P(a,xr)=p
/// and Q(a,xr)=q with a as a given positive parameter;
/// p and q satisfy p+q=1.
/// We invert the equation with min(p,q).
///
/// The parameter a is related to the input argument ai as:
///
/// - a=ai, for central gamma distributions
/// - a=ai/2, for central chi-square distributions.
///   The parameter ai corresponds to the degrees
///   of freedom of the distribution function.
///
/// The parameter ai should be a positive real number. In order
/// to avoid underflow/overflow problems, the admissible value
/// of a should be greater than 1.e-250.
/// p (or q) should be greater than the underflow limit.
///
/// # Inputs
///
/// - `ichi`, flag for the choice of gamma or
///   chi-square distribution function:
///   - `ichi=1`, computes the inverse of the central
///     gamma distribution function.
///   - `ichi=2`, computes the inverse of the central
///     chi-square distribution function.
/// - `ai`, argument of the functions
/// - `p`, function value P(a,x)
/// - `q`, function value Q(a,x)
///
/// # Outputs
///
/// - `xr`, solution of the equations P(a,xr)=p and Q(a,xr)=q
///   with a as a given positive parameter.
/// - `ierr`, error flag
///   - `ierr=0`, computation succesful
///   - `ierr=1`, overflow/underflow problems in one or more steps of the
///     computation.
///     Either the value zero or the initial approximation
///     to the root is given as output.
///   - `ierr=2`, some loss of accuracy is expected in the calculations.
///     For example, the number of iterations in the Newton or
///     Schwarzian-Newton method reached the upper limit N=15.
///     The last value obtained for the root is given as output.
///   - `ierr=3`, any of the arguments of the function is
///     out of range. The value 0 is given as output.
///
/// The claimed accuracy obtained using this inversion routine is near
/// 1.e-12.
///
/// # Methods of computation
///
/// The present code uses different methods of computation
/// depending on the input values.
///
/// # Authors
///
/// - Amparo Gil (U. Cantabria, Santander, Spain)
///   - e-mail: amparo.gil@unican.es
/// - Javier Segura (U. Cantabria, Santander, Spain)
///   - e-mail: javier.segura@unican.es
/// - Nico M. Temme (CWI, Amsterdam, The Netherlands)
///   - e-mail: nico.temme@cwi.nl
///
/// # References
///
/// 1. A. Gil, J. Segura and N.M. Temme, GammaCHI: a package
///    for the inversion and computation of the gamma and
///    chi-square distribution functions (central and noncentral).
///    Computer Physics Commun 191(2015) 132-139
/// 2. A. Gil, J. Segura and N.M. Temme. Efficient and accurate
///    algorithms for the computation and inversion
///    of the incomplete gamma function ratios. SIAM J Sci Comput.
///    (2012) 34(6), A2965-A2981
fn inv_cdf_gam_c(
    ichi: i32,
    ai: f64,
    p: f64,
    q: f64,
) -> Result<f64, InverseCdfError> {
    let asmall = 1.0e-250;
    let scale = if ichi == 2 { 2.0 } else { 1.0 };

    let a = if ichi == 2 {
        // chi-square distribution variables
        ai * 0.5
    } else {
        ai
    };

    if a < asmall {
        return Err(InverseCdfError::OutOfRange);
    }
    if (p > 1.0) || (q > 1.0) {
        return Err(InverseCdfError::OutOfRange);
    }
    if (p <= UNDFL) || (q <= UNDFL) {
        return Err(InverseCdfError::OutOfRange);
    }
    if p + q > 1.0 + 1e-8 {
        return Err(InverseCdfError::OutOfRange);
    }

    let (pcase, porq, s) = if p < 0.5 {
        (true, p, -1.0)
    } else {
        (false, q, 1.0)
    };

    let mut k = 0;
    let mut m = 0;
    let mut x0 = 0.0;
    let mut lgama = 0.0;
    let mut invfp = 0.0;
    let mut ck = [0.0; 5]; // ck(1:5)

    if pcase {
        // P(a,x) is the primary function
        if a < 1.0e-3 {
            return Ok(0.0);
        } else {
            let logr = (1.0 / a) * (p.ln() + loggam(a + 1.0));
            if logr < (0.2 * (1.0 + a)).ln() {
                // p small
                let r = logr.exp();
                m = 0;
                let a2 = a * a;
                let a3 = a2 * a;
                let a4 = a3 * a;
                let ap1 = a + 1.0;
                let ap12 = ap1 * ap1;
                let ap13 = ap1 * ap12;
                let ap14 = ap12 * ap12;
                let ap2 = a + 2.0;
                let ap22 = ap2 * ap2;
                let ap3 = a + 3.0;
                ck = [
                    1.0,
                    1.0 / ap1,
                    0.5 * (3.0 * a + 5.0) / (ap12 * ap2),
                    0.333333333333333333333333333333
                        * (31.0 + 8.0 * a2 + 33.0 * a)
                        / (ap13 * ap2 * ap3),
                    0.0416666666666666666666666666667
                        * (2888.0
                            + 1179.0 * a3
                            + 125.0 * a4
                            + 3971.0 * a2
                            + 5661.0 * a)
                        / (ap14 * ap22 * ap3 * (a + 4.0)),
                ];
                x0 = r
                    * (1.0
                        + r * (ck[1] + r * (ck[2] + r * (ck[3] + r * ck[4]))));
                lgama = loggam(a);
                k = 1;
            }
        }
    } else {
        // Q(a,x) is the primary function
        if a < 10.0 {
            let vmin = 0.02_f64.min((-1.5 * a).exp() / gamma(a));
            let vdif = (1.0 - vmin / q).abs();
            if (q < vmin) && (vdif > 1.0e-30) {
                // Q small
                m = 0;
                let b = 1.0 - a;
                let b2 = b * b;
                let b3 = b2 * b;
                let sqarg =
                    -2.0 / a * (q * gamstar(a) * SQRT_TWO_PI / a.sqrt()).ln();
                if sqarg < 0.0 {
                    x0 = (-100.0_f64).exp();
                } else {
                    let eta = (-2.0 / a
                        * (q * gamstar(a) * SQRT_TWO_PI / a.sqrt()).ln())
                    .sqrt();
                    x0 = a * lambdaeta(eta);
                    let l = x0.ln();
                    if x0 > 5.0 {
                        let l2 = l * l;
                        let l3 = l2 * l;
                        let l4 = l3 * l;
                        let r = 1.0 / x0;
                        ck[0..4].copy_from_slice(&[
                            l - 1.0,
                            (3.0 * b - 2.0 * b * l + l2 - 2.0 * l + 2.0) * 0.5,
                            (24.0 * b * l - 11.0 * b2 - 24.0 * b - 6.0 * l2
                                + 12.0 * l
                                - 12.0
                                - 9.0 * b * l2
                                + 6.0 * b2 * l
                                + 2.0 * l3)
                                * 0.166666666666666666666666666667,
                            (-12.0 * b3 * l + 84.0 * b * l2 - 114.0 * b2 * l
                                + 72.0
                                + 36.0 * l2
                                + 3.0 * l4
                                - 72.0 * l
                                + 162.0 * b
                                - 168.0 * b * l
                                - 12.0 * l3
                                + 25.0 * b3
                                - 22.0 * b * l3
                                + 36.0 * b2 * l2
                                + 120.0 * b2)
                                * 0.0833333333333333333333333333333,
                        ]);
                        x0 = x0 - l
                            + b * r
                                * (ck[0]
                                    + r * (ck[1] + r * (ck[2] + r * ck[3])));
                    } else {
                        let r = 1.0 / x0;
                        ck[0] = l - 1.0;
                        if (l - b * r * ck[0]) < x0 {
                            x0 = x0 - l + b * r * ck[0];
                        }
                    }
                    lgama = loggam(a);
                    k = 1;
                }
            }
        }
    }

    if ((porq - 0.5).abs() < 1.0e-5) && (k == 0) {
        if a > 0.4 {
            m = 0;
            let ainv = 1.0 / a;
            x0 = a - 0.333333333333333333333333333333
                + (0.0197530864197530864197530864198
                    + 0.00721144424848128551832255535959 * ainv)
                    * ainv;
            lgama = loggam(a);
        } else {
            x0 = (-100.0_f64).exp();
        }
        k = 1;
    }

    if ((a - 1.0).abs() < 1.0e-4) && (k == 0) {
        m = 0;
        if pcase {
            x0 = -(1.0 - p).ln();
        } else {
            x0 = -q.ln();
        }
        lgama = loggam(a);
        k = 1;
    }

    if (a < 1.0) && (k == 0) {
        m = 0;
        if pcase {
            x0 = ((1.0 / a) * (porq.ln() + loggam(a + 1.0))).exp();
        } else {
            x0 = ((1.0 / a) * ((1.0 - porq).ln() + loggam(a + 1.0))).exp();
        }
        lgama = loggam(a);
        k = 1;
        if (x0 < DWARF) && (a < 0.5) {
            x0 = (-100.0_f64).exp();
        }
    }

    if k == 0 {
        m = 1;
        let ainv = 1.0 / a;
        let r = inverfc(2.0 * porq);
        let mut eta = s * r / (a * 0.5).sqrt();
        if r < GIANT {
            // a large. Inversion using the error function
            eta = eta
                + (eps1(eta) + (eps2(eta) + eps3(eta) * ainv) * ainv) * ainv;
            if eta.abs() < 1e-4 {
                x0 = a * (1.0 + eta);
                if eta.abs() < 1.0e-8 {
                    return Err(InverseCdfError::LossOfAccuracy {
                        estimate: x0 * scale,
                    });
                }
            } else {
                x0 = a * lambdaeta(eta);
            }
            let y = eta;
            let fp =
                -(a / TWO_PI).sqrt() * (-0.5 * a * y * y).exp() / (gamstar(a));
            invfp = 1.0 / fp;
        } else {
            x0 = 0.0;
            return Err(InverseCdfError::OverflowUnderflow {
                estimate: x0 * scale,
            });
        }
    }

    if k < 2 {
        if ((a < 0.5) && (p > 1.0e-8)) && (q > 1.0e-8) {
            let logd = DWARF.ln();
            let mut km = 0;
            let mut xnew = x0.ln();
            let mut er = 1.0;
            let mut n = 1;
            while ((er > 1.0e-15) && (km == 0)) && (n < 15) {
                let x = xnew;
                let z = x.exp();
                let cdf_result = cdf_gam_c(1, a, z);
                let px = cdf_result.p;
                let qx = cdf_result.q;
                let _ierrf = cdf_result.ierr;
                let w = -0.25 * z * z + 0.5 * z * a - 0.25 * a * a - 0.5 * z;
                let bb = z - a;
                let pf1 = z.powf(a) * (-z).exp() / gamma(a);
                let ffd = if pcase {
                    (px - p) / (bb * 0.5 * (px - p) + pf1)
                } else {
                    (q - qx) / (bb * 0.5 * (q - qx) + pf1)
                };
                let argu = (-w).sqrt() * ffd;
                if 1.0 - argu < 0.0 {
                    xnew = logd - 1.0;
                } else {
                    let g = x - 1.0 / (-w).sqrt()
                        * 0.5
                        * ((1.0 + argu) / (1.0 - argu)).ln();
                    xnew = g;
                    er = (1.0 - x / xnew).abs();
                    n += 1;
                }
                if xnew < logd {
                    km = 1;
                }
            }
            if km == 1 {
                return Err(InverseCdfError::OverflowUnderflow {
                    estimate: 0.0,
                });
            } else {
                let xr = xnew.exp() * scale;
                if n == 15 {
                    return Err(InverseCdfError::LossOfAccuracy {
                        estimate: xr,
                    });
                }
                return Ok(xr);
            }
        } else {
            let mut x = x0;
            let mut t = 1.0;
            let mut n = 1;
            let a2 = a * a;

            // Implementation of a high order Newton-like method
            while (t > 1.0e-15) && (n < 15) {
                x = x0;
                let x2 = x * x;
                let mut r;
                if m == 0 {
                    let dlnr = (1.0 - a) * x.ln() + x + lgama;
                    if dlnr > GIANT.ln() {
                        return Err(InverseCdfError::OverflowUnderflow {
                            estimate: x * scale,
                        });
                    } else {
                        r = dlnr.exp();
                        let px;
                        let qx;
                        if pcase {
                            let cdf_result = cdf_gam_c(1, a, x);
                            px = cdf_result.p;
                            ck[0] = -r * (px - p);
                        } else {
                            let cdf_result = cdf_gam_c(1, a, x);
                            qx = cdf_result.q;
                            ck[0] = r * (qx - q);
                        }
                        r = ck[0];
                        if x0 > 0.1 {
                            ck[1..3].copy_from_slice(&[
                                0.5 * (x - a + 1.0) / x,
                                0.166666666666666666666666666667
                                    * (2.0 * x2 - 4.0 * x * a
                                        + 4.0 * x
                                        + 2.0 * a2
                                        - 3.0 * a
                                        + 1.0)
                                    / x2,
                            ]);
                            x0 = x + r * (1.0 + r * (ck[1] + r * ck[2]));
                        } else {
                            x0 = x + r;
                        }
                    }
                } else {
                    r = -invfp * x;
                    if pcase {
                        let cdf_result = cdf_gam_c(1, a, x);
                        let px = cdf_result.p;
                        let _qx = cdf_result.q;
                        let _ierrf = cdf_result.ierr;
                        ck[0] = -r * (px - p);
                    } else {
                        let cdf_result = cdf_gam_c(1, a, x);
                        let _px = cdf_result.p;
                        let qx = cdf_result.q;
                        let _ierrf = cdf_result.ierr;
                        ck[0] = r * (qx - q);
                    }
                    r = ck[0];
                    if x0 > 0.1 {
                        ck[1..3].copy_from_slice(&[
                            0.5 * (x - a + 1.0) / x,
                            0.166666666666666666666666666667
                                * (2.0 * x2 - 4.0 * x * a + 4.0 * x + 2.0 * a2
                                    - 3.0 * a
                                    + 1.0)
                                / x2,
                        ]);
                        x0 = x + r * (1.0 + r * (ck[1] + r * ck[2]));
                    } else {
                        x0 = x + r;
                    }
                }
                t = (x / x0 - 1.0).abs();
                n += 1;
                x = x0;
            }

            let xr = x * scale;
            if n == 15 {
                return Err(InverseCdfError::LossOfAccuracy { estimate: xr });
            }
            return Ok(xr);
        }
    }

    Ok(x0 * scale)
}

/// Calculation of the noncentral gamma or noncentral
/// chi-square distribution functions P_mu(x,y)
/// and Q_mu(x,y).
///
/// The parameters mu, x and y are related to the input
/// arguments mui, xi and yi as follows:
///
/// - mu=mui, x=xi, y=yi for noncentral gamma distributions.
/// - mu=mui/2, x=xi/2, y=yi/2 for noncentral chi-square
///   distributions. The parameter mui corresponds to the degrees
///   of freedom; xi is the noncentrality parameter; yi is the limit
///   of integration.
///
/// In order to avoid, overflow/underflow problems in IEEE double
/// precision arithmetic, the admissible parameter ranges
/// for computation are:
///
/// - 0<=x<=10000, 0<=y<=10000, 0.5<=mu<=10000
///
/// The aimed relative accuracy is close to 1.0e-11 in the
/// previous parameter domain.
///
/// # Inputs
///
/// - `ichi`: flag for the choice of gamma or
///   chi-square distribution function:
///   - `ichi=1`, computes the noncentral gamma
///     distribution function.
///   - `ichi=2`, computes the noncentral chi-square
///     distribution function.
/// - `mui`: argument of the functions
/// - `xi`: argument of the functions
/// - `yi`: argument of the functions
///
/// # Outputs
///
/// - `p`, function P_mu(x,y)
/// - `q`, function Q_mu(x,y)
/// - `ierr`, error flag
///   - `ierr=0`, computation succesful
///   - `ierr=1`, Underflow problems. The function values
///     are set to zero and one.
///   - `ierr=2`, any of the arguments of the function is
///     out of range. The function values
///     (P_mu(x,y) and Q_mu(x,y)) are set to zero.
///
/// # Methods of computation
///
/// The present code uses different methods of computation
/// depending on the values of mu, x and y: series expansions in terms
/// of incomplete gamma functions, integral representations,
/// asymptotic expansions, and use of three-term homogeneous
/// recurrence relations.
///
/// # Authors
///
/// - Amparo Gil (U. Cantabria, Santander, Spain)
///   - e-mail: amparo.gil@unican.es
/// - Javier Segura (U. Cantabria, Santander, Spain)
///   - e-mail: javier.segura@unican.es
/// - Nico M. Temme (CWI, Amsterdam, The Netherlands)
///   - e-mail: nico.temme@cwi.nl
///
/// # References
///
/// 1. A. Gil, J. Segura and N.M. Temme, GammaCHI: a package
///    for the inversion and computation of the gamma and
///    chi-square distribution functions (central and noncentral).
///    Computer Physics Commun 191(2015) 132-139
/// 2. A. Gil, J. Segura and N.M. Temme, Computation of the Marcum
///    Q-function. ACM Trans Math Soft (2014)
/// 3. A. Gil, J. Segura and N.M. Temme. Efficient and accurate
///    algorithms for the computation and inversion
///    of the incomplete gamma function ratios. SIAM J Sci Comput.
///    (2012) 34(6), A2965-A2981
fn cdf_gam_nc(ichi: i32, mui: f64, xi: f64, yi: f64) -> CdfOutput {
    let (mu, x, y) = if ichi == 2 {
        // chi-square distribution variables
        (mui * 0.5, xi * 0.5, yi * 0.5)
    } else {
        (mui, xi, yi)
    };

    if ((x > 10000.0) || (y > 10000.0)) || (mu > 10000.0) {
        return CdfOutput {
            p: 0.0,
            q: 0.0,
            ierr: 2,
        };
    }
    if ((x < 0.0) || (y < 0.0)) || (mu < 0.5) {
        return CdfOutput {
            p: 0.0,
            q: 0.0,
            ierr: 2,
        };
    }

    let mulim = 135.0;
    let b = 1.0;
    let w = b * (4.0 * x + 2.0 * mu).sqrt();
    let xii = 2.0 * (x * y).sqrt();
    let y0 = x + mu - w;
    let y1 = x + mu + w;

    let CdfOutput { p, q, ierr } = if (y > x + mu) && (x < 30.0) {
        // Series for Q in terms of ratios of Gamma functions
        qser(mu, x, y)
    } else if (y <= x + mu) && (x < 30.0) {
        // Series for P in terms of ratios of Gamma functions
        pser(mu, x, y)
    } else if (mu * mu < 2.0 * xii) && (xii > 30.0) {
        // Asymptotic expansion for xy large
        pqasyxy(mu, x, y)
    } else if ((mu >= mulim) && (y0 <= y)) && (y <= y1) {
        // Asymptotic expansion for mu large
        pqasymu(mu, x, y)
    } else if ((y <= y1) && (y > x + mu)) && (mu < mulim) {
        // Recurrence relation for Q
        qrec(mu, x, y)
    } else if ((y >= y0) && (y <= x + mu)) && (mu < mulim) {
        // Recurrence relation for P
        prec(mu, x, y)
    } else if mu <= 1.0 {
        // One step of the Recurrence Relation
        rec1step(mu, x, y)
    } else {
        // Integral representation
        cdf_pq_trap(mu, x, y)
    };

    if ierr == 0 {
        if p < 1.0e-290 {
            return CdfOutput {
                p: 0.0,
                q: 1.0,
                ierr: 1,
            };
        }
        if q < 1.0e-290 {
            return CdfOutput {
                p: 1.0,
                q: 0.0,
                ierr: 1,
            };
        }
    }

    CdfOutput { p, q, ierr }
}

/// Inversion routine for the noncentral gamma/chi-square
/// cumulative distribution function.
///
/// This routine computes, depending on the value of
/// icho:
///
/// - a) x in the equations Q_mu(x,y)=q, P_mu(x,y)=p
/// - b) y in the equations P_mu(x,y)=p, Q_mu(x,y)=q
///   with (mu,y,p,q) or (mu,x,p,q) as given parameters,
///   respectively.
///
/// In a) and b), we invert the equation with min(p,q).
///
/// The parameters mu and x (or y) are related to the input
/// arguments mui and yxi as follows:
///
/// mu=mui, x=yxi (or y=yxi) for noncentral gamma cumulative
/// distributions
///
/// mu=mui/2, x=yxi/2 (or y=yxi/2) for noncentral chi-square
/// cumulative distributions. The parameter mui corresponds
/// to the degrees of freedom; yxi is either the noncentrality
/// parameter or the limit of integration.
///
/// In statistics, the inversion of Q_mu(x,y)=q with respect
/// to x corresponds to the problem of inverting the noncentral
/// gamma (or chi-square) distribution function with respect
/// to the non-centrality parameter given the upper tail
/// probability.
///
/// On the other hand, the inversion of P_mu(x,y)=p with respect
/// to y with fixed x corresponds to the problem of computing the
/// p-quantiles of the distribution function.
///
/// The admissible parameter ranges for computation are:
///
/// - a) 1.e-20<p<1, 1.e-20<q<1, p+q=1
/// - b) Inversion with respect to x:
///   0<=y<=10000, 0.5<=mu<=10000,
/// - c) Inversion with respect to y:
///   0<=x<=10000, 0.5<=mu<=10000.
///
/// # Inputs
///
/// - `ichi`: flag for the choice of gamma or
///   chi-square distribution function:
///   - ichi=1, computes the inverse of the noncentral
///     gamma distribution function.
///   - ichi=2, computes the inverse of the noncentral
///     chi-square distribution function.
/// - `icho`: choice of the inversion process.
///   - If icho=1, x is computed in the
///     equations Q_mu(x,y)=q, P_mu(x,y)=p.
///   - If icho=2, y is computed in the
///     equations Q_mu(x,y)=q, P_mu(x,y)=p.
/// - `mui`: parameter of the function
/// - `p`: function value P_mu(x,y)
/// - `q`: function value Q_mu(x,y)
/// - `yxi`:
///   - If icho=1, yxi is y in the
///     equations P_mu(x,y)=p, Q_mu(x,y)=q.
///   - If icho=2, yxi is x in the
///     equation P_mu(x,y)=p, Q_mu(x,y)=q.
///
/// # Outputs
///
/// - `xy`:
///   - If icho=1, xy is x in the
///     equation P_mu(x,y)=p, Q_mu(x,y)=q.
///   - If icho=2, xy is y in the
///     equation P_mu(x,y)=p, Q_mu(x,y)=q.
/// - `ierr`: error flag
///   - ierr=0, computation succesful.
///   - ierr=1, in the inversion process with respect to the
///     non-centrality parameter (x-variable), the input
///     values (yx,qp) do not make sense (i.e. the
///     inequality qp<Q_mu(0,yx) holds). The value 0
///     is given as output.
///   - ierr=2, at least one of the gamma distribution
///     function values needed in the inversion process
///     cannot be correctly computed. Either 0 or the
///     last value obtained for the root are given
///     as output.
///   - ierr=3, the number of iterations in the secant method
///     reached the upper limit N=25. The last value
///     obtained for the root is given as output.
///   - ierr=4, any of the arguments of the function is
///     out of range. The value 0 is given as output.
///
/// The claimed accuracy obtained using this inversion routine is close
/// to the value of eps.
///
/// # Methods of Computations
///
/// The present code uses different methods of computation
/// depending on the values of the input values: asymptotic expansions
/// and/or secant method.
///
/// # Authors
///
/// Amparo Gil (U. Cantabria, Santander, Spain)
/// e-mail: amparo.gil@unican.es
///
/// Javier Segura (U. Cantabria, Santander, Spain)
/// e-mail: javier.segura@unican.es
///
/// Nico M. Temme (CWI, Amsterdam, The Netherlands)
/// e-mail: nico.temme@cwi.nl
///
/// # References
///
/// 1. A. Gil, J. Segura and N.M. Temme, GammaCHI: a package
///    for the inversion and computation of the gamma and
///    chi-square distribution functions (central and noncentral).
///    Computer Physics Commun 191(2015) 132-139
/// 2. A. Gil, J. Segura and N.M. Temme, The asymptotic and numerical
///    inversion of the Marcum Q-function. Studies in Applied
///    Mathematics (2014)
/// 3. A. Gil, J. Segura and N.M. Temme, Computation of the Marcum
///    Q-function. ACM Trans Math Soft (2014)
/// 4. A. Gil, J. Segura and N.M. Temme. Efficient and accurate
///    algorithms for the computation and inversion
///    of the incomplete gamma function ratios. SIAM J Sci Comput.
///    (2012) 34(6), A2965-A2981
fn inv_cdf_gam_nc(
    ichi: i32,
    icho: i32,
    mui: f64,
    p: f64,
    q: f64,
    yxi: f64,
) -> Result<f64, InverseCdfError> {
    let eps = 1.0e-12;
    if (p < 1.0e-20) || (q < 1.0e-20) {
        return Err(InverseCdfError::OutOfRange);
    }

    let (mu, yx) = if ichi == 2 {
        // chi-square distribution variables
        (mui * 0.5, yxi * 0.5)
    } else {
        (mui, yxi)
    };
    if !(0.0..=10000.0).contains(&yx) {
        return Err(InverseCdfError::OutOfRange);
    }
    if !(0.5..=10000.0).contains(&mu) {
        return Err(InverseCdfError::OutOfRange);
    }

    if icho == 1 {
        if q > 1.0 + 1.0e-30 {
            return Ok(if ichi == 2 { GIANT * 2.0 } else { GIANT });
        }

        let (pcase, porq, s) = if p < 0.5 {
            (true, p, -1.0)
        } else {
            (false, q, 1.0)
        };

        let qi = q;

        // Inversion with respect to the x variable in the
        // equation Q_mu(x,y)=q, fixed mu, y
        // Calculation of the noncentrality parameter.
        let y0 = yx;

        // We have first to check that qi >=q_mu(0,y)
        let cdf_result = cdf_gam_c(1, mu, y0);
        let _p0 = cdf_result.p;
        let q0 = cdf_result.q;
        let ierr3 = cdf_result.ierr;

        if ierr3 != 0 {
            return Err(InverseCdfError::EvaluationFailed { estimate: 0.0 });
        }

        if qi < q0 {
            return Err(InverseCdfError::InvalidNoncentralityParameter);
        }

        if mu < 5.0 {
            let mut zeta0 = s * inverfc(2.0 * porq) * 2.0_f64.sqrt();
            let mut x = xetay(zeta0, mu, yx);

            if (x * yx).sqrt() > 10.0 {
                let mu2 = mu * mu;
                let mu3 = mu2 * mu;
                zeta0 /= mu.sqrt();
                let y = yx / mu;
                x /= mu;

                let (zeta1, zeta2, zeta3) = zeta3xy(icho, zeta0, x, y);

                zeta0 = zeta0 + zeta1 / mu + zeta2 / mu2 + zeta3 / mu3;
                x = mu * xzetay(zeta0, y);
            }

            let (mut x1, mut x2) = if x > 0.0 {
                (x, x + 0.01)
            } else {
                (1.0e-6, 1.0e-1)
            };

            let CdfOutput {
                p: mut p1,
                q: mut q1,
                ierr: mut ierr1,
            } = cdf_gam_nc(1, mu, x1, y0);

            let CdfOutput {
                p: mut p2,
                q: mut q2,
                ierr: ierr2,
            } = cdf_gam_nc(1, mu, x2, y0);

            if (ierr1 != 0) || (ierr2 != 0) {
                return Err(InverseCdfError::EvaluationFailed {
                    estimate: 0.0,
                });
            }

            let mut delta = 1.0;
            let mut icon = 0;
            let mut xn = 0.0;
            while (delta > eps) && (icon < 25) {
                icon += 1;

                xn = if pcase {
                    x2 - (x2 - x1) * (p2 - p) / (p2 - p1)
                } else {
                    x2 - (x2 - x1) * (q2 - q) / (q2 - q1)
                };

                if xn < 0.0 {
                    xn = 1.0e-10;
                }

                x1 = x2;
                q1 = q2;
                p1 = p2;
                x2 = xn;

                let result = cdf_gam_nc(1, mu, x2, y0);
                p2 = result.p;
                q2 = result.q;
                ierr1 = result.ierr;

                if ierr1 == 0 {
                    if pcase {
                        delta = (1.0 - p2 / p).abs();

                        if p1 - p2 == 0.0 {
                            delta = 0.0;
                        }
                    } else {
                        delta = (1.0 - q2 / q).abs();

                        if q1 - q2 == 0.0 {
                            delta = 0.0;
                        }
                    }
                } else {
                    if icon == 25 {
                        return Err(InverseCdfError::NoConvergence {
                            estimate: if ichi == 2 { xn * 2.0 } else { xn },
                        });
                    }

                    return Err(InverseCdfError::EvaluationFailed {
                        estimate: if ichi == 2 { xn * 2.0 } else { xn },
                    });
                }
            }

            if icon == 25 {
                return Err(InverseCdfError::NoConvergence {
                    estimate: if ichi == 2 { xn * 2.0 } else { xn },
                });
            }

            Ok(if ichi == 2 { xn * 2.0 } else { xn })
        } else {
            // Asymptotics
            let mu2 = mu * mu;
            let mu3 = mu2 * mu;
            let mut zeta0 = s * inverfc(2.0 * porq) * (2.0 / mu).sqrt();
            let y = y0 / mu;
            let x = xzetay(zeta0, y);

            let (zeta1, zeta2, zeta3) = zeta3xy(icho, zeta0, x, y);

            zeta0 = zeta0 + zeta1 / mu + zeta2 / mu2 + zeta3 / mu3;
            let x = mu * xzetay(zeta0, y);
            let mut xn = x;

            let (mut x1, mut x2) = if x > 0.0 {
                (x, x + 0.01)
            } else {
                (1.0e-6, 1.0e-1)
            };

            let CdfOutput {
                p: mut p1,
                q: mut q1,
                ierr: mut ierr1,
            } = cdf_gam_nc(1, mu, x1, y0);

            let CdfOutput {
                p: mut p2,
                q: mut q2,
                ierr: ierr2,
            } = cdf_gam_nc(1, mu, x2, y0);

            if (ierr1 != 0) || (ierr2 != 0) {
                return Err(InverseCdfError::EvaluationFailed {
                    estimate: if ichi == 2 { xn * 2.0 } else { xn },
                });
            }

            let mut delta = 1.0;
            let mut icon = 0;
            while (delta > eps) && (icon < 25) {
                icon += 1;

                xn = if pcase {
                    x2 - (x2 - x1) * (p2 - p) / (p2 - p1)
                } else {
                    x2 - (x2 - x1) * (q2 - q) / (q2 - q1)
                };

                if xn < 0.0 {
                    xn = 1.0e-10;
                }

                x1 = x2;
                q1 = q2;
                p1 = p2;
                x2 = xn;

                let result = cdf_gam_nc(1, mu, x2, y0);
                p2 = result.p;
                q2 = result.q;
                ierr1 = result.ierr;

                if ierr1 == 0 {
                    if pcase {
                        delta = (1.0 - p2 / p).abs();

                        if p1 - p2 == 0.0 {
                            delta = 0.0;
                        }
                    } else {
                        delta = (1.0 - q2 / qi).abs();

                        if q1 - q2 == 0.0 {
                            delta = 0.0;
                        }
                    }
                } else {
                    if icon == 25 {
                        return Err(InverseCdfError::NoConvergence {
                            estimate: if ichi == 2 { xn * 2.0 } else { xn },
                        });
                    }

                    return Err(InverseCdfError::EvaluationFailed {
                        estimate: if ichi == 2 { xn * 2.0 } else { xn },
                    });
                }
            }

            if icon == 25 {
                return Err(InverseCdfError::NoConvergence {
                    estimate: if ichi == 2 { xn * 2.0 } else { xn },
                });
            }

            Ok(if ichi == 2 { xn * 2.0 } else { xn })
        }
    } else {
        // Inversion with respect to the y variable in
        // P_mu(x,y)=p, fixed mu and x
        // Calculation of the p-quantiles of the CDF
        if q.abs() < DWARF {
            return Ok(if ichi == 2 { GIANT * 2.0 } else { GIANT });
        }

        let (pcase, porq, s) = if p < 0.5 {
            (true, p, -1.0)
        } else {
            (false, q, 1.0)
        };

        let x0 = yx;
        if mu < 5.0 {
            let mut zeta0 = s * inverfc(2.0 * porq) * 2.0_f64.sqrt();
            let mut y = yetax(zeta0, mu, yx);

            if (x0 * y).sqrt() > 10.0 {
                let mu2 = mu * mu;
                zeta0 /= mu.sqrt();
                y /= mu;
                let x = yx / mu;

                let (zeta1, zeta2, _zeta3) = zeta3xy(icho, zeta0, x, y);

                zeta0 = zeta0 + zeta1 / mu + zeta2 / mu2;
                y = mu * yzetax(zeta0, x);
            }

            let (mut y1, mut y2) = if y > 0.0 {
                (y, y + 0.01)
            } else {
                (1.0e-6, 1.0e-1)
            };

            let CdfOutput {
                p: mut p1,
                q: mut q1,
                ierr: mut ierr1,
            } = cdf_gam_nc(1, mu, x0, y1);

            let CdfOutput {
                p: mut p2,
                q: mut q2,
                ierr: ierr2,
            } = cdf_gam_nc(1, mu, x0, y2);

            if (ierr1 != 0) || (ierr2 != 0) {
                return Err(InverseCdfError::EvaluationFailed {
                    estimate: 0.0,
                });
            }

            let mut delta = 1.0;
            let mut icon = 0;
            let mut yn = 0.0;

            while (delta > eps) && (icon < 25) {
                icon += 1;

                yn = if pcase {
                    y2 - (y2 - y1) / (p2 - p1) * (p2 - p)
                } else {
                    y2 - (y2 - y1) / (q2 - q1) * (q2 - q)
                };

                if yn < 0.0 {
                    yn = 1.0e-10;
                }

                y1 = y2;
                p1 = p2;
                q1 = q2;
                y2 = yn;

                let result = cdf_gam_nc(1, mu, x0, y2);
                p2 = result.p;
                q2 = result.q;
                ierr1 = result.ierr;

                if ierr1 == 0 {
                    if pcase {
                        delta = (1.0 - p2 / p).abs();

                        if p1 - p2 == 0.0 {
                            delta = 0.0;
                        }
                    } else {
                        delta = (1.0 - q2 / q).abs();

                        if q1 - q2 == 0.0 {
                            delta = 0.0;
                        }
                    }
                } else {
                    if icon == 25 {
                        return Err(InverseCdfError::NoConvergence {
                            estimate: if ichi == 2 { yn * 2.0 } else { yn },
                        });
                    }

                    return Err(InverseCdfError::EvaluationFailed {
                        estimate: if ichi == 2 { yn * 2.0 } else { yn },
                    });
                }
            }

            if icon == 25 {
                return Err(InverseCdfError::NoConvergence {
                    estimate: if ichi == 2 { yn * 2.0 } else { yn },
                });
            }

            Ok(if ichi == 2 { yn * 2.0 } else { yn })
        } else {
            let x = yx;
            let xs = x / mu;
            let mut zeta0 = s * inverfc(2.0 * porq) * (2.0 / mu).sqrt();
            let mut y = yzetax(zeta0, xs);

            let (zeta1, zeta2, _zeta3) = zeta3xy(icho, zeta0, xs, y);

            zeta0 = zeta0 + zeta1 / mu + zeta2 / (mu * mu);
            y = mu * yzetax(zeta0, xs);
            let mut yn = y;

            let (mut y1, mut y2) = if y > 0.0 {
                (y, y + 0.01)
            } else {
                (1.0e-6, 1.0e-1)
            };

            let CdfOutput {
                p: mut p1,
                q: mut q1,
                ierr: mut ierr1,
            } = cdf_gam_nc(1, mu, x, y1);

            let CdfOutput {
                p: mut p2,
                q: mut q2,
                ierr: ierr2,
            } = cdf_gam_nc(1, mu, x, y2);

            if (ierr1 != 0) || (ierr2 != 0) {
                return Err(InverseCdfError::EvaluationFailed {
                    estimate: 0.0,
                });
            }

            let mut delta = 1.0;
            let mut icon = 0;

            while (delta > eps) && (icon < 25) {
                icon += 1;

                yn = if pcase {
                    y2 - (y2 - y1) / (p2 - p1) * (p2 - p)
                } else {
                    y2 - (y2 - y1) / (q2 - q1) * (q2 - q)
                };
                if yn < 0.0 {
                    yn = 1.0e-10;
                }

                y1 = y2;
                p1 = p2;
                q1 = q2;
                y2 = yn;

                let result = cdf_gam_nc(1, mu, x, y2);
                p2 = result.p;
                q2 = result.q;
                ierr1 = result.ierr;

                if ierr1 == 0 {
                    if pcase {
                        delta = (1.0 - p2 / p).abs();

                        if p1 - p2 == 0.0 {
                            delta = 0.0;
                        }
                    } else {
                        delta = (1.0 - q2 / q).abs();

                        if q1 - q2 == 0.0 {
                            delta = 0.0;
                        }
                    }
                } else {
                    if icon == 25 {
                        return Err(InverseCdfError::NoConvergence {
                            estimate: if ichi == 2 { yn * 2.0 } else { yn },
                        });
                    }

                    return Err(InverseCdfError::EvaluationFailed {
                        estimate: if ichi == 2 { yn * 2.0 } else { yn },
                    });
                }
            }

            if icon == 25 {
                return Err(InverseCdfError::NoConvergence {
                    estimate: if ichi == 2 { yn * 2.0 } else { yn },
                });
            }

            Ok(if ichi == 2 { yn * 2.0 } else { yn })
        }
    }
}

/// Computation of: the error function, the complementary error
///                 function or the scaled complementary error
///                 function.
///
/// # Inputs
///
/// - `x`: real number.
/// - `erfcc`: logical variable.
/// - `expo`: logical variable.
///
/// The meaning of the logical variables `erfcc` and `expo`
/// is the following:
///
/// - When `erfcc = true` and `expo = false`, the function computes
///   the complementary error function `erfc(x)`.
/// - When `erfcc = true`, `expo = true` and `x > 0`, the function computes
///   the scaled complementary error function `exp(x*x}erfc(x)`.
/// - When `erfcc = false` and `expo = false`, the function computes
///   the error function `erf(x)`.
fn errorfunction(x: f64, erfcc: bool, expo: bool) -> f64 {
    if erfcc {
        if x < -6.5 {
            2.0
        } else if x < 0.0 {
            2.0 - errorfunction(-x, true, false)
        } else if x == 0.0 {
            1.0
        } else if x < 0.5 {
            let y = if expo { (x * x).exp() } else { 1.0 };

            y * (1.0 - errorfunction(x, false, false))
        } else if x < 4.0 {
            let y = if expo { 1.0 } else { (-x * x).exp() };

            const R: [f64; 9] = [
                1.230339354797997253e3,
                2.051078377826071465e3,
                1.712047612634070583e3,
                8.819522212417690904e2,
                2.986351381974001311e2,
                6.611919063714162948e1,
                8.883149794388375941,
                5.641884969886700892e-1,
                2.153115354744038463e-8,
            ];
            const S: [f64; 8] = [
                1.230339354803749420e3,
                3.439367674143721637e3,
                4.362619090143247158e3,
                3.290799235733459627e3,
                1.621389574566690189e3,
                5.371811018620098575e2,
                1.176939508913124993e2,
                1.574492611070983473e1,
            ];

            y * fractio(x, 8, &R, &S)
        } else {
            let mut z = x * x;
            let y = if expo { 1.0 } else { (-z).exp() };
            z = 1.0 / z;

            const R: [f64; 6] = [
                6.587491615298378032e-4,
                1.608378514874227663e-2,
                1.257817261112292462e-1,
                3.603448999498044394e-1,
                3.053266349612323440e-1,
                1.631538713730209785e-2,
            ];
            const S: [f64; 5] = [
                2.335204976268691854e-3,
                6.051834131244131912e-2,
                5.279051029514284122e-1,
                1.872952849923460472,
                2.568520192289822421,
            ];

            y * (ONE_OVER_SQRT_PI - z * fractio(z, 5, &R, &S)) / x
        }
    } else {
        if x == 0.0 {
            0.0
        } else if x.abs() > 6.5 {
            x / x.abs()
        } else if x > 0.5 {
            1.0 - errorfunction(x, true, false)
        } else if x < -0.5 {
            errorfunction(-x, true, false) - 1.0
        } else {
            const R: [f64; 5] = [
                3.209377589138469473e3,
                3.774852376853020208e2,
                1.138641541510501556e2,
                3.161123743870565597e0,
                1.857777061846031527e-1,
            ];
            const S: [f64; 4] = [
                2.844236833439170622e3,
                1.282616526077372276e3,
                2.440246379344441733e2,
                2.360129095234412093e1,
            ];
            let z = x * x;

            x * fractio(z, 4, &R, &S)
        }
    }
}

/// Inverse of the complementary error function
///     
/// Input: x, real positive number.
fn inverfc(x: f64) -> f64 {
    if x == 0.0 {
        GIANT
    } else {
        if x > 1.0 {
            -inverfc(2.0 - x)
        } else {
            #[allow(clippy::approx_constant)]
            let y0 = 0.70710678 * invq(x * 0.5);
            let f = errorfunction(y0, true, false) - x;
            let y02 = y0 * y0;
            let fp = -2.0 / SQRT_PI * (-y02).exp();
            let c1 = -1.0 / fp;
            let c2 = y0;
            let c3 = (4.0 * y02 + 1.0) * 0.333333333333333333333333333333;
            let c4 = y0 * (12.0 * y02 + 7.0) * 0.166666666666666666666666666667;
            let c5 = (8.0 * y02 + 7.0)
                * (12.0 * y02 + 1.0)
                * 0.333333333333333333333333333333;
            let r = f * c1;
            let h = r * (1.0 + r * (c2 + r * (c3 + r * (c4 + r * c5))));

            y0 + h
        }
    }
}

/// Computation of the Euler gamma function
/// Gamma(x).
///
///  Input: x, real number.
fn gamma(x: f64) -> f64 {
    let k = x.round() as i32;
    let k1 = k - 1;
    let dw = if k == 0 { DWARF } else { MACHTOL };

    if (k <= 0) && (((k as f64) - x).abs() <= dw) {
        if (k % 2) > 0 {
            // k is odd
            1.0_f64.copysign((k as f64) - x) * GIANT
        } else {
            // k is even
            1.0_f64.copysign(x - (k as f64)) * GIANT
        }
    } else if x < 0.45 {
        PI / ((PI * x).sin() * gamma(1.0 - x))
    } else if (((k as f64) - x).abs() < dw) && (x < 21.0) {
        let mut value = 1.0;
        for n in 2..=k1 {
            value *= n as f64;
        }

        value
    } else if (((k as f64) - x - 0.5).abs() < dw) && (x < 21.0) {
        let mut value = PI.sqrt();
        for n in 1..=k1 {
            value *= (n as f64) - 0.5;
        }

        value
    } else if x < 3.0 {
        let k = if (k as f64) > x { k1 } else { k };

        let k1 = 3 - k;
        let z = (k1 as f64) + x;
        let mut value = gamma(z);
        for n in 1..=k1 {
            value /= z - (n as f64);
        }

        value
    } else {
        SQRT_TWO_PI * (-x + (x - 0.5) * x.ln() + stirling(x)).exp()
    }
}

/// Computation of the logarithm of
/// the Gamma Function, log(gamma(x))
///
/// Input: x, real positive number.
fn loggam(x: f64) -> f64 {
    if x >= 3.0 {
        (x - 0.5) * x.ln() - x + LN_SQRT_TWO_PI + stirling(x)
    } else if x >= 2.0 {
        (x - 2.0) * (3.0 - x) * auxloggam(x - 2.0) + logoneplusx(x - 2.0)
    } else if x >= 1.0 {
        (x - 1.0) * (2.0 - x) * auxloggam(x - 1.0)
    } else if x > 0.5 {
        x * (1.0 - x) * auxloggam(x) - logoneplusx(x - 1.0)
    } else if x > 0.0 {
        x * (1.0 - x) * auxloggam(x) - x.ln()
    } else {
        GIANT
    }
}

/// Computation of the regulated gamma function:
///    gamstar(x) = exp(stirling(x)), x>0; or
///    gamma(x)/(exp(-x+(x-0.5)*ln(x))/sqrt(2*pi)
///
/// Input: x, real positive number.
fn gamstar(x: f64) -> f64 {
    if x >= 3.0 {
        (stirling(x)).exp()
    } else if x > 0.0 {
        gamma(x) / ((-x + (x - 0.5) * x.ln()).exp() * SQRT_TWO_PI)
    } else {
        GIANT
    }
}

/// Computation of the quotient of two gamma functions
///  gamma(x)/gamma(y)
fn quotgamm(x: f64, y: f64) -> f64 {
    if (x <= 0.0) || (y <= 0.0) {
        qratio(x, y)
    } else if x > y {
        1.0 / quotgamm(y, x)
    } else {
        let n = (y - x) as i32;

        if (n as f64) == (y - x) {
            1.0 / shiftfact(x, n)
        } else if n > 15 {
            qratio(x, y)
        } else if n > 0 {
            quotgamm(x + (n as f64), y) / shiftfact(x, n)
        } else if x < 26.0 {
            qratio(x, y)
        } else {
            qasym(x, y)
        }
    }
}

fn qratio(x: f64, y: f64) -> f64 {
    if ((x <= 0.0) || (y <= 0.0)) || (y > 2.5 * x) {
        gamma(x) / gamma(y)
    } else {
        let b = y - x;
        let c = b / x;
        let q = lnec(c);
        let q = (-b * x.ln() - (b - 0.5) * (1.0 + c).ln() - x * q).exp();

        q * gamstar(x) / gamstar(y)
    }
}

fn qasym(x: f64, y: f64) -> f64 {
    let w = (x + y - 1.0) / 2.0;
    let w2 = 1.0 / (w * w);
    let r = (x - y + 1.0) / 2.0;
    let r2 = r * r;
    let r3 = r2 * r;
    let r4 = r2 * r2;
    let r5 = r4 * r;
    let r6 = r5 * r;
    let r7 = r6 * r;

    let cc = [
        1.0,
        r / 12.0,
        r / 1440.0 + r2 / 288.0,
        r / 90720.0 + r2 / 17280.0 + r3 / 10368.0,
        r / 4838400.0 + 101.0 * r2 / 87091200.0 + r3 / 414720.0 + r4 / 497664.0,
        r / 239500800.0
            + 13.0 * r2 / 522547200.0
            + 61.0 * r3 / 1045094400.0
            + r4 / 14929920.0
            + r5 / 29859840.0,
        691.0 * r / 7846046208000.0
            + 7999.0 * r2 / 14485008384000.0
            + 59.0 * r3 / 41803776000.0
            + 143.0 * r4 / 75246796800.0
            + r5 / 716636160.0
            + r6 / 2149908480.0,
        r / 523069747200.0
            + 2357.0 * r2 / 188305108992000.0
            + 5941.0 * r3 / 173820100608000.0
            + 11.0 * r4 / 214990848000.0
            + 41.0 * r5 / 902961561600.0
            + r6 / 42998169600.0
            + r7 / 180592312320.0,
    ];

    let mut s = 1.0;
    let mut k = 1;
    let mut t = 1.0;
    let mut u = 1.0_f64;
    let mut value = 0.0;
    while (u.abs() > MACHTOL) && (k < 8) {
        t *= -4.0 * w2 * ((k as f64) - r) * ((k as f64) - r - 0.5);
        u = t * cc[k];
        s += u;
        k += 1;
        value = s * ((x - y) * w.ln()).exp();
    }

    value
}

fn shiftfact(x: f64, n: i32) -> f64 {
    if n == 0 {
        1.0
    } else if n < 0 {
        1.0 / shiftfact(x - (n as f64), n)
    } else {
        let mut s = 1.0;
        for k in 0..n {
            s *= x + (k as f64);
        }

        s
    }
}

fn invq(x: f64) -> f64 {
    let t = (-2.0 * x.ln()).sqrt();

    t - (2.515517 + t * (0.802853 + t * 0.010328))
        / (1.0 + t * (1.432788 + t * (0.189269 + t * 0.001308)))
}

fn xetay(zeta: f64, mu: f64, y: f64) -> f64 {
    let mu2 = mu * mu;
    let z2 = 0.5 * zeta * zeta;
    let p = y * y;
    let q = mu - y;
    let r = y - mu + mu * (mu / y).ln();
    let s = 2.0 * (z2 - r);

    let x0 = if y <= mu {
        let p = q * q + p * s;

        if p > 0.0 {
            s / (q + p.sqrt())
        } else {
            y + z2 + (mu2 + 4.0 * y * z2).sqrt()
        }
    } else {
        if zeta > 0.0 {
            0.0
        } else {
            y + z2 + (mu2 + 4.0 * y * z2).sqrt()
        }
    };

    let mut x = if x0 < 0.0 { 0.0 } else { x0 };

    let mut x0 = fxeta(x, y, mu, z2);
    let mut k = 1;
    if x0 < 0.0 {
        k = 20;
    }
    while ((x / x0 - 1.0).abs() > 1.0e-12) && (k < 20) {
        x = x0;
        x0 = fxeta(x, y, mu, z2);
        if x0 < 0.0 {
            k = 20;
        }
        k += 1;
    }
    if k < 20 {
        x = x0;
    }

    x
}

fn xzetay(zeta: f64, y: f64) -> f64 {
    let tym1 = 2.0 * y - 1.0;
    let w = tym1 * tym1;
    let z = zeta * ((2.0 * y - 1.0).abs()).sqrt() / w;
    let mut x;

    if (z.abs() < 0.03) && (y >= 1.0) {
        let mut yk = [0.0; 11];
        yk[2] = y * y;
        for k in 3..=8 {
            yk[k] = y * yk[k - 1];
        }

        let mut ck = [0.0; 11];
        ck[2..11].copy_from_slice(&[
            0.3333333333333333333333333333 * (3.0 * y - 1.0),
            0.0277777777777777777777777777778 * (6.0 * y - 1.0),
            0.0037037037037037037037037037037 * (54.0 * yk[2] - 9.0 * y + 1.0),
            0.000231481481481481481481481481481
                * (1152.0 * yk[3] - 36.0 * yk[2] + 12.0 * y - 1.0),
            0.0000587889476778365667254556143445
                * (6480.0 * yk[4] + 1404.0 * yk[3] - 108.0 * yk[2] + 15.0 * y
                    - 1.0),
            1.83715461493239271017048794827e-7
                * (3110400.0 * yk[5] + 1741824.0 * yk[4] - 78408.0 * yk[3]
                    + 20196.0 * yk[2]
                    - 2502.0 * y
                    + 139.0),
            0.00000489907897315304722712130119538
                * (181440.0 * yk[6]
                    + 178848.0 * yk[5]
                    + 7128.0 * yk[4]
                    + 1044.0 * yk[3]
                    - 198.0 * yk[2]
                    + 21.0 * y
                    - 1.0),
            4.25267271975090905132057395432e-10
                * (571.0 + 144072.0 * yk[2] - 13704.0 * y
                    + 3344302080.0 * yk[7]
                    + 4961710080.0 * yk[6]
                    + 805469184.0 * yk[5]
                    - 845856.0 * yk[3]
                    + 9283248.0 * yk[4]),
            6.59808615912868313417010262003e-10
                * (3527193600.0 * yk[8]
                    + 7215350400.0 * yk[7]
                    + 2287963584.0 * yk[6]
                    + 75781008.0 * yk[5]
                    + 3557520.0 * yk[4]
                    - 738720.0 * yk[3]
                    + 95580.0 * yk[2]
                    - 7587.0 * y
                    + 281.0),
        ]);

        let mut s = -z;
        let mut t = 1.0;
        let mut k = 2;
        let mut z2 = z * z;
        while ((t / s).abs() > 1.0e-15) && (k < 11) {
            t = ck[k] * z2;
            s += t;
            z2 *= z;
            k += 1;
        }

        x = y - 1.0 + w * s;
    } else {
        let z2 = 0.5 * zeta * zeta;
        let mut p = y * y;
        let q = 1.0 - y;
        let r = y - 1.0 - y.ln();
        let s = 2.0 * (z2 - r);

        x = if y <= 1.0 {
            p = q * q + p * s;

            if p > 0.0 {
                s / (q + p.sqrt())
            } else {
                y + z2 + (1.0 + 4.0 * y * z2).sqrt()
            }
        } else {
            if zeta > 0.0 {
                0.0
            } else {
                y + z2 + (1.0 + 4.0 * y * z2).sqrt()
            }
        };

        let mut x0 = fx(x, y, z2);
        let mut k = 1;
        while ((x / x0 - 1.0).abs() > 1.0e-12) && (k < 20) {
            x = x0;
            x0 = fx(x, y, z2);
            k += 1;
        }

        x = x0;
    }

    x
}

fn yzetax(zeta: f64, x: f64) -> f64 {
    let txp1 = 2.0 * x + 1.0;
    let w = txp1 * txp1;
    let z = zeta * txp1.sqrt() / w;
    let mut y;

    if z.abs() < 0.05 {
        let mut xk = [0.0; 101];
        xk[2] = x * x;
        for k in 3..=8 {
            xk[k] = x * xk[k - 1];
        }

        let mut ck = [0.0; 101];
        ck[2..11].copy_from_slice(&[
            0.3333333333333333333333333333 * (3.0 * x + 1.0),
            0.0277777777777777777777777777778 * (6.0 * x + 1.0),
            -0.0037037037037037037037037037037 * (54.0 * xk[2] + 9.0 * x + 1.0),
            0.000231481481481481481481481481481
                * (1152.0 * xk[3] + 36.0 * xk[2] + 12.0 * x + 1.0),
            -0.0000587889476778365667254556143445
                * (6480.0 * xk[4]
                    - 1404.0 * xk[3]
                    - 108.0 * xk[2]
                    - 15.0 * x
                    - 1.0),
            1.83715461493239271017048794827e-7
                * (3110400.0 * xk[5]
                    - 1741824.0 * xk[4]
                    - 78408.0 * xk[3]
                    - 20196.0 * xk[2]
                    - 2502.0 * x
                    - 139.0),
            -0.00000489907897315304722712130119538
                * (181440.0 * xk[6] - 178848.0 * xk[5] + 7128.0 * xk[4]
                    - 1044.0 * xk[3]
                    - 198.0 * xk[2]
                    - 21.0 * x
                    - 1.0),
            4.25267271975090905132057395432e-10
                * (3344302080.0 * xk[7] - 4961710080.0 * xk[6]
                    + 805469184.0 * xk[5]
                    - 9283248.0 * xk[4]
                    - 845856.0 * xk[3]
                    - 144072.0 * xk[2]
                    - 13704.0 * x
                    - 571.0),
            -6.59808615912868313417010262003e-10
                * (281.0
                    + 7587.0 * x
                    + 95580.0 * xk[2]
                    + 738720.0 * xk[3]
                    + 3557520.0 * xk[4]
                    - 75781008.0 * xk[5]
                    + 2287963584.0 * xk[6]
                    - 7215350400.0 * xk[7]
                    + 3527193600.0 * xk[8]),
        ]);

        let mut s = z;
        let mut t = 1.0;
        let mut k = 2;
        let mut z2 = z * z;
        while ((t / s).abs() > 1.0e-15) && (k < 11) {
            t = ck[k] * z2;
            s += t;
            z2 *= z;
            k += 1;
        }

        y = x + 1.0 + w * s;
    } else {
        let z2 = 0.5 * zeta * zeta;
        let mut y0 = if zeta > 0.0 { x + 2.0 } else { (-z2).exp() };

        y = 0.0;
        let mut k = 1;
        while ((y / y0 - 1.0).abs() > 1.0e-8) && (k < 20) {
            y = y0;
            y0 = fy(x, y, z2);
            k += 1;
        }
        y = y0;
    }

    y
}

fn yetax(zeta: f64, mu: f64, x: f64) -> f64 {
    let z2 = 0.5 * zeta * zeta;
    let mut y0 = if zeta > 0.0 {
        x + mu + 1.0
    } else {
        (-z2).exp()
    };

    let mut y = 0.0;
    let mut k = 1;
    while ((y / y0 - 1.0).abs() > 1.0e-8) && (k < 20) {
        y = y0;
        y0 = fyeta(x, y, mu, z2);
        if y0 < 0.0 {
            k = 20;
        }
        k += 1;
    }
    if k < 20 {
        y = y0;
    }

    y
}

fn fxeta(x: f64, y: f64, mu: f64, z2: f64) -> f64 {
    let mu2 = mu * mu;
    let w = (mu2 + 4.0 * x * y).sqrt();
    let fd = (w + mu - 2.0 * y) / (mu + w);
    let f = x + y - w + mu * ((mu + w) / (2.0 * y)).ln() - z2;

    x - f / fd
}

fn fx(x: f64, y: f64, z2: f64) -> f64 {
    let w = (1.0 + 4.0 * x * y).sqrt();
    let fd = (w + 1.0 - 2.0 * y) / (1.0 + w);
    let f = x + y - w + ((1.0 + w) / (2.0 * y)).ln() - z2;

    x - f / fd
}

fn fyeta(x: f64, y: f64, mu: f64, z2: f64) -> f64 {
    let mu2 = mu * mu;
    let w = (mu2 + 4.0 * x * y).sqrt();
    let fd = ((y - mu) * w + y * mu - 2.0 * x * y - mu2) / (y * (mu + w));
    let f = x + y - w + mu * ((mu + w) / (2.0 * y)).ln() - z2;

    y - f / fd
}

fn fy(x: f64, y: f64, z2: f64) -> f64 {
    let w = (1.0 + 4.0 * x * y).sqrt();
    let fd = ((y - 1.0) * w + y - 2.0 * x * y - 1.0) / (y * (1.0 + w));
    let f = x + y - w + ((1.0 + w) / (2.0 * y)).ln() - z2;

    y - f / fd
}

fn eta1xy(zeta: f64, x: f64, y: f64, mu: f64) -> f64 {
    let mu2 = mu * mu;
    let w = (mu2 + 4.0 * x * y).sqrt();
    let f0 = zeta * (mu + w) / ((-mu - w + 2.0 * y) * w.sqrt());

    f0.ln() / zeta
}

fn zeta1xy(zeta: f64, x: f64, y: f64) -> f64 {
    let txp1 = 2.0 * x + 1.0;
    let w = txp1 * txp1.sqrt();
    let z = zeta / w;

    if z.abs() < 0.03 {
        let mut xk = [0.0; 12];
        xk[2] = x * x;
        let mut zk = [0.0; 12];
        zk[2] = z * z;
        for k in 3..=11 {
            xk[k] = x * xk[k - 1];
            zk[k] = z * zk[k - 1];
        }

        let ck = [
            -(1.0 / 3.0) * (3.0 * x + 1.0),
            (1.0 / 36.0) * (36.0 * xk[2] + 6.0 * x + 1.0) * z,
            -(1.0 / 1620.0)
                * (2160.0 * xk[3] - 594.0 * xk[2] - 9.0 * x - 1.0)
                * zk[2],
            (1.0 / 6480.0)
                * (12960.0 * xk[4] - 11664.0 * xk[3] + 108.0 * xk[2]
                    - 84.0 * x
                    - 7.0)
                * zk[3],
            -(1.0 / 90720.0)
                * (-2952.0 * xk[2] - 375.0 * x
                    + 62676.0 * xk[3]
                    + 290304.0 * xk[5]
                    - 482112.0 * xk[4]
                    - 25.0)
                * zk[4],
            (1.0 / 382725.0)
                * (-1809.0 * xk[2] - 198.0 * x + 2041200.0 * xk[6]
                    - 11.0
                    - 72117.0 * xk[3]
                    - 5161320.0 * xk[5]
                    + 1588734.0 * xk[4])
                * zk[5],
            -(1.0 / 73483200.0)
                * (1147209.0 * x
                    + 54629.0
                    + 10506132.0 * xk[2]
                    + 53276346.0 * xk[3]
                    + 145651608.0 * xk[4]
                    + 26074872.0 * xk[5]
                    - 3096325440.0 * xk[6]
                    + 17239858560.0 * xk[7])
                * zk[6],
            (1.0 / 9797760.0)
                * (9864.0 * xk[2]
                    + 888.0 * x
                    + 562961664.0 * xk[6]
                    + 37.0
                    + 156764160.0 * xk[8]
                    - 707927040.0 * xk[7]
                    + 75456.0 * xk[3]
                    - 100476288.0 * xk[5]
                    + 3282768.0 * xk[4])
                * zk[7],
            -(1.0 / 3491921664000.0)
                * (1085205600.0 * xk[2] + 85989897.0 * x
                    - 173727743003136.0 * xk[6]
                    + 3184811.0
                    + 99325771776000.0 * xk[9]
                    - 557433645465600.0 * xk[8]
                    + 609525121720320.0 * xk[7]
                    + 8540105400.0 * xk[3]
                    + 11923016301072.0 * xk[5]
                    - 8705251440.0 * xk[4])
                * zk[8],
            (1.0 / 15913705500.0)
                * (523341.0 * xk[2]
                    + 36930.0 * x
                    + 386373372840.0 * xk[6]
                    + 1231.0
                    + 814781721600.0 * xk[10]
                    - 5512856630400.0 * xk[9]
                    + 7843246148160.0 * xk[8]
                    - 3236596377408.0 * xk[7]
                    + 4431105.0 * xk[3]
                    - 8476062588.0 * xk[5]
                    + 4677750.0 * xk[4])
                * zk[9],
            -(1.0 / 84737299046400.0)
                * (-1405644714.0 * xk[2]
                    - 90601269.0 * x
                    - 570910822987680.0 * xk[6]
                    - 2745493.0
                    - 62881400385699840.0 * xk[10]
                    + 111932154011713536.0 * xk[9]
                    - 62582386500956160.0 * xk[8]
                    + 11503114507558656.0 * xk[7]
                    - 13895395416.0 * xk[3]
                    + 1634520610800.0 * xk[5]
                    - 100596777120.0 * xk[4]
                    + 7888272202137600.0 * xk[11])
                * zk[10],
        ];

        let mut s = ck[0] + ck[1];
        let mut t = 1.0_f64;
        let mut k = 2;
        while (t.abs() > 1.0e-15) && (k < 11) {
            t = ck[k];
            s += t;
            k += 1;
        }

        s / w
    } else {
        let w = (1.0 + 4.0 * x * y).sqrt();
        let f0 = zeta * (1.0 + 2.0 * x + w) / (2.0 * (y - x - 1.0) * w.sqrt());

        f0.ln() / zeta
    }
}

fn zeta3xy(iflag: i32, zeta0: f64, x: f64, y: f64) -> (f64, f64, f64) {
    let mut zeta1 = zeta1xy(zeta0, x, y);
    let mut zeta2;
    let mut zeta3;
    let pp = 1.0 / (1.0 + 4.0 * x * y).sqrt();

    if (zeta0.abs() >= 0.1) && ((1.0 - pp).abs() > 1.0e-4) {
        let p = pp;
        let pp2 = pp * pp;
        let p3 = pp2 * pp;
        let p4 = p3 * pp;
        let p5 = p4 * pp;
        let p6 = p5 * pp;
        let p7 = p6 * pp;
        let p8 = p7 * pp;
        let p9 = p8 * pp;
        let p10 = p9 * pp;
        let zeta02 = zeta0 * zeta0;
        let zeta03 = zeta02 * zeta0;
        let zeta04 = zeta03 * zeta0;
        let zeta05 = zeta04 * zeta0;
        let zeta06 = zeta05 * zeta0;
        let zeta12 = zeta1 * zeta1;
        let zeta13 = zeta12 * zeta1;
        if iflag == 1 {
            let xx2 = x * x;
            let x3 = xx2 * x;
            let x4 = x3 * x;
            let dd = (2.0 * pp * x + pp - 1.0) * (2.0 * pp * x + pp - 1.0);
            let dd4 = dd * dd;

            zeta2 = -(1.0 / 24.0)
                * (-24.0 + 48.0 * pp - 24.0 * pp2 - 96.0 * pp2 * xx2
                    + 96.0 * pp * x
                    + 38.0 * zeta02 * p3
                    + 5.0 * zeta02 * p5
                    - 22.0 * zeta02 * p4
                    + 12.0 * zeta02 * zeta12
                    + 48.0 * zeta02 * zeta12 * x * pp2
                    - 48.0 * zeta02 * zeta12 * x * pp
                    - 24.0 * zeta1 * zeta03 * x * p4
                    + 24.0 * zeta1 * zeta03 * x * pp2
                    - 36.0 * zeta03 * zeta1 * pp2
                    + 12.0 * zeta02 * zeta12 * pp2
                    + 12.0 * zeta03 * zeta1 * pp
                    + 36.0 * zeta02 * x * pp2
                    - 12.0 * zeta02 * x * p3
                    + 48.0 * pp2 * xx2 * zeta02 * zeta12
                    - 24.0 * pp * zeta02 * zeta12
                    + 9.0 * zeta02 * pp
                    - 30.0 * zeta02 * pp2
                    - 96.0 * x * pp2
                    - 12.0 * zeta03 * zeta1 * p4
                    + 36.0 * zeta03 * zeta1 * p3
                    - 44.0 * zeta02 * x * p4
                    + 20.0 * zeta02 * x * p5
                    + 20.0 * p5 * xx2 * zeta02
                    - 12.0 * p3 * xx2 * zeta02)
                / (zeta03
                    * ((2.0 * pp * x + pp - 1.0) * (2.0 * pp * x + pp - 1.0)));
            zeta3 = if zeta0 > 0.1 {
                (1.0 / 48.0)
                    * (-120.0
                        + 480.0 * p
                        + 2880.0 * p3 * x
                        + 960.0 * pp * x
                        + 240.0 * p4 * zeta02
                        - 24.0 * zeta02 * p
                        - 240.0 * zeta02 * p3
                        - 2880.0 * pp2 * xx2
                        + 120.0 * pp2 * zeta02
                        + 5760.0 * p3 * xx2
                        - 120.0 * zeta02 * p5
                        + 24.0 * p6 * zeta02
                        - 2880.0 * p4 * xx2
                        - 3840.0 * p4 * x3
                        + 3840.0 * p3 * x3
                        - 960.0 * p4 * x
                        - 720.0 * pp2
                        + 480.0 * p3
                        - 120.0 * p4
                        - 2880.0 * pp2 * x
                        + 576.0 * zeta02 * p4 * x
                        + 48.0 * zeta02 * pp2 * x
                        - 288.0 * zeta02 * p3 * x
                        + 144.0 * zeta02 * p6 * x
                        - 480.0 * zeta02 * p5 * x
                        + 288.0 * zeta02 * p6 * xx2
                        + 96.0 * zeta02 * p4 * xx2
                        + 96.0 * zeta02 * p3 * xx2
                        - 480.0 * zeta02 * p5 * xx2
                        - 192.0 * p5 * x3 * zeta03 * zeta1
                        + 320.0 * zeta03 * p7 * x3 * zeta1
                        + 240.0 * zeta03 * p7 * xx2 * zeta1
                        - 96.0 * p5 * x4 * zeta03 * zeta1
                        + 80.0 * zeta1 * zeta03 * x * p7
                        + 1536.0 * zeta0 * zeta1 * x3 * p3
                        + 1152.0 * zeta0 * zeta1 * x * p3
                        + 2304.0 * zeta0 * zeta1 * xx2 * p3
                        + 160.0 * zeta03 * x4 * p7 * zeta1
                        - 240.0 * zeta1 * zeta05 * x3 * p9
                        - 48.0 * zeta0 * zeta1 * p4
                        - 168.0 * p5 * x3 * zeta04
                        + 174.0 * zeta03 * p5 * zeta1
                        - 90.0 * zeta05 * zeta1 * p5
                        + 30.0 * zeta05 * zeta1 * p3
                        - 6.0 * zeta05 * zeta1 * pp2
                        - 204.0 * p5 * xx2 * zeta04
                        - 780.0 * zeta04 * x * p4
                        + 420.0 * p4 * xx2 * zeta04
                        + 192.0 * p6 * x3 * zeta02
                        - 365.0 * zeta04 * p7
                        + 305.0 * zeta04 * p6
                        - 95.0 * zeta04 * p9
                        + 253.0 * zeta04 * p8
                        + 15.0 * zeta04 * p10
                        - 1920.0 * p4 * x4
                        - 145.0 * zeta04 * p5
                        + 35.0 * zeta04 * p4
                        - 48.0 * zeta0 * zeta1
                        - 256.0 * zeta03 * zeta1 * p4
                        + 214.0 * zeta03 * zeta1 * p3
                        - 288.0 * zeta0 * zeta1 * pp2
                        - 96.0 * zeta03 * zeta1 * pp2
                        + 18.0 * zeta03 * zeta1 * p
                        + 192.0 * zeta0 * zeta1 * p
                        - 120.0 * pp2 * zeta04 * zeta12
                        - 12.0 * pp2 * zeta06 * zeta12
                        - 96.0 * p4 * zeta06 * zeta12
                        + 60.0 * p3 * zeta06 * zeta12
                        + 240.0 * p3 * zeta04 * zeta12
                        - 96.0 * zeta03 * pp * zeta13
                        + 144.0 * zeta03 * pp2 * zeta13
                        + 24.0 * zeta03 * zeta13
                        + 24.0 * pp * zeta04 * zeta12
                        + 120.0 * p5 * zeta04 * zeta12
                        + 24.0 * p8 * zeta06 * zeta12
                        + 24.0 * p5 * zeta06 * zeta12
                        - 84.0 * p7 * zeta06 * zeta12
                        + 84.0 * p6 * zeta06 * zeta12
                        - 24.0 * p6 * zeta04 * zeta12
                        + 24.0 * zeta03 * p4 * zeta13
                        - 96.0 * p4 * zeta04 * zeta12 * xx2
                        + 480.0 * p5 * zeta04 * zeta12 * xx2
                        + 192.0 * p4 * zeta04 * zeta12 * x3
                        + 288.0 * p3 * zeta04 * zeta12 * x
                        + 480.0 * p5 * zeta06 * zeta12 * x
                        + 72.0 * p3 * zeta06 * zeta12 * x
                        - 336.0 * p4 * zeta06 * zeta12 * x
                        + 96.0 * p4 * zeta06 * zeta12 * xx2
                        - 192.0 * p6 * zeta06 * zeta12 * xx2
                        + 96.0 * p8 * zeta06 * zeta12 * xx2
                        - 192.0 * p6 * zeta04 * zeta12 * x3
                        - 576.0 * zeta03 * p3 * zeta13 * x
                        - 192.0 * zeta03 * pp * zeta13 * x
                        + 576.0 * zeta03 * pp2 * zeta13 * xx2
                        + 576.0 * zeta03 * p4 * zeta13 * xx2
                        + 768.0 * zeta03 * p4 * zeta13 * x3
                        - 768.0 * zeta03 * p3 * zeta13 * x3
                        + 576.0 * zeta03 * pp2 * zeta13 * x
                        + 384.0 * zeta03 * p4 * zeta13 * x4
                        - 576.0 * p4 * zeta04 * zeta12 * x
                        - 48.0 * pp2 * zeta04 * zeta12 * x
                        - 96.0 * p3 * zeta04 * zeta12 * xx2
                        - 144.0 * p6 * zeta04 * zeta12 * x
                        + 96.0 * p8 * zeta06 * zeta12 * x
                        - 144.0 * p6 * zeta06 * zeta12 * x
                        - 168.0 * p7 * zeta06 * zeta12 * x
                        - 96.0 * zeta03 * p3 * zeta13
                        + 192.0 * zeta03 * p4 * zeta13 * x
                        + 480.0 * p5 * zeta04 * zeta12 * x
                        - 288.0 * p6 * zeta04 * zeta12 * xx2
                        - 240.0 * p4 * zeta04 * zeta12
                        - 1152.0 * zeta03 * xx2 * p3 * zeta13
                        + 600.0 * zeta05 * zeta1 * p8 * x
                        - 360.0 * zeta05 * zeta1 * p9 * xx2
                        - 180.0 * zeta05 * zeta1 * p9 * x
                        + 312.0 * zeta05 * zeta1 * p4 * xx2
                        - 744.0 * zeta05 * zeta1 * p4 * x
                        + 156.0 * zeta05 * zeta1 * p3 * x
                        + 432.0 * zeta05 * zeta1 * p7 * xx2
                        - 912.0 * zeta05 * zeta1 * p6 * xx2
                        - 444.0 * zeta05 * zeta1 * p7 * x
                        - 624.0 * zeta05 * zeta1 * p6 * x
                        - 48.0 * zeta05 * zeta1 * p5 * x3
                        - 72.0 * zeta05 * zeta1 * p5 * xx2
                        + 1236.0 * zeta05 * zeta1 * p5 * x
                        + 288.0 * zeta1 * zeta05 * x3 * p7
                        - 3.0 * zeta04 * p3
                        + 96.0 * p6 * x3 * zeta04
                        - 576.0 * p8 * x3 * zeta04
                        - 288.0 * p8 * x4 * zeta04
                        + 162.0 * zeta04 * p3 * x
                        + 10.0 * zeta03 * zeta1 * p7
                        + 48.0 * p6 * x4 * zeta04
                        - 64.0 * zeta03 * p6 * zeta1
                        - 294.0 * zeta05 * zeta1 * p7
                        + 270.0 * zeta05 * zeta1 * p6
                        - 30.0 * zeta05 * zeta1 * p9
                        + 150.0 * zeta05 * zeta1 * p8
                        - 30.0 * zeta05 * zeta1 * p4
                        + 192.0 * zeta0 * zeta1 * p3
                        + 144.0 * zeta03 * x * p3 * zeta1
                        + 672.0 * zeta03 * p5 * x * zeta1
                        + 576.0 * zeta03 * xx2 * p5 * zeta1
                        - 384.0 * zeta1 * zeta0 * x * p4
                        - 1536.0 * p4 * x3 * zeta0 * zeta1
                        - 1152.0 * p4 * xx2 * zeta0 * zeta1
                        - 768.0 * p4 * x4 * zeta0 * zeta1
                        - 1140.0 * zeta04 * xx2 * p9
                        + 940.0 * zeta04 * x * p8
                        - 760.0 * p9 * x3 * zeta04
                        + 652.0 * p8 * xx2 * zeta04
                        + 928.0 * p7 * x3 * zeta04
                        - 1400.0 * p6 * xx2 * zeta04
                        + 360.0 * zeta04 * xx2 * p10
                        - 570.0 * zeta04 * x * p9
                        - 306.0 * zeta04 * x * p7
                        + 1312.0 * zeta04 * xx2 * p7
                        - 984.0 * zeta04 * x * p6
                        + 120.0 * zeta04 * p10 * x
                        + 1418.0 * zeta04 * p5 * x
                        - 192.0 * zeta02 * p4 * x3
                        + 480.0 * p10 * x3 * zeta04
                        + 240.0 * p10 * x4 * zeta04
                        + 192.0 * zeta03 * xx2 * p4 * zeta1
                        - 512.0 * zeta03 * x3 * p6 * zeta1
                        + 384.0 * zeta03 * x3 * p4 * zeta1
                        + 600.0 * zeta1 * zeta05 * xx2 * p8
                        - 240.0 * zeta03 * xx2 * p3 * zeta1
                        - 768.0 * zeta03 * p6 * xx2 * zeta1
                        - 384.0 * zeta03 * p6 * zeta1 * x
                        - 512.0 * zeta03 * zeta1 * p4 * x
                        - 1152.0 * zeta0 * zeta1 * pp2 * xx2
                        - 1152.0 * zeta0 * zeta1 * pp2 * x
                        + 384.0 * zeta0 * zeta1 * pp * x)
                    / (zeta05 * dd4)
            } else {
                0.0
            };

            if zeta2.abs() > (10.0 * zeta1).abs() {
                zeta2 = 0.0;
            }
            if zeta3.abs() > (5.0 * zeta2).abs() {
                zeta3 = 0.0;
            }
        } else {
            let y2 = y * y;
            let dd = (2.0 * pp * y + pp - 1.0) * (2.0 * pp * y + pp - 1.0);
            let _dd4 = dd * dd;

            zeta2 = -(1.0 / 24.0)
                * (-24.0 - 24.0 * pp2 - 20.0 * zeta02 * y * p5
                    + 60.0 * zeta02 * y * p3
                    + 4.0 * zeta02 * y * p4
                    + 36.0 * zeta02 * y * pp2
                    + 5.0 * zeta02 * p5
                    + 2.0 * zeta02 * p3
                    - 3.0 * zeta02 * pp
                    + 10.0 * zeta02 * p4
                    - 6.0 * zeta02 * pp2
                    - 96.0 * y2 * pp2
                    + 96.0 * y * pp2
                    - 48.0 * pp
                    + 96.0 * y * pp
                    + 12.0 * zeta02 * zeta12
                    + 48.0 * zeta02 * zeta12 * y2 * pp2
                    - 48.0 * zeta02 * zeta12 * y * pp2
                    - 48.0 * zeta02 * zeta12 * y * pp
                    - 24.0 * p4 * zeta1 * zeta03 * y
                    + 36.0 * zeta02 * y2 * p3
                    - 48.0 * zeta02 * y2 * p4
                    + 20.0 * zeta02 * y2 * p5
                    + 72.0 * pp2 * zeta1 * zeta03 * y
                    + 48.0 * p3 * zeta1 * zeta03 * y
                    + 12.0 * p4 * zeta03 * zeta1
                    + 12.0 * p3 * zeta1 * zeta03
                    - 12.0 * pp * zeta1 * zeta03
                    - 12.0 * pp2 * zeta1 * zeta03
                    + 12.0 * zeta02 * zeta12 * pp2
                    + 24.0 * zeta02 * zeta12 * pp)
                / (zeta03
                    * (-pp + 2.0 * y * pp - 1.0)
                    * (-pp + 2.0 * y * pp - 1.0));
            zeta3 = 0.0;
        }
    } else {
        if zeta0.abs() < 0.1 {
            let mut zk = [0.0; 14];
            zk[1] = zeta0;
            for k in 2..=13 {
                zk[k] = zeta0 * zk[k - 1];
            }

            let a1 = if iflag == 1 {
                -(2.0 * y - 1.0).sqrt()
            } else {
                (2.0 * x + 1.0).sqrt()
            };
            let a12 = a1 * a1;
            let a13 = a12 * a1;
            let a14 = a13 * a1;
            let a15 = a14 * a1;
            let a16 = a15 * a1;
            let a17 = a16 * a1;
            let a18 = a17 * a1;
            let a19 = a18 * a1;
            let a110 = a19 * a1;
            let a111 = a110 * a1;
            let a112 = a111 * a1;
            let a113 = a112 * a1;
            let a114 = a113 * a1;
            let a115 = a114 * a1;
            let a116 = a115 * a1;
            let a117 = a116 * a1;
            let a118 = a117 * a1;
            let a119 = a118 * a1;
            let a120 = a119 * a1;
            let a121 = a120 * a1;
            let a122 = a121 * a1;
            let a123 = a122 * a1;
            let a124 = a123 * a1;
            let a125 = a124 * a1;
            let a126 = a125 * a1;
            let a127 = a126 * a1;
            let a128 = a127 * a1;
            let a129 = a128 * a1;
            let a130 = a129 * a1;
            let a131 = a130 * a1;
            let a132 = a131 * a1;
            let a133 = a132 * a1;
            let a134 = a133 * a1;
            let a135 = a134 * a1;
            let a136 = a135 * a1;
            let a137 = a136 * a1;
            let a138 = a137 * a1;
            let a139 = a138 * a1;
            let a140 = a139 * a1;
            let a141 = a140 * a1;
            let a142 = a141 * a1;
            let a143 = a142 * a1;
            let a144 = a143 * a1;
            let a145 = a144 * a1;

            let z1k;
            let z2k;
            let z3k;

            if iflag == 1 {
                z1k = [
                    (1.0 / 6.0) * (-1.0 + 3.0 * a12) / a13,
                    -(1.0 / 36.0) * (-7.0 + 9.0 * a14 - 3.0 * a12) / a16,
                    (1.0 / 3240.0)
                        * (-27.0 * a14 - 1305.0 * a12 - 830.0 + 540.0 * a16)
                        / a19,
                    -(1.0 / 6480.0)
                        * (378.0 * a16 - 3861.0 * a14 - 5790.0 * a12 - 2330.0
                            + 810.0 * a18)
                        / a112,
                    (1.0 / 181440.0)
                        * (21384.0 * a18 - 316680.0 * a12 - 362187.0 * a14
                            + 18144.0 * a110
                            - 137403.0 * a16
                            - 95228.0)
                        / a115,
                    -(1.0 / 3061800.0)
                        * (-2687337.0 * a18
                            - 9958725.0 * a12
                            - 15661485.0 * a14
                            + 255150.0 * a112
                            + 524880.0 * a110
                            - 11068785.0 * a16
                            - 2409050.0)
                        / a118,
                    (1.0 / 32659200.0)
                        * (-188597241.0 * a18
                            - 193437300.0 * a12
                            - 383271210.0 * a14
                            + 7186320.0 * a112
                            - 31075812.0 * a110
                            + 2332800.0 * a114
                            - 381779055.0 * a16
                            - 39276200.0)
                        / a121,
                    -(1.0 / 9797760.0)
                        * (-224575173.0 * a18
                            - 104286000.0 * a12
                            - 248164308.0 * a14
                            - 9558324.0 * a112
                            - 83064852.0 * a110
                            + 2591352.0 * a114
                            - 315202860.0 * a16
                            + 612360.0 * a116
                            - 18277000.0)
                        / a124,
                    (1.0 / 10158317568000.0)
                        * (-109954260438210.0 * a18
                            + 46249674715125.0 * a12
                            + 62781588191700.0 * a14
                            - 54608288635548.0 * a112
                            - 119825060843610.0 * a110
                            - 7563831922620.0 * a114
                            - 8702169317100.0 * a16
                            + 453523384125.0 * a118
                            + 1757028090825.0 * a116
                            + 10636061704625.0)
                        / a127,
                    -(1.0 / 127309644000.0)
                        * (-4279315309500.0 * a12
                            - 13522731279750.0 * a14
                            - 24318275837625.0 * a16
                            - 27193050218565.0 * a18
                            - 19306988291745.0 * a110
                            - 8416870188111.0 * a112
                            - 1966263434982.0 * a114
                            - 110547732420.0 * a116
                            + 43701836850.0 * a118
                            + 6365482200.0 * a120
                            - 590307086600.0)
                        / a130,
                    (1.0 / 169474598092800.0)
                        * (-10054160837120400.0 * a12
                            - 35647819563676320.0 * a14
                            - 73341753727362120.0 * a16
                            - 96411337398782406.0 * a18
                            - 83837472483930165.0 * a110
                            - 48030703419545847.0 * a112
                            - 17258239538608068.0 * a114
                            - 3328592071143456.0 * a116
                            - 124745787998208.0 * a118
                            + 64150951173120.0 * a120
                            + 7703390822400.0 * a122
                            - 1254367500495200.0)
                        / a133,
                    -(1.0 / 127673385840000.0)
                        * (-13328252707770000.0 * a12
                            - 52369797400420500.0 * a14
                            - 121214416407084000.0 * a16
                            - 182880107334960750.0 * a18
                            - 187699522401641250.0 * a110
                            - 132420813236216505.0 * a112
                            - 63077110684718130.0 * a114
                            - 19108750611117258.0 * a116
                            - 3096024909924960.0 * a118
                            - 70413390440550.0 * a120
                            + 52537017231000.0 * a122
                            + 5319724410000.0 * a124
                            - 1518227841130000.0)
                        / a136,
                    (1.0 / 61010855313408000.0)
                        * (-11182030123135872000.0 * a12
                            - 48211055728647411600.0 * a14
                            - 123923771254537254000.0 * a16
                            - 210854486895115763700.0 * a18
                            - 249105487192355593800.0 * a110
                            - 208187581439825642655.0 * a112
                            - 122734345760116505415.0 * a114
                            - 49700657941195125384.0 * a116
                            - 12918774774646679328.0 * a118
                            - 1782426573908893440.0 * a120
                            - 19225434558297600.0 * a122
                            + 26994120536678400.0 * a124
                            + 2346571358208000.0 * a126
                            - 1172073510107192000.0)
                        / a139,
                ];
                let denom = 135.0 * a16 - 81.0 * a14 + 45.0 * a12 - 155.0;
                z2k = [
                    -(1.0 / 3240.0) * denom / a19,
                    (1.0 / 2592.0)
                        * (243.0 * a18
                            - 108.0 * a16
                            - 81.0 * a14
                            - 420.0 * a12
                            - 505.0)
                        / a112,
                    -(1.0 / 816480.0)
                        * (115668.0 * a110
                            - 10125.0 * a18
                            - 168777.0 * a16
                            - 422163.0 * a14
                            - 808185.0 * a12
                            - 450646.0)
                        / a115,
                    (1.0 / 2099520.0)
                        * (386370.0 * a112 + 169128.0 * a110
                            - 1119501.0 * a18
                            - 3223665.0 * a16
                            - 7046865.0 * a14
                            - 7557660.0 * a12
                            - 2817130.0)
                        / a118,
                    -(1.0 / 97977600.0)
                        * (21695040.0 * a114 + 24086160.0 * a112
                            - 96673419.0 * a110
                            - 379667007.0 * a18
                            - 930893985.0 * a16
                            - 1394843625.0 * a14
                            - 1038337440.0 * a12
                            - 293913460.0)
                        / a121,
                    (1.0 / 1763596800.0)
                        * (449166060.0 * a116 + 859525992.0 * a114
                            - 2656196064.0 * a112
                            - 14784723612.0 * a110
                            - 41801254713.0 * a18
                            - 77951791260.0 * a16
                            - 85566907944.0 * a14
                            - 48921809160.0 * a12
                            - 11223491860.0)
                        / a124,
                    -(1.0 / 1745960832000.0)
                        * (496767427200.0 * a118 + 1408198977360.0 * a116
                            - 3493441585260.0 * a114
                            - 27999657809937.0 * a112
                            - 92926054500345.0 * a110
                            - 205740653936895.0 * a18
                            - 294351301812645.0 * a16
                            - 252775949608710.0 * a14
                            - 117580863321900.0 * a12
                            - 22746727187800.0)
                        / a127,
                    (1.0 / 185177664000.0)
                        * (57686149080.0 * a120 + 222823107360.0 * a118
                            - 436405833540.0 * a116
                            - 5162127807816.0 * a114
                            - 20227890756213.0 * a112
                            - 52060153504905.0 * a110
                            - 91113392194965.0 * a18
                            - 103966409573460.0 * a16
                            - 73162123900740.0 * a14
                            - 28729989825600.0 * a12
                            - 4813736065880.0)
                        / a130,
                    -(1.0 / 4575814148505600.0)
                        * (1538064514022400.0 * a122
                            + 7672351342360320.0 * a120
                            - 11167805088902592.0 * a118
                            - 205652023257710016.0 * a116
                            - 950381517372586299.0 * a114
                            - 2810929213567245771.0 * a112
                            - 5804053176780884025.0 * a110
                            - 8234830657985188257.0 * a18
                            - 7765555911833079360.0 * a16
                            - 4626732255591405960.0 * a14
                            - 1573659776235346800.0 * a12
                            - 232808295847072400.0)
                        / a133,
                    (1.0 / 9899597917440000.0)
                        * (3551202490374000.0 * a124
                            + 22031130267554400.0 * a122
                            - 20837102148613440.0 * a120
                            - 675301583963071080.0 * a118
                            - 3664495983529292058.0 * a116
                            - 12356951269780552050.0 * a114
                            - 29435189701611164625.0 * a112
                            - 49859198031571298370.0 * a110
                            - 58946499123740602110.0 * a18
                            - 47245278542339908800.0 * a16
                            - 24406920061093785600.0 * a14
                            - 7326842720724450000.0 * a12
                            - 971150252248330000.0)
                        / a136,
                    -(1.0 / 1372744244551680000.0)
                        * (521075978809344000.0 * a126
                            + 3909422480008780800.0 * a124
                            - 1633235164439558400.0 * a122
                            - 135284080062035250720.0 * a120
                            - 857295093769286465124.0 * a118
                            - 3274109931036233003247.0 * a116
                            - 8863662891070952959995.0 * a114
                            - 17449997056857395695665.0 * a112
                            - 24805449338445590346675.0 * a110
                            - 25028886072322240272450.0 * a18
                            - 17423719285466831337000.0 * a16
                            - 7946267334428713399200.0 * a14
                            - 2135976241246185702000.0 * a12
                            - 256579957380860372000.0)
                        / a139,
                ];
                z3k = [
                    (1.0 / 816480.0)
                        * (36645.0 * a12 - 18873.0 * a18 + 51849.0 * a14
                            - 70119.0 * a16
                            + 23814.0 * a110
                            - 72268.0)
                        / a115,
                    -(1.0 / 20995200.0)
                        * (-8069175.0 * a12 - 9594126.0 * a18
                            + 12125430.0 * a14
                            - 601695.0 * a16
                            - 725355.0 * a110
                            + 1840725.0 * a112
                            - 13017775.0)
                        / a118,
                    (1.0 / 146966400.0)
                        * (-642384960.0 * a12 + 32359635.0 * a14
                            - 182419209.0 * a18
                            - 197892153.0 * a110
                            + 277231815.0 * a16
                            + 3090960.0 * a112
                            - 393014140.0
                            + 25981560.0 * a114)
                        / a121,
                    -(1.0 / 5290790400.0)
                        * (-125006633430.0 * a12 + 10913802705.0 * a18
                            - 77775185187.0 * a14
                            + 24578935920.0 * a16
                            + 1163129706.0 * a114
                            - 30234584880.0 * a110
                            - 15750047187.0 * a112
                            + 1570244130.0 * a116
                            - 48417507530.0)
                        / a124,
                    (1.0 / 1047576499200.0)
                        * (-99583081889940.0 * a12 + 14149065233457.0 * a18
                            - 115129891436076.0 * a14
                            - 34747982243667.0 * a16
                            - 5763893468058.0 * a114
                            - 7234652444283.0 * a110
                            - 17998039415961.0 * a112
                            + 686547606456.0 * a116
                            + 468534132000.0 * a118
                            - 28570914236480.0)
                        / a127,
                    -(1.0 / 246903552000.0)
                        * (-79763065718700.0 * a12
                            - 16013022932655.0 * a18
                            - 130493283697140.0 * a14
                            - 90942208981260.0 * a16
                            - 10200929842908.0 * a114
                            + 2512219926825.0 * a110
                            + 154901667060.0 * a120
                            - 11180851682679.0 * a112
                            - 2208929530140.0 * a116
                            + 354198515580.0 * a118
                            - 18298525303660.0)
                        / a130,
                    (1.0 / 5719767685632000.0)
                        * (-5618405733128588400.0 * a12
                            - 5882173102258447995.0 * a18
                            - 11729493633609952200.0 * a14
                            - 12103105143601672200.0 * a16
                            - 882178514895765087.0 * a114
                            - 911634009726294855.0 * a110
                            + 15298094789389440.0 * a120
                            - 419312125712563083.0 * a112
                            - 491823047868672312.0 * a116
                            - 75180895923051360.0 * a118
                            + 4782670826169600.0 * a122
                            - 1078217577276585200.0)
                        / a133,
                    -(1.0 / 39598391669760000.0)
                        * (-109241601084281219400.0 * a12
                            - 273755617744412105280.0 * a18
                            - 275531080361356997100.0 * a14
                            - 369234543309244628400.0 * a16
                            - 16629926930547479010.0 * a114
                            - 104248218659100772410.0 * a110
                            - 700142882381607420.0 * a120
                            - 21434621063709196035.0 * a112
                            - 16119995582024586972.0 * a116
                            - 6389833770645023520.0 * a118
                            + 178205363298894600.0 * a122
                            + 42471918559639800.0 * a124
                            - 18066934415886340600.0)
                        / a136,
                    (1.0 / 4118232733655040000.0)
                        * (-30058294461554586318000.0 * a12
                            - 140901213435715105015500.0 * a18
                            - 88530627902844856856400.0 * a14
                            - 144616754379461084721000.0 * a16
                            - 7905053018941478835435.0 * a114
                            - 81513990419502162468375.0 * a110
                            - 1152647132166760552560.0 * a120
                            - 27145640835037997266995.0 * a112
                            + 5500015182539136000.0 * a126
                            - 5904155800054927152621.0 * a116
                            - 3802193002950503796882.0 * a118
                            - 89958010436156592000.0 * a122
                            + 29001477679986355200.0 * a124
                            - 4375459586172885448000.0)
                        / a139,
                    -(1.0 / 4633011825361920000.0)
                        * (-85391848625055033024600.0 * a12
                            - 657624808266209998555770.0 * a18
                            - 286937627528945308701240.0 * a14
                            - 550253819731820080247280.0 * a16
                            - 76885144694468451642015.0 * a114
                            - 501113443972115359383570.0 * a110
                            - 8731660442937289348014.0 * a120
                            - 240951516125845943628639.0 * a112
                            + 48345076518821304480.0 * a126
                            - 28316641973836032929619.0 * a116
                            - 18073829022079601109456.0 * a118
                            - 2112862299988788781488.0 * a122
                            - 113834507284119452040.0 * a124
                            + 7524133830529617600.0 * a128
                            - 11114038570390896171200.0)
                        / a142,
                    (1.0 / 1323188177323364352000000.0)
                        * (-59322584268580541162995440000.0 * a12
                            - 691457486921045899947309280500.0 * a18
                            - 223595825592435993168270246000.0 * a14
                            - 490908387679243133903442594000.0 * a16
                            - 171020750418261506307267347625.0 * a114
                            - 647976462663409836197469228000.0 * a110
                            - 12206949275396521902047114736.0 * a120
                            - 406244910341218086344121040125.0 * a112
                            - 32295304702451043090432000.0 * a126
                            - 55282508132808557750323745025.0 * a116
                            - 22729370374617460059789308025.0 * a118
                            - 4703198025098354603242138200.0 * a122
                            - 934171582904834946460056000.0 * a124
                            + 19604322782640697627008000.0 * a128
                            + 2563130465125906185216000.0 * a130
                            - 6988238080689917196734008000.0)
                        / a145,
                ];
            } else {
                z1k = [
                    -(1.0 / 6.0) * (3.0 * a12 - 1.0) / a13,
                    (1.0 / 36.0) * (9.0 * a14 - 15.0 * a12 + 7.0) / a16,
                    -(1.0 / 3240.0)
                        * (540.0 * a16 - 1917.0 * a14 + 2205.0 * a12 - 830.0)
                        / a19,
                    (1.0 / 6480.0)
                        * (810.0 * a18 - 4698.0 * a16 + 9261.0 * a14
                            - 7710.0 * a12
                            + 2330.0)
                        / a112,
                    -(1.0 / 181440.0)
                        * (381360.0 * a12 - 591507.0 * a14 + 438165.0 * a16
                            - 150984.0 * a18
                            + 18144.0 * a110
                            - 95228.0)
                        / a115,
                    (1.0 / 3061800.0)
                        * (-11369925.0 * a12 + 21709485.0 * a14
                            - 21255885.0 * a16
                            + 11073267.0 * a18
                            - 2821230.0 * a110
                            + 255150.0 * a112
                            + 2409050.0)
                        / a118,
                    -(1.0 / 32659200.0)
                        * (213857700.0 * a12 - 485973810.0 * a14
                            + 593130195.0 * a16
                            - 415312191.0 * a18
                            + 163829628.0 * a110
                            - 32587920.0 * a112
                            + 2332800.0 * a114
                            - 39276200.0)
                        / a121,
                    (1.0 / 9797760.0)
                        * (-112887600.0 * a12 + 297838548.0 * a14
                            - 436001580.0 * a16
                            + 384287733.0 * a18
                            - 206353980.0 * a110
                            + 64657116.0 * a112
                            - 10429560.0 * a114
                            + 612360.0 * a116
                            + 18277000.0)
                        / a124,
                    -(1.0 / 6983843328000.0)
                        * (141307987590000.0 * a12 - 424805883301800.0 * a14
                            + 725840751913200.0 * a16
                            - 772229389384035.0 * a18
                            + 526083694083585.0 * a110
                            - 226625680966608.0 * a112
                            + 58331119524480.0 * a114
                            - 7846872019200.0 * a116
                            + 387991296000.0 * a118
                            - 20443712366000.0)
                        / a127,
                    (1.0 / 127309644000.0)
                        * (-4516095853500.0 * a12 + 15243943695750.0 * a14
                            - 29792346039225.0 * a16
                            + 37162390768965.0 * a18
                            - 30723150555405.0 * a110
                            + 16899523555761.0 * a112
                            - 6027938529228.0 * a114
                            + 1306793604780.0 * a116
                            - 149793206850.0 * a118
                            + 6365482200.0 * a120
                            + 590307086600.0)
                        / a130,
                    -(1.0 / 169474598092800.0)
                        * (10525154504264400.0 * a12
                            - 39423889481296320.0 * a14
                            + 86765073240966120.0 * a16
                            - 124208571575574486.0 * a18
                            + 120861414343735065.0 * a110
                            - 81043795131556257.0 * a112
                            + 37111541665177980.0 * a114
                            - 11221784750483424.0 * a116
                            + 2089073823123456.0 * a118
                            - 207552534174720.0 * a120
                            + 7703390822400.0 * a122
                            - 1254367500495200.0)
                        / a133,
                    (1.0 / 127673385840000.0)
                        * (-13865529287610000.0 * a12
                            + 57079362420580500.0 * a14
                            - 139720236603948000.0 * a16
                            + 225822732500400750.0 * a18
                            - 252922506605618850.0 * a110
                            + 200288672908829505.0 * a112
                            - 112243862958457230.0 * a114
                            + 43771264478848998.0 * a116
                            - 11426583533685840.0 * a118
                            + 1855110460918950.0 * a120
                            - 161971347951000.0 * a122
                            + 5319724410000.0 * a124
                            + 1518227841130000.0)
                        / a136,
                    -(1.0 / 61010855313408000.0)
                        * (11575128683790672000.0 * a12
                            - 51951107748591651600.0 * a14
                            + 140021157313351314000.0 * a16
                            - 252228200979738683700.0 * a18
                            + 319682393781675724800.0 * a110
                            - 292147632621771581655.0 * a112
                            + 193923227055129156585.0 * a114
                            - 92862978773912211384.0 * a116
                            + 31385770990121594592.0 * a118
                            - 7177250648648367360.0 * a120
                            + 1029090082244774400.0 * a122
                            - 79870195141632000.0 * a124
                            + 2346571358208000.0 * a126
                            - 1172073510107192000.0)
                        / a139,
                ];
                z2k = [
                    -(1.0 / 3240.0)
                        * (675.0 * a16 - 999.0 * a14 + 225.0 * a12 + 155.0)
                        / a19,
                    (1.0 / 2592.0)
                        * (891.0 * a18 - 2808.0 * a16
                            + 2403.0 * a14
                            + 12.0 * a12
                            - 505.0)
                        / a112,
                    -(1.0 / 816480.0)
                        * (360612.0 * a110 - 1847367.0 * a18 + 2973159.0 * a16
                            - 1409877.0 * a14
                            - 529305.0 * a12
                            + 450646.0)
                        / a115,
                    (1.0 / 2099520.0)
                        * (1086210.0 * a112 - 7963596.0 * a110
                            + 19603053.0 * a18
                            - 19160091.0 * a16
                            + 3104595.0 * a14
                            + 6145380.0 * a12
                            - 2817130.0)
                        / a118,
                    -(1.0 / 97977600.0)
                        * (56687040.0 * a114 - 551882160.0 * a112
                            + 1875163689.0 * a110
                            - 2848377393.0 * a18
                            + 1668615795.0 * a16
                            + 420389865.0 * a14
                            - 914516400.0 * a12
                            + 293913460.0)
                        / a121,
                    (1.0 / 1763596800.0)
                        * (1110514860.0 * a116 - 13665128328.0 * a114
                            + 60227454744.0 * a112
                            - 126426509100.0 * a110
                            + 126738043767.0 * a18
                            - 31045914900.0 * a16
                            - 50559078024.0 * a14
                            + 44844180360.0 * a12
                            - 11223491860.0)
                        / a124,
                    -(1.0 / 1745960832000.0)
                        * (1175752195200.0 * a118 - 17657759442960.0 * a116
                            + 96804669865860.0 * a114
                            - 262891435135923.0 * a112
                            + 376197022690335.0 * a110
                            - 237036075843105.0 * a18
                            - 54081767113605.0 * a16
                            + 185058317296710.0 * a14
                            - 110315414573100.0 * a12
                            + 22746727187800.0)
                        / a127,
                    (1.0 / 185177664000.0)
                        * (131757214680.0 * a120 - 2352880625760.0 * a118
                            + 15566372327340.0 * a116
                            - 52398360836136.0 * a114
                            + 98321547399957.0 * a112
                            - 97554313727955.0 * a110
                            + 26026236868275.0 * a18
                            + 49064508958500.0 * a16
                            - 59349145009140.0 * a14
                            + 27358014415200.0 * a12
                            - 4813736065880.0)
                        / a130,
                    -(1.0 / 4575814148505600.0)
                        * (3409988483865600.0 * a122
                            - 70961952197364480.0 * a120
                            + 553703220184315968.0 * a118
                            - 2241437046929907840.0 * a116
                            + 5244963547824369369.0 * a114
                            - 7095472642735153677.0 * a112
                            + 4486300046287926867.0 * a110
                            + 1337585653909522017.0 * a18
                            - 4915423617047143200.0 * a16
                            + 3978271574265348360.0 * a14
                            - 1513747067086254000.0 * a12
                            + 232808295847072400.0)
                        / a133,
                    (1.0 / 9899597917440000.0)
                        * (7676034955974000.0 * a124
                            - 183183385919954400.0 * a122
                            + 1655721130272604560.0 * a120
                            - 7880259888799266600.0 * a118
                            + 22235847209559202842.0 * a116
                            - 38230186388999393250.0 * a114
                            + 36747733201815133575.0 * a112
                            - 8636628178136012670.0 * a110
                            - 24623457047307646110.0 * a18
                            + 34546349053518271200.0 * a16
                            - 21767456292393873600.0 * a14
                            + 7098994800259362000.0 * a12
                            - 971150252248330000.0)
                        / a136,
                    -(1.0 / 1372744244551680000.0)
                        * (1101852389965824000.0 * a126
                            - 29762759848147430400.0 * a124
                            + 307134113531054227200.0 * a122
                            - 1688681289865388980320.0 * a120
                            + 5608250074097827426764.0 * a118
                            - 11746912410175672315053.0 * a116
                            + 15025452822410836851045.0 * a114
                            - 9019215315411973606935.0 * a112
                            - 4425894762564289432275.0 * a110
                            + 14384768308354215936450.0 * a18
                            - 13855276123879399209000.0 * a16
                            + 7263110850633113767200.0 * a14
                            - 2080655317262245542000.0 * a12
                            + 256579957380860372000.0)
                        / a139,
                ];
                z3k = [0.0; 11];
            }

            let mut s = z1k[0];
            let mut t = 1.0_f64;
            let mut k = 1;
            while (t.abs() > 1.0e-15) && (k < 13) {
                t = z1k[k] * zk[k];
                s += t;
                k += 1;
            }
            zeta1 = s;

            s = z2k[0];
            t = 1.0;
            k = 1;
            while (t.abs() > 1.0e-15) && (k < 11) {
                t = z2k[k] * zk[k];
                s += t;
                k += 1;
            }
            zeta2 = s;

            if iflag == 1 {
                s = z3k[0];
                t = 1.0;
                k = 1;
                while (t.abs() > 1.0e-15) && (k < 11) {
                    t = z3k[k] * zk[k];
                    s += t;
                    k += 1;
                }
                zeta3 = s;
            } else {
                zeta3 = 0.0;
            }
            if zeta2.abs() > (10.0 * zeta1).abs() {
                zeta2 = 0.0;
            }
            if zeta3.abs() > (5.0 * zeta2).abs() {
                zeta3 = 0.0;
            }
        } else {
            zeta2 = 0.0;
            zeta3 = 0.0;
        }
    }

    if zeta1.abs() < EPSS {
        zeta1 = 0.0;
    }
    if zeta2.abs() < EPSS {
        zeta2 = 0.0;
    }
    if zeta3.abs() < EPSS {
        zeta3 = 0.0;
    }

    (zeta1, zeta2, zeta3)
}

///   Evaluation of the cf for the ratio Ipnu(z)/Ipnu-1(z)
///   We use Lentz-Thompson algorithm.
fn fc(pnu: f64, z: f64) -> f64 {
    let mut m = 0;
    let mut b = 2.0 * pnu / z;
    let mut a = 1.0;
    let mut value = DWARF;
    let mut c0 = value;
    let mut d0 = 0.0;
    let mut delta = 0.0_f64;
    while ((delta - 1.0).abs() > EPSS) && (m < 500) {
        d0 = b + a * d0;
        if d0.abs() < DWARF {
            d0 = DWARF;
        }

        c0 = b + a / c0;
        if c0.abs() < DWARF {
            c0 = DWARF;
        }
        d0 = 1.0 / d0;

        delta = c0 * d0;
        value *= delta;
        m += 1;
        a = 1.0;
        b = 2.0 * (pnu + (m as f64)) / z;
    }

    value
}

fn nmax(mu: f64, x: f64, y: f64) -> i32 {
    let lneps = -36.0;
    let c = -mu + y - mu * y.ln() + lneps;
    let mut n = 10.0 + 2.0 * (-mu + (mu * mu + 4.0 * x * y).sqrt());
    let mut n1 = 0.0;
    while ((n - n1).abs() > 1.0) && (n > 0.0) {
        n1 = n;
        n = -((mu + n).ln() * mu - 2.0 * n + c)
            / ((n / (x * y)).ln() + (mu + n).ln());
    }
    if n < 0.0 {
        n = 0.0;
    }

    n as i32 + 1
}

fn factor(x: f64, n: i32) -> f64 {
    let mut factor = 1.0;
    for i in 1..=n {
        factor *= x / (i as f64);
    }

    factor
}

fn pol(fjkm: &[f64], d: usize, v: f64) -> f64 {
    let mut m = d;
    let mut s = fjkm[d];
    while m > 0 {
        m -= 1;
        s = s * v + fjkm[m];
    }

    s
}

const FJKM: [&[f64]; 152] = [
    &[0.50000000000000000000, 0.16666666666666666667],
    &[-0.12500000000000000000, 0.0, 0.20833333333333333333],
    &[
        0.62500000000000000000e-1,
        -0.54166666666666666667e-1,
        -0.31250000000000000000,
        0.28935185185185185185,
    ],
    &[
        -0.39062500000000000000e-1,
        0.83333333333333333333e-1,
        0.36631944444444444444,
        -0.83333333333333333333,
        0.42390046296296296296,
    ],
    &[
        0.27343750000000000000e-1,
        -0.10145089285714285714,
        -0.38281250000000000000,
        1.6061921296296296296,
        -1.7903645833333333333,
        0.64144483024691358025,
    ],
    &[
        -0.20507812500000000000e-1,
        0.11354166666666666667,
        0.36983072916666666667,
        -2.5763888888888888889,
        4.6821108217592592593,
        -3.5607638888888888889,
        0.99199861754115226337,
    ],
    &[
        0.16113281250000000000e-1,
        -0.12196955605158730159,
        -0.33297526041666666667,
        3.7101836350859788360,
        -9.7124626253858024691,
        11.698143727494855967,
        -6.8153513213734567901,
        1.5583573120284636488,
    ],
    &[
        -0.13092041015625000000e-1,
        0.12801339285714285714,
        0.27645252046130952381,
        -4.9738777281746031746,
        17.501935105096726190,
        -29.549479166666666667,
        26.907133829250257202,
        -12.754267939814814815,
        2.4771798425577632030,
    ],
    &[
        0.10910034179687500000e-1,
        -0.13242874035415539322,
        -0.20350690569196428571,
        6.3349384739790013228,
        -28.662114811111800044,
        63.367483364421434083,
        -79.925485618811085391,
        58.757341382271304870,
        -23.521455678429623200,
        3.9743166454849898231,
    ],
    &[
        -0.92735290527343750000e-2,
        0.13569064670138888889,
        0.11668911254144670021,
        -7.7625075954861111111,
        43.784562625335567773,
        -121.31910738398368607,
        198.20121981295421734,
        -200.43673900016432327,
        123.80342757950794259,
        -42.937783937667895519,
        6.4238224989853211488,
    ],
    &[
        0.80089569091796875000e-2,
        -0.13811212730852318255,
        -0.18036238655211433532e-1,
        9.2275853445797140866,
        -63.433189058657045718,
        213.60596888977804302,
        -432.96183396641609600,
        563.58282810729226948,
        -476.64858951490111802,
        254.12602383553942414,
        -77.797248335368675787,
        10.446593930548512362,
    ],
    &[
        -0.70078372955322265625e-2,
        0.13990718736965074856,
        -0.90802493534784075207e-1,
        -10.703046719402920575,
        88.139055705916160082,
        -352.55365414896970073,
        860.26747669490580229,
        -1381.3884907075539460,
        1497.5262381375579615,
        -1089.5695395426785795,
        511.32054028583482617,
        -140.15612725058882506,
        17.075450695147740963,
    ],
    &[
        0.61992406845092773438e-2,
        -0.14122658948520402530,
        0.20847570003254474385,
        12.163573370672875144,
        -118.39689039212288489,
        552.67487991757989471,
        -1587.5976792806460534,
        3052.8623335041016490,
        -4067.0706975337409188,
        3781.4312193762993828,
        -2415.5306966669781670,
        1012.7298787459738084,
        -251.37116645382357870,
        28.031797071713952493,
    ],
    &[
        -0.55350363254547119141e-2,
        0.14217921713372686883,
        -0.33386405994128191388,
        -13.585546133738642981,
        154.66282442015249412,
        -830.71069930083407076,
        2761.0291182562342601,
        -6219.8351157050681259,
        9888.1927799238643295,
        -11266.694472611704499,
        9175.5017581920039296,
        -5225.7429703251833306,
        1980.4053574007652015,
        -449.21570290311749301,
        46.189888661376921323,
    ],
    &[
        0.49815326929092407227e-2,
        -0.14284537645361756992,
        0.46603144339556328292,
        14.946922390174830635,
        -197.35300817536964730,
        1205.6532423474201960,
        -4572.9473467250314847,
        11865.183572985041892,
        -22026.784993357215819,
        29873.206689727991728,
        -29749.925047590507307,
        21561.076414337110462,
        -11081.438701085531999,
        3832.1051284526998677,
        -800.40791995840375064,
        76.356879052900946470,
    ],
    &[
        -0.45145140029489994049e-2,
        0.14328537051348750262,
        -0.60418804953366830816,
        -16.227111168548122706,
        246.84286168977111296,
        -1698.7528990888950203,
        7270.2387180078329370,
        -21434.839860240815288,
        45694.866035689911070,
        -72195.530107556687632,
        85409.022842807474925,
        -75563.234444869051891,
        49344.501227769590532,
        -23110.149147008741710,
        7349.7909384681412957,
        -1422.6485707704091767,
        126.58493346342458430,
    ],
    &[0.12500000000000000000, 0.0, -0.20833333333333333333],
    &[
        -0.62500000000000000000e-1,
        0.14583333333333333333,
        0.52083333333333333333,
        -0.65972222222222222222,
    ],
    &[
        0.46875000000000000000e-1,
        -0.25000000000000000000,
        -0.69791666666666666667,
        2.5000000000000000000,
        -1.6059027777777777778,
    ],
    &[
        -0.39062500000000000000e-1,
        0.34218750000000000000,
        0.72916666666666666667,
        -5.6712962962962962963,
        8.1640625000000000000,
        -3.5238233024691358025,
    ],
    &[
        0.34179687500000000000e-1,
        -0.42708333333333333333,
        -0.59798177083333333333,
        10.208333333333333333,
        -24.385308159722222222,
        22.482638888888888889,
        -7.3148750964506172840,
    ],
    &[
        -0.30761718750000000000e-1,
        0.50665457589285714286,
        0.29326171875000000000,
        -16.044663008432539683,
        56.156774450231481481,
        -82.372823832947530864,
        56.160933883101851852,
        -14.669405462319958848,
    ],
    &[
        0.28198242187500000000e-1,
        -0.58203125000000000000,
        0.19236328125000000000,
        23.032335069444444444,
        -110.33599717881944444,
        227.74508101851851852,
        -243.01300676761831276,
        131.66775173611111111,
        -28.734679254811814129,
    ],
    &[
        -0.26184082031250000000e-1,
        0.65396069723462301587,
        -0.86386369977678571429,
        -30.956497628348214286,
        194.54890778287588183,
        -527.74348743041776896,
        780.79702721113040123,
        -656.29672278886959877,
        295.22178492918917181,
        -55.334928257039108939,
    ],
    &[
        0.24547576904296875000e-1,
        -0.72297712053571428571,
        1.7246239871070498512,
        39.546341145833333333,
        -316.99617299397786458,
        1081.5824590773809524,
        -2074.4037171674994144,
        2398.8177766525205761,
        -1664.7533222350236974,
        640.37285196437757202,
        -105.19241070496638071,
    ],
    &[
        -0.23183822631835937500e-1,
        0.78948182770700165720,
        -2.7769457196432446677,
        -48.483511725054617511,
        486.26944746794524016,
        -2023.8687997794445650,
        4819.5203340475451309,
        -7173.8455521540386687,
        6815.5497547693867415,
        -4029.1488859965138299,
        1353.9765257894256969,
        -197.95866455017786514,
    ],
    &[
        0.22024631500244140625e-1,
        -0.85378706190321180556,
        4.0223697649378354858,
        57.408728524667245370,
        -711.17788174487525874,
        3529.7027186963924024,
        -10126.360073656459287,
        18593.571833843032106,
        -22636.974191769862737,
        18256.758136740546277,
        -9401.7390963482140877,
        2805.1309521324368189,
        -369.51173382133760772,
    ],
    &[
        -0.21023511886596679688e-1,
        0.91614229084450603921,
        -5.4618897365663646792,
        -65.927090730899790043,
        1000.5846459432155138,
        -5819.5104958244309663,
        19669.512303383909626,
        -43248.109984766956219,
        64686.833219925562541,
        -66644.721592787005810,
        46700.224876576248105,
        -21305.241783200054197,
        5716.0564388863858560,
        -685.13376643364457482,
    ],
    &[
        0.20147532224655151367e-1,
        -0.97675104265089158888,
        7.0960977732520869186,
        73.612404446931816840,
        -1363.2529599857205103,
        9163.5749605651064785,
        -35864.832027517679572,
        92376.947946883135629,
        -164834.83784046136147,
        208040.74214864238663,
        -185789.85543054601897,
        115101.05980498683057,
        -47134.497747406170101,
        11488.455724263405622,
        -1263.2564781342309572,
    ],
    &[
        -0.19372627139091491699e-1,
        1.0357822247248985273,
        -8.9252867409545555669,
        -80.010760848208515926,
        1807.7010112763722305,
        -13886.239779926939526,
        62074.025784691064765,
        -184145.36258906485362,
        383582.03973738516554,
        -576038.14802241650407,
        628833.57177487054480,
        -495573.34466362548232,
        275136.26556037481059,
        -102207.94135045741701,
        22823.518594320784899,
        -2318.1664194368241801,
    ],
    &[
        0.18680747598409652710e-1,
        -1.0933780262877533843,
        10.949523517716476548,
        84.643531757174997082,
        -2342.0651134541361650,
        20369.770814493525849,
        -102837.36670370684414,
        346648.70123159565251,
        -828961.75261733863230,
        1449716.2069081751493,
        -1879301.2063630471735,
        1807331.1927210534022,
        -1274496.0479553760313,
        641015.88735632964269,
        -217895.35698164739918,
        44894.191761385746325,
        -4236.6734164587391641,
    ],
    &[
        -0.18058056011795997620e-1,
        1.1496596058234620572,
        -13.168702574894088581,
        -87.009904621222723801,
        2973.9704746271364310,
        -29057.863221639334277,
        164134.77797509230729,
        -621775.71008157787326,
        1684395.4811437516714,
        -3374103.7733925392782,
        5084254.0101088150007,
        -5797111.7426650438469,
        4981685.5893221501493,
        -3178510.0651440248351,
        1461172.7927229457558,
        -457795.06653758949338,
        87552.178162658627517,
        -7715.5318619797584859,
    ],
    &[
        0.70312500000000000000e-1,
        0.0,
        -0.40104166666666666667,
        0.0,
        0.33420138888888888889,
    ],
    &[
        -0.10546875000000000000,
        0.15234375000000000000,
        1.4036458333333333333,
        -1.6710069444444444444,
        -1.8381076388888888889,
        2.0609085648148148148,
    ],
    &[
        0.13183593750000000000,
        -0.42187500000000000000,
        -2.8623046875000000000,
        8.0208333333333333333,
        1.0777994791666666667,
        -14.036458333333333333,
        8.0904586226851851852,
    ],
    &[
        -0.15380859375000000000,
        0.79892578125000000000,
        4.5903320312500000000,
        -22.751985677083333333,
        14.934624565972222222,
        42.526662567515432099,
        -65.691460503472222222,
        25.746658387988683128,
    ],
    &[
        0.17303466796875000000,
        -1.2773437500000000000,
        -6.3615722656250000000,
        50.011935763888888889,
        -73.559339735243055556,
        -70.026331018518518519,
        271.34066056616512346,
        -242.71375868055555556,
        72.412718470695087449,
    ],
    &[
        -0.19033813476562500000,
        1.8524126325334821429,
        7.9220947265625000000,
        -94.174715169270833333,
        221.09830050998263889,
        13.578712293836805556,
        -765.03722541714891975,
        1204.5108913845486111,
        -777.22725008740837191,
        187.66711848589945559,
    ],
    &[
        0.20619964599609375000,
        -2.5202636718750000000,
        -8.9979495239257812500,
        159.65856119791666667,
        -527.02200527615017361,
        337.21907552083333333,
        1618.7873626708984375,
        -4211.0382245852623457,
        4434.1363497656886306,
        -2259.2768162856867284,
        458.84770992088928362,
    ],
    &[
        -0.22092819213867187500,
        3.2776113237653459821,
        9.3003209795270647321,
        -250.77115683984504175,
        1087.8174260457356771,
        -1404.3028911260910976,
        -2563.9444452795962738,
        11622.495969086321293,
        -17934.344163614699543,
        14479.313178892270800,
        -6122.6397406024697386,
        1074.0188194633057124,
    ],
    &[
        0.23473620414733886719,
        -4.1216033935546875000,
        -8.5290844099862234933,
        371.57630452473958333,
        -2030.4431650042155432,
        3928.2498148600260417,
        2472.6031768756442600,
        -26784.706192883150077,
        57707.467479758203765,
        -65779.375284558624561,
        43428.755429000357378,
        -15731.921483001918296,
        2430.2098720207426574,
    ],
    &[
        -0.24777710437774658203,
        5.0497246547178788619,
        6.3753494648706345331,
        -525.77763195628211612,
        3515.3901490500364354,
        -9098.9465854134383025,
        1501.4499341175879961,
        52968.402569427411743,
        -156962.17039551999834,
        237710.55444046526561,
        -217889.76091367761240,
        122183.02599420558225,
        -38765.366765003690558,
        5352.0219072834891857,
    ],
    &[
        0.26016595959663391113,
        -6.0597300529479980469,
        -2.5233336282830660035,
        716.61609867398701017,
        -5739.3531518108567233,
        18710.056678357368214,
        -15227.052778022872591,
        -90410.693429278463984,
        374173.78606362103489,
        -726678.85908967426430,
        867690.63613487545936,
        -669330.97296360068003,
        326923.43164094028666,
        -92347.975136788220980,
        11528.702830431737704,
    ],
    &[
        -0.27199168503284454346,
        7.1495960926884537810,
        -3.3482232633271774688,
        -946.77886875050071077,
        8937.5223755513871158,
        -35337.290730230231910,
        49459.110954680523755,
        130172.10642681313787,
        -799759.43622037315810,
        1951258.3781149473349,
        -2917568.5809228727915,
        2902364.7717490216619,
        -1939009.9106363828308,
        839993.29007310418975,
        -213947.07574365748020,
        24380.364047003816130,
    ],
    &[
        0.28332467190921306610,
        -8.3174842023230218268,
        11.564968830087189329,
        1218.3176746274854345,
        -13385.506466376132701,
        62540.301444639907312,
        -122382.91365675340023,
        -143089.29056273054521,
        1554707.5273250397863,
        -4718379.1202435790866,
        8605382.3882172381886,
        -10599895.885891960945,
        9077838.6492033673420,
        -5358306.8036736424269,
        2087192.8797093735160,
        -484205.51887813298356,
        50761.444989589644273,
    ],
    &[
        -0.29422177467495203018,
        9.5617123863640260863,
        -22.456083202924761739,
        -1532.5752010328979216,
        19400.899387993296528,
        -105087.87893355958488,
        262967.92353732330762,
        61613.709412718183861,
        -2770349.5737553264845,
        10454840.708341905254,
        -22841191.197018135498,
        33883152.513403664925,
        -35702419.891311431439,
        26908321.887260639130,
        -14241918.449935481857,
        5042187.1772326832703,
        -1074259.7908211056482,
        104287.72699173731366,
    ],
    &[
        0.30472969519905745983,
        -10.880732823367957230,
        36.353598566849979929,
        1890.1183163422473995,
        -27344.503966626558254,
        169206.00701162634518,
        -515265.57929420780237,
        248217.18376755761876,
        4532205.9690912962460,
        -21492951.405026820809,
        55557247.108151935296,
        -97285892.160889723465,
        122607925.76741209887,
        -113234217.41453912066,
        76312885.330872101297,
        -36634431.269047918012,
        11891540.519643040965,
        -2342835.9225964451203,
        211794.47349942484210,
    ],
    &[
        0.73242187500000000000e-1,
        0.0,
        -0.89121093750000000000,
        0.0,
        1.8464626736111111111,
        0.0,
        -1.0258125964506172840,
    ],
    &[
        -0.18310546875000000000,
        0.23193359375000000000,
        4.0104492187500000000,
        -4.6045898437500000000,
        -12.002007378472222222,
        13.232982494212962963,
        8.7194070698302469136,
        -9.4032821341306584362,
    ],
    &[
        0.32043457031250000000,
        -0.87890625000000000000,
        -10.464160156250000000,
        26.736328125000000000,
        29.225667317708333333,
        -103.40190972222222222,
        17.131070360725308642,
        92.323133680555555556,
        -50.991434481899434156,
    ],
    &[
        -0.48065185546875000000,
        2.1109008789062500000,
        21.025415039062500000,
        -89.876334092881944444,
        -15.284450954861111111,
        411.04911024305555556,
        -389.32152566792052469,
        -293.92095419801311728,
        567.72315884813850309,
        -213.02470796338270176,
    ],
    &[
        0.66089630126953125000,
        -4.0893554687500000000,
        -36.043276468912760417,
        229.67792968750000000,
        -150.64704827202690972,
        -1115.0236545138888889,
        2175.9328758333936150,
        -176.78170412165637860,
        -2817.5643744553721654,
        2651.5545930587705761,
        -757.67687847693870206,
    ],
    &[
        -0.85916519165039062500,
        6.9690023149762834821,
        55.378133392333984375,
        -495.03058466109018477,
        733.55991770426432292,
        2262.8678469622576678,
        -7898.5588740407684703,
        5695.1407000317985629,
        7718.7923309734328784,
        -16089.784052072184022,
        10424.896645958040967,
        -2413.3719004256171872,
    ],
    &[
        1.0739564895629882812,
        -10.898971557617187500,
        -78.354169082641601562,
        948.65802978515625000,
        -2219.4645106141832140,
        -3394.0875061035156250,
        22215.581371235788604,
        -30531.682191548916538,
        -6945.0954169277301051,
        63170.236743335697713,
        -72433.473359744190134,
        36368.490166893057699,
        -7090.9841426397698721,
    ],
    &[
        -1.3040900230407714844,
        16.023568312327067057,
        103.72332413083031064,
        -1667.3156506674630301,
        5406.6561977448034539,
        2872.9832533515445770,
        -52104.157882492630570,
        110439.44251210509668,
        -46659.137282813036883,
        -173333.53977304078369,
        339551.86235951279889,
        -281181.85781749484440,
        116143.52270798282713,
        -19586.901426503340492,
    ],
    &[
        1.5486069023609161377,
        -22.482862472534179688,
        -129.63729095714432853,
        2741.6385669817243304,
        -11510.430102675971531,
        3157.6331450774177672,
        105738.58606273177860,
        -320687.78222286271460,
        312110.04133755429291,
        306706.99777237116391,
        -1199602.2626751819183,
        1489876.7581807900958,
        -983301.03812460934208,
        346445.22525468589947,
        -51524.795648340968513,
    ],
    &[
        -1.8067080527544021606,
        30.413155585075869705,
        153.62532423010894230,
        -4275.6818557748761872,
        22280.234407631843178,
        -22294.360392132099974,
        -188892.07658652729984,
        800265.57729686724177,
        -1211192.8548070343556,
        -77428.408184713489979,
        3343683.8379703140094,
        -6075462.1554119391419,
        5742939.8025630234344,
        -3178262.5289756645302,
        978732.98065558879521,
        -130276.59845140693203,
    ],
    &[
        2.0777142606675624847,
        -39.947360754013061523,
        -172.57638879468381540,
        6386.1869555628867376,
        -40128.233950133856041,
        67957.947814703914434,
        297268.90885718166849,
        -1779345.5277624794845,
        3703482.9515239098067,
        -1986101.2546910898185,
        -7335848.9571808003709,
        20236466.148311260729,
        -26009069.048248407006,
        20168378.697199375155,
        -9655403.7938215681211,
        2644939.5099481697170,
        -318773.08892039496616,
    ],
    &[
        -2.3610389325767755508,
        51.215318048867833364,
        182.72462206625770697,
        -9201.6026100350086393,
        68268.256270370096570,
        -162141.74207057405274,
        -402709.29424480088095,
        3603004.6646962551842,
        -9740822.7436915944519,
        10107345.074822039354,
        11498843.003383757375,
        -56841651.345428293012,
        97219891.251414595951,
        -99519441.572391483730,
        65942266.170395132237,
        -27893470.527717198286,
        6888375.1431181415309,
        -758786.31484749532876,
    ],
    &[
        2.6561687991488724947,
        -64.344060508074698510,
        -179.63738093291836674,
        12860.884276726402663,
        -110864.10416322604021,
        338921.66960863282529,
        430704.46372470858983,
        -6738355.6173354573243,
        22959038.837731227338,
        -34599901.818598601926,
        -5491093.1482636375735,
        136348100.13563343194,
        -311391327.48073317359,
        406747852.87490380879,
        -350465400.65634558429,
        203621539.64411279745,
        -77285292.825523293342,
        17387623.032021543609,
        -1764164.5657772609975,
    ],
    &[
        -2.9626498144352808595,
        79.458041370067243966,
        158.20533868869907341,
        -17512.092404496780068,
        173186.23779115767550,
        -648825.90238130558651,
        -226118.57798798103100,
        11744385.639221317992,
        -49655262.440949257658,
        97992370.806143206674,
        -45563229.252833811893,
        -276725901.55879139753,
        874903955.44068001049,
        -1431639430.0141678479,
        1544324559.7308983592,
        -1156507831.0309397511,
        599846625.33396072076,
        -206711172.70469868149,
        42729154.354849580701,
        -4019188.6691200667599,
    ],
    &[
        0.11215209960937500000,
        0.0,
        -2.3640869140625000000,
        0.0,
        8.7891235351562500000,
        0.0,
        -11.207002616222993827,
        0.0,
        4.6695844234262474280,
    ],
    &[
        -0.39253234863281250000,
        0.46730041503906250000,
        13.002478027343750000,
        -14.578535970052083333,
        -65.918426513671875000,
        71.777842203776041667,
        106.46652485411844136,
        -113.93785993160043724,
        -53.700220869401845422,
        56.813277151686010374,
    ],
    &[
        0.88319778442382812500,
        -2.2430419921875000000,
        -40.888863372802734375,
        99.291650390625000000,
        222.92270863850911458,
        -632.81689453125000000,
        -205.55324667471426505,
        1232.7702877845293210,
        -339.12856875133121946,
        -728.45517005449459877,
        393.21792165601858550,
    ],
    &[
        -1.6191959381103515625,
        6.5174388885498046875,
        97.292139053344726562,
        -384.72013047112358941,
        -422.46132278442382812,
        2925.8162224946198640,
        -1437.2810672241964458,
        -5929.6163575631600839,
        6678.9649706318545243,
        1992.7516382310725152,
        -5560.5995012212682653,
        2034.9551693024277015,
    ],
    &[
        2.6311933994293212891,
        -14.813423156738281250,
        -194.76567316055297852,
        1116.2957621256510417,
        214.74175742997063531,
        -9500.2007904052734375,
        12733.852428636433166,
        15619.117721871584041,
        -43856.442195416477973,
        16041.189890575016477,
        30538.376827393703173,
        -31457.433732481486840,
        8757.4502329231489542,
    ],
    &[
        -3.9467900991439819336,
        28.973631262779235840,
        345.96240515708923340,
        -2698.2640026051657540,
        1749.1663194396760729,
        24230.291604531833104,
        -57186.682706525590685,
        -12268.269917924904529,
        179642.68522044022878,
        -184075.59647969633791,
        -55836.464134952713487,
        219854.46366368396092,
        -146898.32628401899970,
        33116.007471226346158,
    ],
    &[
        5.5912859737873077393,
        -51.149418354034423828,
        -562.32073248028755188,
        5740.3790382385253906,
        -8505.8885195685227712,
        -51098.161945523156060,
        189688.56368664133696,
        -98986.676113505422333,
        -524313.33320720157996,
        1006412.2572891519230,
        -338656.53584266578056,
        -879242.30314162838225,
        1184856.1356540792633,
        -599009.59593194338847,
        113723.03789882673771,
    ],
    &[
        -7.5881738215684890747,
        83.791322236259778341,
        852.65149199664592743,
        -11106.114063047180100,
        25906.550896742895797,
        90628.038560826472504,
        -518627.00526554010007,
        625235.13813439073187,
        1093177.6471805288836,
        -3890056.2699064931220,
        3443257.5304835279133,
        1688397.8063561002636,
        -6072278.7538110165925,
        5368522.3053911687123,
        -2206353.9977704553128,
        362368.26917284610367,
    ],
    &[
        9.9594781408086419106,
        -129.64064240455627441,
        -1221.6473410353064537,
        19961.318612462793078,
        -64135.377206358956439,
        -132491.34865988838862,
        1231383.3446542421759,
        -2354589.9435372119185,
        -1264272.3547066582332,
        11829877.236705665039,
        -17640228.849921599961,
        3679036.5985685346058,
        21328290.463957701877,
        -31765842.068457922539,
        21552604.862053478433,
        -7505720.5013225646891,
        1087467.9477654193166,
    ],
    &[
        -12.725999846588820219,
        191.72191139924424616,
        1668.3300609576205413,
        -33822.376037367149478,
        139670.53397437636223,
        142413.44221576977052,
        -2615894.9488370680107,
        7028008.7411020993735,
        -1692308.6869349767646,
        -29470781.812749969812,
        66963160.307047821471,
        -48089009.180540108686,
        -47220345.652008289171,
        138176622.72569342840,
        -141477318.49446414033,
        78991076.066288083382,
        -23950277.790642797164,
        3106959.7999206284827,
    ],
    &[
        15.907499808236025274,
        -273.33609867841005325,
        -2184.4474298407246048,
        54603.014533953543693,
        -277734.45331034886641,
        -41109.662796960089573,
        5064705.8054347585059,
        -18090940.629099192262,
        15917528.891696545807,
        60220437.637860917599,
        -208561974.26419501420,
        250941409.40257928779,
        12084648.536915454989,
        -458976381.31741616556,
        699975914.26074700776,
        -563757375.34166870146,
        269426949.85344351478,
        -72497863.184361287773,
        8519623.3256649401434,
    ],
    &[
        -19.522840673744212836,
        378.05442703043402114,
        2752.8286152184838570,
        -84659.009316768248795,
        515277.57789757005885,
        -327436.12858763123432,
        -9039645.4021864355266,
        41795541.145581633576,
        -61523469.344794095368,
        -95072522.336003533273,
        557312492.76991978150,
        -963898631.17215744429,
        480963077.05278739701,
        1131925258.2353560419,
        -2762861092.1224474487,
        3066633312.6192085228,
        -2065683582.7903266052,
        866733914.71761334168,
        -209952808.47963646972,
        22561861.306890567863,
    ],
    &[
        23.590099147440923844,
        -509.71270865877158940,
        -3345.7051051560481552,
        126830.08496875773140,
        -904536.17796320887184,
        1241459.9239568200231,
        14964746.535519588726,
        -88697323.877097900818,
        182496348.04247934870,
        82548357.373675412652,
        -1305701152.6740455738,
        3075905875.9322221769,
        -2915784132.1314453282,
        -1631935529.3260648957,
        8923557290.4467172403,
        -13522339111.332256776,
        12138202400.912393639,
        -7081644626.2422883477,
        2655510812.9195669082,
        -585530475.83660861349,
        57986597.253985419492,
    ],
    &[
        0.22710800170898437500,
        0.0,
        -7.3687943594796316964,
        0.0,
        42.534998745388454861,
        0.0,
        -91.818241543240017361,
        0.0,
        84.636217674600734632,
        0.0,
        -28.212072558200244877,
    ],
    &[
        -1.0219860076904296875,
        1.1733913421630859375,
        47.897163336617606027,
        -52.809692909604027158,
        -361.54748933580186632,
        389.90415516606083623,
        964.09153620402018229,
        -1025.3036972328468605,
        -1057.9527209325091829,
        1114.3768660489096727,
        409.07505209390355072,
        -427.88310046603704731,
    ],
    &[
        2.8104615211486816406,
        -6.8132400512695312500,
        -175.59265831538609096,
        412.65248413085937500,
        1483.6983865298922100,
        -3828.1498870849609375,
        -3429.1824372044316045,
        12120.007883707682292,
        557.04779563126740632,
        -15403.791616777333703,
        5099.3321148946942616,
        6770.8974139680587706,
        -3602.9167662868229396,
    ],
    &[
        -6.0893332958221435547,
        23.218954324722290039,
        480.30594648633684431,
        -1808.5316684886387416,
        -3878.5356589824434311,
        19896.567837257637549,
        -442.43697992960611979,
        -68889.990792852959025,
        51994.291933598341765,
        82310.686911069807202,
        -107791.27622674358562,
        -11421.030640241631355,
        61775.622629784098705,
        -22242.802900370862046,
    ],
    &[
        11.417499929666519165,
        -60.543208122253417969,
        -1092.4312783437115805,
        5864.8337360927036830,
        6502.6598836863797808,
        -73117.673620733634505,
        62464.986490075687042,
        248344.19895160816334,
        -446788.55343178424816,
        -141685.28980603760980,
        805685.00855625677338,
        -411181.55351158250234,
        -353321.84981767407842,
        410732.51135669781511,
        -112357.72180097660343,
    ],
    &[
        -19.409749880433082581,
        133.60695303976535797,
        2184.1347855359315872,
        -15685.927751365060709,
        -3330.0494749048683378,
        213065.39140775687165,
        -371035.73548135295431,
        -595658.10351999312306,
        2217706.3121620208025,
        -928359.76150830112939,
        -3462387.4565158783367,
        4492508.5831562094441,
        -105953.60990151918538,
        -3174045.4228780972346,
        2222890.1148558130258,
        -492012.66653936007240,
    ],
    &[
        30.732103977352380753,
        -262.68730902671813965,
        -3966.7030987024307251,
        36616.605837684018271,
        -23288.020921949948583,
        -522073.19210540329968,
        1445873.6105443313563,
        729826.91993359621660,
        -8027322.7404775209228,
        9022069.9722413070898,
        8528377.7669713558429,
        -26111911.326974580072,
        15072848.600502055062,
        11547035.062352154444,
        -20141460.694713124158,
        10381853.494410238157,
        -1934247.3992962518385,
    ],
    &[
        -46.098155966028571129,
        474.28179555572569370,
        6683.6986173737261977,
        -77185.928108340199369,
        113831.12007369411134,
        1111273.3131467255535,
        -4487654.8822124313622,
        1363280.0193113290821,
        22934079.022569534587,
        -45086888.891430235790,
        -1310272.8912292084566,
        103829405.25391139096,
        -124354846.65650933807,
        7129538.3762123397968,
        107358912.41929520843,
        -104902766.60439072992,
        43358616.238781106380,
        -6986431.7916780392488,
    ],
    &[
        66.266099201166070998,
        -801.85159036517143250,
        -10599.673972529735017,
        150238.26124378282197,
        -349014.85897753891605,
        -2089251.7495501184712,
        11932847.810754978267,
        -12233248.989355019522,
        -52996346.810335384350,
        167552806.49381000405,
        -104151238.67453537869,
        -295472139.38679802840,
        638921130.05027917750,
        -364575119.55069248400,
        -321938848.50186568760,
        670099675.87405621186,
        -477068925.07477830810,
        165792634.22539301473,
        -23563863.859185525714,
    ],
    &[
        -92.036248890508431941,
        1286.5459820115674202,
        15984.246642014751810,
        -274251.94134559012498,
        875242.48789234256927,
        3479369.5294348938478,
        -28243123.382389417092,
        48978753.833933594038,
        96403146.255130900766,
        -511553591.82361285211,
        629980523.20384634815,
        530948403.41019250637,
        -2455930387.0192782105,
        2650615136.5125114389,
        -13787083.107153494438,
        -2934135354.5042859293,
        3427343175.1586684272,
        -1959752975.9949664819,
        590135160.40330437780,
        -75099321.778257988527,
    ],
    &[
        124.24893600218638312,
        -1977.9098049255553633,
        -23091.371137548782696,
        474842.97883978903678,
        -1940160.8026042800714,
        -5051698.1764753913085,
        60910728.961121054906,
        -150451158.23316204445,
        -115862597.99920025974,
        1340568362.9608932867,
        -2534626023.8559564861,
        57223508.996001180928,
        7462542511.1368897210,
        -12803010436.906857334,
        6529418551.9917203148,
        8333347633.6309463361,
        -17879701023.370781023,
        15384544080.383156452,
        -7429525037.6309918827,
        1979364564.1715841600,
        -228201703.20311712289,
    ],
    &[
        -163.78268836651841411,
        2934.5753597465244149,
        32133.674298098814287,
        -786448.50173515116761,
        3940574.6676366506081,
        6023482.0215492578926,
        -121576300.80985078356,
        396478314.88388992406,
        -28867608.104982563453,
        -3079357130.2226958330,
        8249384124.8426910359,
        -5275292901.0490022350,
        -17864885603.458945318,
        48480263067.875486448,
        -46292296797.993831733,
        -6808810264.3074322272,
        70205428541.430035549,
        -89866265315.798467190,
        62728346454.981427111,
        -26379975109.497266807,
        6313975478.5070403854,
        -665761463.93251599995,
    ],
    &[
        0.57250142097473144531,
        0.0,
        -26.491430486951555525,
        0.0,
        218.19051174421159048,
        0.0,
        -699.57962737613254123,
        0.0,
        1059.9904525279998779,
        0.0,
        -765.25246814118164230,
        0.0,
        212.57013003921712286,
    ],
    &[
        -3.1487578153610229492,
        3.5304254293441772461,
        198.68572865213666643,
        -216.34668231010437012,
        -2072.8098615700101096,
        2218.2702027328178365,
        8045.1657148255242242,
        -8511.5521330762792517,
        -14309.871109127998352,
        15016.531410813331604,
        11861.413256188315456,
        -12371.581568282436550,
        -3719.9772756862996501,
        3861.6906957124443986,
    ],
    &[
        10.233462899923324585,
        -24.045059680938720703,
        -830.55504153881754194,
        1907.3829950605119978,
        9817.0755057463759468,
        -24000.956291863274953,
        -37145.398656393453558,
        109134.42187067667643,
        44836.131085879493643,
        -222597.99503087997437,
        21083.102663859050460,
        208148.67133440140671,
        -75945.993209761297570,
        -72698.984473412256018,
        38306.908850817252349,
    ],
    &[
        -25.583657249808311462,
        94.002347901463508606,
        2561.4464542163269860,
        -9323.5958246609994343,
        -30925.007683879997995,
        137806.75057795568307,
        66832.114908046586804,
        -695211.42408942898710,
        297044.24306208789349,
        1456689.0310313083630,
        -1349408.4412036920976,
        -1160751.9107670259288,
        1778986.1326650806502,
        2732.4118798791034334,
        -771855.42780552482418,
        274755.25810395356345,
    ],
    &[
        54.365271655842661858,
        -276.51818633079528809,
        -6505.3076391667127609,
        33393.909001989024026,
        70377.367684318314469,
        -559956.42247611134141,
        214189.08434628603635,
        2932546.1434609688359,
        -3873550.9334950489425,
        -5169809.5626455059758,
        12515387.720161636405,
        -485287.10103841189331,
        -14696506.049911874334,
        8973612.6112505443054,
        4358025.7113579717181,
        -5899263.9630258568617,
        1593568.9458830170786,
    ],
    &[
        -103.29401614610105753,
        679.49573871586471796,
        14406.592158034443855,
        -97833.485413427407644,
        -109874.73447139263153,
        1806898.4781330669971,
        -2228617.5688649368428,
        -9039218.0698707036945,
        22913970.357558080752,
        6014580.7747564135778,
        -67365551.082731008652,
        49347945.008100392566,
        61291772.218955972273,
        -101851641.61990357673,
        18812228.438435360796,
        48878028.306514326695,
        -36319210.680616361669,
        7931540.8655372143613,
    ],
    &[
        180.76452825567685068,
        -1472.1516226977109909,
        -28789.438034501904622,
        248412.49281514968191,
        57720.374439080618322,
        -4919773.7285477433167,
        10556839.574755387407,
        20546468.524262875642,
        -95782021.413901062575,
        47859753.794019524423,
        248947938.76422519634,
        -397302112.77505450021,
        -60373613.593196943601,
        619895830.67946774788,
        -460542977.57691540068,
        -133881283.62288371220,
        360816116.57296145253,
        -191228273.50596204944,
        35131056.264643928958,
    ],
    &[
        -296.97029642004054040,
        2903.8678610353963450,
        53085.574017544759304,
        -566162.21564224131681,
        351303.05079310148650,
        11717363.296337798053,
        -37248885.401731669600,
        -29556393.543845627395,
        317963019.15514055453,
        -391675101.18705789558,
        -632571201.89067213266,
        2001265322.6458411313,
        -1008064787.2696644192,
        -2363398838.2753953344,
        3743079200.5912006268,
        -1091651847.4629542050,
        -1902208028.0358040356,
        2133928476.4409109596,
        -893289789.98112876745,
        141870657.61208999896,
    ],
    &[
        464.01608815631334437,
        -5325.3068480961956084,
        -91725.505981153156193,
        1185316.6466349283157,
        -1734253.4162956622921,
        -24965643.657687719930,
        109865301.06964371257,
        -7842536.5990090553082,
        -882004248.16533042144,
        1812569161.1229405097,
        796190115.17482420785,
        -7547640676.2891254543,
        8606208381.3162846761,
        4634326771.3840251843,
        -19504767652.151161478,
        15458811432.485826518,
        3233232293.8508888375,
        -14292761227.388542723,
        10872211035.524945516,
        -3794154076.5815443275,
        531367092.46942384383,
    ],
    &[
        -696.02413223447001656,
        9211.7329484450783639,
        150176.80785295595602,
        -2316795.9016969799608,
        5353301.7267099139240,
        48240128.320645392607,
        -285088568.66606943443,
        236539658.57255855849,
        2091063546.0424140229,
        -6478408107.9479218188,
        2114947617.4171696310,
        22484222108.370436724,
        -43504182331.312579142,
        9073306866.7430378915,
        72378013713.663446159,
        -104755002543.75143509,
        35288894490.708636111,
        58426452630.062379587,
        -83650899080.625595930,
        49569134901.994243944,
        -14909719423.420583328,
        1869289195.4875346262,
    ],
    &[
        1009.2349917399815240,
        -15188.489623096204014,
        -234912.74468029359442,
        4278033.8338821994157,
        -13570280.995019535225,
        -85095459.392767211454,
        669951675.09023006543,
        -1041016026.5703218480,
        -4241350570.5261899700,
        19536358548.670235798,
        -19158789931.456781880,
        -52588961930.566950783,
        168223251386.27505861,
        -131543443714.17931735,
        -183278984759.32788630,
        500731039137.68584765,
        -395175402811.80868032,
        -83866621459.668331313,
        443918936157.05017610,
        -420471377306.84165961,
        207052924562.97132855,
        -54907932888.507130529,
        6236056730.2635893277,
    ],
    &[
        1.7277275025844573975,
        0.0,
        -108.09091978839465550,
        0.0,
        1200.9029132163524628,
        0.0,
        -5305.6469786134031084,
        0.0,
        11655.393336864533248,
        0.0,
        -13586.550006434137439,
        0.0,
        8061.7221817373093845,
        0.0,
        -1919.4576623184069963,
    ],
    &[
        -11.230228766798973083,
        12.382047101855278015,
        918.77281820135457175,
        -990.83343139361767542,
        -12609.480588771700859,
        13410.082530915935834,
        66320.587232667538855,
        -69857.685218409807594,
        -169003.20338453573209,
        176773.46560911208759,
        224178.07510616326774,
        -233235.77511045269270,
        -149141.86036214022361,
        154516.34181663176320,
        39348.882077527343424,
        -40628.520519072948089,
    ],
    &[
        42.113357875496149063,
        -96.752740144729614258,
        -4309.3875268953187125,
        9728.1827809555189950,
        67131.493914289162272,
        -158519.18454455852509,
        -361549.21741861661275,
        965627.75010763936573,
        791368.90269480066167,
        -2797294.4008474879795,
        -473067.29978352049251,
        4157484.3019688460562,
        -742925.21875958646140,
        -3063454.4290601775661,
        1186992.6183777028865,
        886789.43999110403230,
        -463948.91246287829107,
    ],
    &[
        -119.32118064723908901,
        426.72709654457867146,
        14774.672950860112906,
        -52455.440737222809167,
        -242280.57068559899926,
        994102.67395229967167,
        1032233.4449197463691,
        -6741567.1085603777512,
        646495.83215296654790,
        20680668.518424851831,
        -13425393.794712164658,
        -29950004.715897617246,
        32133326.488920312047,
        16877471.905929211282,
        -30879035.210339582752,
        1961435.1350279426026,
        10741165.112229910651,
        -3791244.3495002070744,
    ],
    &[
        283.38780403719283640,
        -1397.7315495908260345,
        -41380.460209263255820,
        205570.88069305533455,
        656958.69278146053058,
        -4406726.3053560412498,
        -460386.15389300137896,
        31745963.553354091997,
        -30185144.899501059008,
        -92885893.761842946947,
        159671874.67639934532,
        89037317.391947147287,
        -324112057.62064508289,
        75417289.288447113978,
        275169534.60077554540,
        -191141129.57979682342,
        -56632059.761422146328,
        92789782.492575658213,
        -24828398.690560814574,
    ],
    &[
        -595.11438847810495645,
        3784.5764889410929754,
        100370.50080364884343,
        -654419.09384311857985,
        -1371381.6192330607878,
        15498343.812751787009,
        -11665144.592299357907,
        -112601561.49426607138,
        219262318.25667364074,
        254104121.37964987144,
        -1006396481.7103226659,
        253036623.98729691889,
        1831393810.4978639927,
        -1774529065.4266491466,
        -1003472637.9292914864,
        2290280875.9786741918,
        -651246794.10887221841,
        -803784902.98109705890,
        640483342.29369123263,
        -138440607.21363135318,
    ],
    &[
        1140.6359112497011665,
        -8957.2682584379799664,
        -218407.33629125532461,
        1794837.6930232931017,
        2046161.0563529025012,
        -45986041.828079905965,
        74544641.814482732603,
        314850611.45725349592,
        -1044136171.4521382041,
        -187106978.33355739111,
        4438953626.0956817754,
        -4408582954.0483059788,
        -6264272163.4397192983,
        14149702487.577580987,
        -2713149740.6519136094,
        -14233558941.890554329,
        12753481214.719287040,
        934842459.40433410209,
        -6845048171.9341116529,
        3754053882.0127951637,
        -682202534.28377278514,
    ],
    &[
        -2036.8498415173235117,
        19163.284363303755526,
        436370.36938456366105,
        -4395730.5804739226086,
        -1112960.9840974107984,
        119491789.40923461317,
        -304912988.84344882540,
        -691963837.01336538937,
        3885386169.7360802683,
        -2313847981.3260245039,
        -14816925759.772210986,
        28337891715.905708458,
        7872387353.1924133326,
        -72435569735.091307437,
        59410896148.189366465,
        46141874867.831978016,
        -105411711029.14681739,
        45764643115.283153298,
        33619493138.706044340,
        -45524891856.627263052,
        19398990085.810093364,
        -3046176001.4829695650,
    ],
    &[
        3437.1841075604834259,
        -37884.316448339552153,
        -813572.82790964112831,
        9844700.1196742646463,
        -5932665.5737596173789,
        -278602941.80981757775,
        999702331.75599910068,
        1082145758.5676864833,
        -12097872233.934345656,
        16269568192.896168430,
        37442906272.629884505,
        -127678482632.39934914,
        56533169829.379619075,
        263705962704.22363017,
        -430880513772.40015440,
        28685028305.645455607,
        544657720548.16430202,
        -543385080747.85240194,
        33319140131.863860742,
        311025095335.01861076,
        -257492440682.78870598,
        90635479554.844098597,
        -12545989968.390205031,
    ],
    &[
        -5537.6855066252232973,
        70275.288364441469184,
        1432194.8403133915934,
        -20502591.084346145880,
        29691158.739889488095,
        592508323.24904278254,
        -2831998971.7799303680,
        -487772628.07968000257,
        32638786024.059947790,
        -70496897875.469508762,
        -62531875035.153643030,
        456706039700.77564846,
        -503779673552.18388727,
        -650638245764.84731771,
        2119375307389.9958522,
        -1373635599107.5234068,
        -1758554601545.7817139,
        3640007756944.3988399,
        -1912987782878.0613666,
        -1044942776057.9478291,
        2082243082925.6114167,
        -1292209352199.4454475,
        389815335189.77376153,
        -48292926381.689492854,
    ],
    &[
        6.0740420012734830379,
        0.0,
        -493.91530477308801242,
        0.0,
        7109.5143024893637214,
        0.0,
        -41192.654968897551298,
        0.0,
        122200.46498301745979,
        0.0,
        -203400.17728041553428,
        0.0,
        192547.00123253153236,
        0.0,
        -96980.598388637513489,
        0.0,
        20204.291330966148643,
    ],
    &[
        -45.555315009551122785,
        49.604676343733444810,
        4692.1953953443361180,
        -5021.4722651930614596,
        -81759.414478627682797,
        86499.090680287258611,
        556100.84208011694252,
        -583562.61205938197672,
        -1894107.2072367706267,
        1975574.1838921155999,
        3559503.1024072718499,
        -3695103.2205942155394,
        -3754666.5240343648810,
        3883031.1915227192359,
        2085082.8653557065400,
        -2149736.5976147982157,
        -474800.84627770449312,
        488270.37383168192555,
    ],
    &[
        193.61008879059227183,
        -437.33102409169077873,
        -24389.798720089893322,
        54330.683525039681367,
        481258.52318321001006,
        -1109084.2311883407405,
        -3433050.7548587226633,
        8650457.5434684857726,
        11004225.300068311602,
        -33238526.475380749062,
        -15303078.309507955098,
        69562860.629902112723,
        1830924.9239440239572,
        -80869740.517663243591,
        18943271.994495349280,
        49072182.784650581825,
        -19806771.899029389669,
        -12122574.798579689186,
        6307948.1226220563244,
    ],
    &[
        -613.09861450354219414,
        2147.8571771753195208,
        91956.399098661058815,
        -320284.80771632295052,
        -1938565.2131506522862,
        7533108.6348155997448,
        12364512.209251265036,
        -65358277.938386196419,
        -16530840.331176116137,
        269292234.00676317160,
        -105780946.98529899276,
        -575008738.51905591292,
        462747211.13824444405,
        619754381.87898314676,
        -755460241.94008607635,
        -252322946.06096464110,
        569416784.91969405599,
        -61357261.820865861243,
        -164974352.55837953060,
        57850732.229668052938,
    ],
    &[
        1609.3838630717982596,
        -7751.9961041252827272,
        -281313.57426896920515,
        1362914.7891508336068,
        5966457.1081110733990,
        -36097318.956064416329,
        -22309783.293551422257,
        336158425.14225148967,
        -205028522.03506721003,
        -1388339192.4100137759,
        1836112835.6997822732,
        2556726458.1368042032,
        -5652778880.4580993178,
        -1072688790.0156425156,
        8223828086.6764744334,
        -2962125430.6175380614,
        -5363095697.4586512056,
        4151384158.2283357977,
        758617226.29066830002,
        -1589602926.9007581937,
        422197436.26031767718,
    ],
    &[
        -3701.5828850651359971,
        22929.423816498228916,
        740942.45462626213339,
        -4683368.4794967535629,
        -14688082.295896812171,
        136985890.98054216279,
        -37063877.251336542111,
        -1319571104.7402945920,
        2005453574.1427636671,
        4917316117.1324589056,
        -13428166320.891447862,
        -3828355991.3598264145,
        37778220592.815991623,
        -20635506272.066951931,
        -47082248514.737817855,
        56649997707.122923550,
        14169411429.388481941,
        -52510965464.207838905,
        18673363121.781645026,
        14082226269.939926879,
        -12159500780.523353877,
        2607014902.9539700770,
    ],
    &[
        7711.6310105523666607,
        -58858.321154496479721,
        -1741907.2159207465870,
        13794087.929804007718,
        28916480.065464689455,
        -437983696.24595980730,
        494945307.55363422412,
        4180226129.2961517207,
        -10954467146.581302332,
        -11060935369.012122216,
        67274495072.654389414,
        -33385597946.547532147,
        -168738543918.69430001,
        236353638119.12099768,
        123426482948.35286349,
        -460569493399.15116382,
        183685386812.60549147,
        323426896220.20397235,
        -345059927788.03296106,
        18066712637.598243570,
        137638821647.93765226,
        -78528723144.419087956,
        14147149999.271829167,
    ],
    &[
        -14872.431234636707131,
        135739.23591067179473,
        3743563.4066693378186,
        -36117123.586437547311,
        -41857416.064054280688,
        1225475415.8400151136,
        -2467842162.9074118554,
        -10944143126.183310329,
        45219896350.687109257,
        3706147531.3941538260,
        -259638349679.49799671,
        331511238218.36097150,
        503674442974.85219625,
        -1481164218255.8010841,
        369277357070.34485189,
        2339646237464.7011180,
        -2569885298111.3265694,
        -667794049596.58740652,
        2906153064933.9250617,
        -1575977334606.5289263,
        -578377418663.53101806,
        1021516684285.5146634,
        -444821917816.52114438,
        69214137882.703873029,
    ],
    &[
        26956.281612779031676,
        -287752.74431752909550,
        -7479028.3293311244821,
        86134990.009811768863,
        23679004.644932133834,
        -3077161905.3885988089,
        9103660424.3905427045,
        23518972784.280263949,
        -154703115855.86457830,
        108277078595.12757644,
        805587125662.51652400,
        -1805165807562.2091925,
        -672890155245.49490410,
        6669143665397.1378050,
        -6132703262892.6427237,
        -7384886877294.0818151,
        17989877009001.983669,
        -6680625953666.2171081,
        -14315112877022.322823,
        17853687377733.207721,
        -3800827233430.3173356,
        -6918410846679.1440993,
        6365772360949.3812603,
        -2267586042740.4274751,
        310920009576.22258316,
    ],
    &[
        24.380529699556063861,
        0.0,
        -2499.8304818112096241,
        0.0,
        45218.768981362726273,
        0.0,
        -331645.17248456357783,
        0.0,
        1268365.2733216247816,
        0.0,
        -2813563.2265865341107,
        0.0,
        3763271.2976564039964,
        0.0,
        -2998015.9185381067501,
        0.0,
        1311763.6146629772007,
        0.0,
        -242919.18790055133346,
    ],
    &[
        -207.23450244622654282,
        223.48818891259725206,
        26248.220059017701053,
        -27914.773713558507469,
        -565234.61226703407842,
        595380.45825460922926,
        4808855.0010261718786,
        -5029951.7826825475971,
        -20928027.009806808897,
        21773603.858687892085,
        52050919.691850881048,
        -53926628.509575237122,
        -77147061.601956281926,
        79655909.133727217924,
        67455358.167107401877,
        -69454035.446132806377,
        -32138208.559242941417,
        33012717.635684926217,
        6437358.4793646103367,
        -6599304.6046316445590,
    ],
    &[
        984.36388661957607837,
        -2194.2476729600457475,
        -149715.34984220301505,
        329977.62359907967038,
        3636074.9553359345392,
        -8229815.9546080161817,
        -32850375.705398849013,
        79594841.396295258680,
        140766384.09976010426,
        -388119773.63641718318,
        -302391232.58882834949,
        1069154026.1028829621,
        267438889.51147761435,
        -1738631339.5172586463,
        117013574.77418801057,
        1654904787.0330349261,
        -452792004.09905362650,
        -852646349.53093518044,
        354479824.94387953335,
        183646906.05281680809,
        -95153470.227211795243,
    ],
    &[
        -3445.2736031685162743,
        11875.044917870854988,
        615370.50617503436198,
        -2111012.2880743031723,
        -16085470.193130487741,
        60142798.465327082670,
        138071565.42237886418,
        -645362876.57211229968,
        -403042168.77936625406,
        3392353592.8461412643,
        -459521271.54126358493,
        -9731832243.4128846845,
        5664098916.1399176504,
        15628587453.068137370,
        -14586127233.110452133,
        -13112286301.064475188,
        18076138583.500999115,
        3815036344.9012045022,
        -11188129037.135692765,
        1563031125.3210441483,
        2774182673.1720275815,
        -967769239.01720317824,
    ],
    &[
        9905.1616091094842886,
        -46820.775577189124306,
        -2040487.1579820312439,
        9691735.5725892393225,
        54815060.692073363805,
        -309390686.57090219229,
        -360017948.08027628199,
        3579817310.0615042649,
        -975311608.49901937973,
        -19324823653.635730322,
        19615966368.548239543,
        52313543472.729543241,
        -87319509031.594965641,
        -62880701946.001201560,
        187577712375.82047424,
        -5014883987.8038807144,
        -210187382319.75343210,
        95721572764.807326435,
        109424352395.85297165,
        -93578868051.240421500,
        -10054064393.166929203,
        29497575770.435656525,
        -7788016225.4016704591,
    ],
    &[
        -24762.904022773710722,
        150202.12250011534820,
        5795836.3854749992775,
        -35748503.915727151813,
        -151894819.53780369009,
        1257834576.5897312294,
        283767191.88351472079,
        -15254386392.322461649,
        17551211231.567450237,
        79224659176.160664514,
        -169127446206.19948897,
        -159965663833.05011647,
        667618088176.46828695,
        -96638419442.062469711,
        -1307530021252.0141810,
        998989571129.40670193,
        1171444346499.5230642,
        -1737201461305.1621935,
        -114746520425.41058268,
        1243665816084.6793073,
        -512525557301.93515073,
        -261796114655.33666678,
        247688439610.96543843,
        -52756420815.901269809,
    ],
    &[
        55716.534051240849124,
        -415613.72155461745024,
        -14629090.289521179515,
        112517365.15692088569,
        349569807.61470724572,
        -4300749690.4008170962,
        2780201489.2284702260,
        53008111096.391616511,
        -112833892526.88952297,
        -236800084652.33408957,
        948058960807.18952080,
        15893204800.580178374,
        -3467524908336.7843972,
        3164309467171.3659456,
        5582435675279.1817146,
        -10541249137249.730392,
        -1168774181536.2029753,
        14413943369875.125772,
        -7960463497916.1060492,
        -7323822211977.2888688,
        9405979314966.6882022,
        -1275663773372.9196006,
        -2930439830152.3544240,
        1747630840980.1348510,
        -312613977240.16973767,
    ],
    &[
        -115412.82053471318747,
        1027788.8260207103833,
        33623754.973679064933,
        -313584768.22511895708,
        -659891574.79280520771,
        12850764070.127691360,
        -19440345035.583477731,
        -155138555406.99154257,
        516911193990.92260102,
        451041007491.51124256,
        -4074544650661.9635765,
        3101195098601.7687528,
        13186035652463.635190,
        -24971415775329.260965,
        -10847664957177.134063,
        66205224018265.472705,
        -37500185472771.584184,
        -69777795804669.385671,
        99311193873506.569397,
        -492168373279.20159968,
        -80134019127845.984412,
        51160427044311.739411,
        9045641917569.2467301,
        -24123027505367.165512,
        10768904399045.846700,
        -1663085461560.5466581,
    ],
    &[
        110.01714026924673817,
        0.0,
        -13886.089753717040532,
        0.0,
        308186.40461266239848,
        0.0,
        -2785618.1280864546890,
        0.0,
        13288767.166421818329,
        0.0,
        -37567176.660763351308,
        0.0,
        66344512.274729026665,
        0.0,
        -74105148.211532657748,
        0.0,
        50952602.492664642206,
        0.0,
        -19706819.118432226927,
        0.0,
        3284469.8530720378211,
    ],
    &[
        -1045.1628325578440126,
        1118.5075927373418381,
        159690.03216774596612,
        -168947.42533689065981,
        -4160516.4622709423795,
        4365974.0653460506451,
        43177080.985340047679,
        -45034159.737397684138,
        -232553425.41238182077,
        241412603.52332969965,
        732559944.88488535051,
        -757604729.32539425138,
        -1426407013.9066740733,
        1470636688.7564934244,
        1741470982.9710174571,
        -1790874415.1120392289,
        -1299291363.5629483763,
        1333259765.2247248044,
        541937525.75688624049,
        -555075405.16917439177,
        -96891860.665625115724,
        99081507.234339807604,
    ],
    &[
        5487.1048709286810663,
        -12101.885429617141199,
        -991438.75239470139087,
        2166230.0015798583230,
        28994419.876786743130,
        -64719144.968659103681,
        -321629835.31147623339,
        757688130.83951567540,
        1749409837.5100643555,
        -4544758370.9162618687,
        -5113992851.9544763313,
        15778214197.520607549,
        7774473545.9444870052,
        -33570323211.012887492,
        -3804246527.4759322626,
        44463088926.919594649,
        -5920634247.3331925357,
        -35768726949.850578829,
        10834752690.813605970,
        16001937124.166968265,
        -6803368741.9070923418,
        -3054556963.3569951737,
        1577229794.0273014954,
    ],
    &[
        -21033.902005226610754,
        71551.022388357981754,
        4410889.4214898588524,
        -14945521.653227529678,
        -139310283.22771754330,
        506112018.78208167499,
        1519598088.8133635683,
        -6552067582.0564506220,
        -6692370095.4282068536,
        42444451633.414257582,
        5560288488.7766793217,
        -155035909620.11651130,
        57543525805.054081844,
        335136386281.40047184,
        -242028870673.91835183,
        -425158049601.57076545,
        447281929473.23048294,
        285539009675.26705395,
        -446532174700.75534828,
        -56114815650.416797381,
        234199738711.39910784,
        -38367666879.807869654,
        -50717346515.577689017,
        17618025541.849480837,
    ],
    &[
        65730.943766333158607,
        -305976.00327882005331,
        -15752484.913133094466,
        73626048.766219891723,
        517727813.38636845015,
        -2779778621.0311819654,
        -4848484539.3006943808,
        38867748248.486701209,
        2816113040.3833130041,
        -261989764937.19714608,
        193832179511.35043238,
        941830771354.52418439,
        -1252062600825.4805385,
        -1788586580037.9230755,
        3715406759129.6400037,
        1338049108553.0784173,
        -6078517409833.2670243,
        1046877894952.0462047,
        5489626110236.1661278,
        -2928719118680.3861544,
        -2340856193318.6421567,
        2206204676067.3123808,
        119173943641.45927347,
        -589883942966.21075926,
        154983207892.81174977,
    ],
    &[
        -177473.54816909952824,
        1058104.4704696212045,
        47977824.897952560186,
        -290115715.66147843083,
        -1577194622.2444813090,
        12040320836.774941282,
        8938496432.7761692719,
        -177729149538.54760085,
        142764258530.36399735,
        1190892990607.9998843,
        -2059313111949.9121859,
        -3727699663610.1911429,
        10889256225145.522967,
        3246312869985.9515053,
        -29702334678008.056483,
        12354361209407.777556,
        43335047349400.917011,
        -41504037524268.002306,
        -28336804589555.309295,
        52945932725332.519399,
        -2984104941157.1313565,
        -30618403352283.411013,
        14099792752244.009722,
        5138575314225.4923364,
        -5394419195595.0379138,
        1142750145697.5795143,
    ],
    &[
        428894.40807532385991,
        -3139491.0587579525708,
        -129346362.91769878378,
        971658116.23636815026,
        4056885717.6650654269,
        -43776518263.068506188,
        6886509445.7336568338,
        666192636622.63242699,
        -1146605162893.2335873,
        -4150515189210.3819757,
        12899393468582.902633,
        7742402532790.6606612,
        -63390570866544.420475,
        31574846906545.697293,
        155509830706127.21122,
        -197328514020298.70121,
        -161495194791368.91656,
        431593958485279.11169,
        -57955664355845.420187,
        -444813800230214.83356,
        304547176185415.82363,
        164529366533754.02760,
        -262168848376474.26253,
        51430539333046.601717,
        65933562346693.241986,
        -41287526582645.050139,
        7341963962580.3111652,
    ],
    &[
        551.33589612202058561,
        0.0,
        -84005.433603024085289,
        0.0,
        2243768.1779224494292,
        0.0,
        -24474062.725738728468,
        0.0,
        142062907.79753309519,
        0.0,
        -495889784.27503030925,
        0.0,
        1106842816.8230144683,
        0.0,
        -1621080552.1083370752,
        0.0,
        1553596899.5705800562,
        0.0,
        -939462359.68157840255,
        0.0,
        325573074.18576574902,
        0.0,
        -49329253.664509961973,
    ],
    &[
        -5789.0269092812161489,
        6156.5841733625632060,
        1050067.9200378010661,
        -1106071.5424398171230,
        -32534638.579875516724,
        34030484.031823816343,
        403822034.97468901972,
        -420138076.79184817203,
        -2628163794.2543622609,
        2722872399.4527176577,
        10165740577.638121340,
        -10496333767.154808213,
        -24903963378.517825536,
        25641858589.733168515,
        39716473526.654258344,
        -40797193894.726483060,
        -41170317838.620371488,
        42206049105.000758192,
        26774677250.924984473,
        -27400985490.712703408,
        -9929978762.6658553451,
        10147027478.789699178,
        1603200744.0965737641,
        -1636086913.2062470721,
    ],
    &[
        33286.904728366992856,
        -72776.338288106717300,
        -7048423.0820374073034,
        15288988.915750383523,
        243935418.08573977628,
        -538504362.70138786302,
        -3246894911.6396827767,
        7489063194.0760509112,
        21666937100.705365161,
        -53983904963.062576171,
        -80910564664.877465851,
        229101080335.06400288,
        172760876423.44066571,
        -610977234886.30398648,
        -187937135374.72033958,
        1053702358870.4190989,
        18639458829.443774842,
        -1174519256075.3585225,
        213630362751.48244186,
        817332252922.97321022,
        -266086886489.81593243,
        -322968489592.27962303,
        139744842706.19027127,
        55347422611.580177333,
        -28497920919.101275948,
    ],
    &[
        -138695.43636819580357,
        466698.94436858890046,
        33739004.101605793511,
        -113151831.10727233791,
        -1262494179.8855194512,
        4485679097.1221702841,
        16876412838.414698204,
        -68734331237.685231727,
        -99319007190.867271605,
        535143925100.64569760,
        219559829042.68087919,
        -2402169771537.2297740,
        385225420494.61274582,
        6605716163805.3629860,
        -3536807746028.4626688,
        -11320974870399.200872,
        9487604757721.1762395,
        11727696199159.893852,
        -13727281916428.067929,
        -6409955842808.2983124,
        11468445778721.253331,
        723160235150.79794197,
        -5214971540434.5399686,
        952560905703.62661138,
        1001898723474.6755508,
        -346817425242.52751961,
    ],
    &[
        468097.09774266083704,
        -2151404.5559841446618,
        -129076990.59865050915,
        595316208.51428773472,
        5064273602.5906671292,
        -26185510869.073114681,
        -61837037264.905576505,
        433386998010.23271090,
        186443883288.37312122,
        -3537220973754.0018335,
        1681730347233.8501195,
        15988837394678.875714,
        -16993822946683.547492,
        -41340811227869.288548,
        67591520066179.679649,
        56621717984129.588648,
        -149792388677379.47570,
        -20183328379251.847125,
        196651762467137.74506,
        -54964390301576.370525,
        -147653284026647.88789,
        88925712863728.440410,
        52472318964377.745533,
        -54571161032830.893735,
        -776744894101.81078918,
        12653076888080.966521,
        -3310861678129.4432166,
    ],
    &[
        -1357481.5834537164274,
        7977851.1535147665703,
        419517728.92817665197,
        -2495517054.7161424403,
        -16720780562.050493234,
        120296564778.19436333,
        154404724838.89607546,
        -2110123804399.9206379,
        974123695127.88869692,
        17443881430511.946630,
        -24448434409173.656268,
        -73513952038406.365892,
        169708094306922.00526,
        139392884698962.95542,
        -607701552209729.47437,
        42904363912061.406899,
        1241046770899817.3524,
        -768125556097572.10187,
        -1403171934960438.0242,
        1622188475923667.7103,
        657623934547413.88722,
        -1631882122398223.2875,
        237083343503533.13124,
        785835171244720.50574,
        -396417689594574.31797,
        -105868095076065.58561,
        125179414423474.77661,
        -26396909127729.654202,
    ],
    &[
        3038.0905109223842686,
        0.0,
        -549842.32757228868713,
        0.0,
        17395107.553978164538,
        0.0,
        -225105661.88941527780,
        0.0,
        1559279864.8792575133,
        0.0,
        -6563293792.6192843320,
        0.0,
        17954213731.155600080,
        0.0,
        -33026599749.800723140,
        0.0,
        41280185579.753973955,
        0.0,
        -34632043388.158777923,
        0.0,
        18688207509.295824922,
        0.0,
        -5866481492.0518472276,
        0.0,
        814789096.11831211495,
    ],
    &[
        -34938.040875607419089,
        36963.434549555675268,
        7422871.4222258972763,
        -7789432.9739407564011,
        -269624167.08666155034,
        281220905.45598032670,
        3939349083.0647673616,
        -4089419524.3243775468,
        -30405957365.145521510,
        31445477275.065026519,
        141110816541.31461314,
        -145486345736.39413603,
        -421924022682.15660188,
        433893498502.92700194,
        842178293619.91844007,
        -864196026786.45225550,
        -1135205103443.2342838,
        1162725227163.0702664,
        1021645279950.6839487,
        -1044733308876.1231340,
        -588678536542.81848505,
        601137341549.01570167,
        196527129983.73688212,
        -200438117645.10478028,
        -28925012912.200080081,
        29468205642.945621491,
    ],
    &[
        218362.75547254636931,
        -473942.11970389194590,
        -53559985.272697166145,
        115466888.79018062430,
        2162702487.2919505639,
        -4731469254.6820607544,
        -33930459549.835830283,
        76986136366.180025009,
        271095146839.75321729,
        -654897543249.28815561,
        -1244130265844.5028996,
        3321026659065.3578720,
        3434492363731.4649585,
        -10772528238693.360048,
        -5553407245149.3813559,
        23184673024360.107644,
        4148109873524.0835034,
        -33519510690760.226852,
        1766187462911.1875877,
        32207800350987.663468,
        -7064569616534.6127663,
        -19734747129816.391118,
        6780185269401.9041713,
        6981112975541.6982009,
        -3063627371132.2565100,
        -1085299076029.5917371,
        557485489473.28346831,
    ],
    &[
        -982632.39962645866188,
        3276416.0527937831379,
        274430595.86767397376,
        -912438377.61048048104,
        -11979589299.502912885,
        41816043571.570359444,
        191330729548.58232223,
        -746943185635.96708583,
        -1416198224023.6464721,
        6856956393274.5884258,
        4829515891796.9969935,
        -36877687475484.603376,
        -2050957082915.0942624,
        124378250253033.78820,
        -44312193894090.533579,
        -271207689501172.62955,
        179586202876957.77361,
        381618869449173.21877,
        -359490061117882.96881,
        -330393612742248.46582,
        427952236370799.92495,
        148052565489017.04595,
        -307266140839707.15754,
        -4687264091266.0864760,
        123260326898726.61625,
        -24376238900981.871642,
        -21272360948501.370513,
        7341892911307.8818418,
    ],
    &[
        3562042.4486459126493,
        -16196313.687936474068,
        -1119546446.8249808309,
        5105896697.6570577310,
        51476060928.630380287,
        -258447407741.23544355,
        -779649080899.64056288,
        4982179421789.6942804,
        4004696303571.8542110,
        -48152095258426.146318,
        10206156605278.263985,
        264344253278752.35082,
        -218798857956960.84499,
        -868622560677829.56823,
        1161601525559524.2658,
        1687979001191903.2190,
        -3349241418127553.8805,
        -1649878672532120.5740,
        5897451215285124.6365,
        -125952827188585.92633,
        -6433283969334751.7469,
        2343935079955608.5977,
        4106725553395393.7642,
        -2735980005540132.5807,
        -1230142327980830.8915,
        1417490532431040.0017,
        -23384761374805.900562,
        -289892454526107.40352,
        75592403781851.468191,
    ],
    &[
        18257.755474293174691,
        0.0,
        -3871833.4425726126206,
        0.0,
        143157876.71888898129,
        0.0,
        -2167164983.2237950935,
        0.0,
        17634730606.834969383,
        0.0,
        -87867072178.023265677,
        0.0,
        287900649906.15058872,
        0.0,
        -645364869245.37650328,
        0.0,
        1008158106865.3820948,
        0.0,
        -1098375156081.2233068,
        0.0,
        819218669548.57732864,
        0.0,
        -399096175224.46649796,
        0.0,
        114498237732.02580995,
        0.0,
        -14679261247.695616661,
    ],
    &[
        -228221.94342866468364,
        240393.78041152680010,
        56141584.917302882999,
        -58722807.212351291413,
        -2362104965.8616681913,
        2457543550.3409275122,
        40092552189.640209230,
        -41537328845.122739292,
        -361511977440.11687235,
        373268464511.34018528,
        1977009124005.5234777,
        -2035587172124.2056548,
        -7053565922700.6894237,
        7245499689304.7898162,
        17102169035002.477337,
        -17532412281166.061672,
        -28732506045663.389701,
        29404611450240.311097,
        33500442260477.310858,
        -34232692364531.459729,
        -26624606760328.763181,
        27170752540027.814733,
        13768818045244.094179,
        -14034882162060.405178,
        -4179185677218.9420633,
        4255517835706.9592699,
        565151558036.28124143,
        -574937732201.41165254,
    ],
    &[
        1540498.1181434866146,
        -3322911.4963213577938,
        -433313348.25129661430,
        929240026.21742702895,
        20173953055.394385937,
        -43806310275.980028275,
        -367752562201.24170098,
        823522693625.04213554,
        3453452850623.2709660,
        -8147245540357.7558550,
        -18967395863304.498472,
        48502623842268.842654,
        64652876623215.013090,
        -187135422438997.88267,
        -137928375585894.45832,
        487895841149504.63648,
        171009666849543.97695,
        -877097552972882.42245,
        -74254863627598.106482,
        1089588154832573.5204,
        -116085557257555.85969,
        -919163347233503.76274,
        228872931917376.68922,
        502861180782827.78742,
        -180138187046491.99093,
        -160984522251228.28879,
        71472589051967.572740,
        22899647546405.161991,
        -11739127546959.248774,
    ],
    &[
        -7445740.9043601853037,
        24634048.351746637287,
        2366020806.0262900637,
        -7808648260.6425043412,
        -118977141825.00262846,
        409347654830.37623610,
        2227886871742.0341399,
        -8418196051278.7614582,
        -19993114336865.499162,
        89749606285710.604304,
        91027650219177.795567,
        -567320456048460.00800,
        -158541319151866.40491,
        2286994298876364.5224,
        -406108119093234.18714,
        -6109006264284954.9274,
        3061509093597577.3617,
        10949569443954370.550,
        -8409227264715248.2782,
        -12972671111828071.122,
        13506857028733485.383,
        9542099548494263.3202,
        -13665198589678693.875,
        -3500771616752033.2515,
        8599505926079281.8598,
        -192584364085979.71888,
        -3085105840164255.6689,
        648294322883069.58159,
        483163296698761.31750,
        -166336791576720.83219,
    ],
    &[
        118838.42625678325312,
        0.0,
        -29188388.122220813403,
        0.0,
        1247009293.5127103248,
        0.0,
        -21822927757.529223729,
        0.0,
        205914503232.41001569,
        0.0,
        -1196552880196.1815990,
        0.0,
        4612725780849.1319668,
        0.0,
        -12320491305598.287160,
        0.0,
        23348364044581.840938,
        0.0,
        -31667088584785.158403,
        0.0,
        30565125519935.320612,
        0.0,
        -20516899410934.437391,
        0.0,
        9109341185239.8989559,
        0.0,
        -2406297900028.5039611,
        0.0,
        286464035717.67904299,
    ],
    &[
        -1604318.7544665739172,
        1683544.3719710960859,
        452420015.89442260775,
        -471878941.30923648336,
        -21822662636.472430684,
        22654002165.480904234,
        425547091271.81986272,
        -440095709776.83934521,
        -4427161819496.8153373,
        4564438154985.0886811,
        28118992684610.267576,
        -28916694604741.055309,
        -117624507411652.86515,
        120699657932218.95313,
        338813510903952.89689,
        -347027171774351.75500,
        -688776739315164.30766,
        704342315344885.53495,
        997513290420732.48968,
        -1018624682810589.2619,
        -1023931704917833.2405,
        1044308455264456.7876,
        728349929088172.52737,
        -742027862028795.48563,
        -341600294446496.21085,
        347673188569989.47682,
        95048767051125.906463,
        -96652965651144.909104,
        -11888257482283.680284,
        12079233506095.466313,
    ],
    &[
        11631310.969882660899,
        -24956069.513924483156,
        -3719130469.3827566264,
        7939241569.2440612457,
        197650420583.57805736,
        -426477178381.34693109,
        -4137136219101.0505865,
        9165629658162.2739663,
        44999979919399.924736,
        -104192738635599.46794,
        -290053332678279.44824,
        717931728117708.95938,
        1184950942733150.9332,
        -3238133498156090.6407,
        -3148099361614567.8423,
        10004238940145809.174,
        5326672157182975.4416,
        -21713978561461112.072,
        -4997511985428331.4237,
        33440445545533127.273,
        429328409587666.98618,
        -36372499368723031.528,
        5419838346824587.4483,
        27328510015364670.604,
        -7462027883028047.7909,
        -13500043636525530.253,
        5000259547410615.2462,
        3946328556046746.4962,
        -1769166076587921.0596,
        -517354048506128.35163,
        264752449010576.61885,
    ],
    &[
        832859.30401628929898,
        0.0,
        -234557963.52225152478,
        0.0,
        11465754899.448237157,
        0.0,
        -229619372968.24646817,
        0.0,
        2485000928034.0853236,
        0.0,
        -16634824724892.480519,
        0.0,
        74373122908679.144941,
        0.0,
        -232604831188939.92523,
        0.0,
        523054882578444.65558,
        0.0,
        -857461032982895.05140,
        0.0,
        1026955196082762.4888,
        0.0,
        -889496939881026.44181,
        0.0,
        542739664987659.72270,
        0.0,
        -221349638702525.19597,
        0.0,
        54177510755106.049005,
        0.0,
        -6019723417234.0054450,
    ],
    &[
        -12076459.908236194835,
        12631699.444247054368,
        3870206398.1171501588,
        -4026578373.7986511753,
        -212116465639.79238740,
        219760302239.42454551,
        4707197145849.0525974,
        -4860276727827.8835762,
        -55912520880766.919782,
        57569188166122.976664,
        407553205759865.77271,
        -418643088909794.09305,
        -1970887757079997.3409,
        2020469839019116.7709,
        6629237688884787.8691,
        -6784307576344081.1526,
        -15953173918642561.995,
        16301877173694858.432,
        27867483571944089.170,
        -28439124260599352.538,
        -35429954264855305.864,
        36114591062243814.190,
        32466638305657465.126,
        -33059636265578149.421,
        -20895477102024899.324,
        21257303545350005.806,
        8964660367452270.4366,
        -9112226793253953.9006,
        -2302544207092007.0827,
        2338662547595411.1154,
        267877692066913.24230,
        -271890841011735.91260,
    ],
    &[
        6252951.4934347970025,
        0.0,
        -2001646928.1917763315,
        0.0,
        110997405139.17901279,
        0.0,
        -2521558474912.8546213,
        0.0,
        31007436472896.461417,
        0.0,
        -236652530451649.25168,
        0.0,
        1212675804250347.4165,
        0.0,
        -4379325838364015.4378,
        0.0,
        11486706978449752.110,
        0.0,
        -22268225133911142.562,
        0.0,
        32138275268586241.200,
        0.0,
        -34447226006485144.698,
        0.0,
        27054711306197081.241,
        0.0,
        -15129826322457681.181,
        0.0,
        5705782159023670.8096,
        0.0,
        -1301012723549699.4268,
        0.0,
        135522158703093.69029,
    ],
];

fn fjkproc16(u: f64, fjk: &mut [[f64; 17]; 17]) {
    let mut un = [0.0; 65]; // un(0:64)
    un[1] = u;
    let v = u * u;
    un[2] = v;

    for n in 2..=64 {
        un[n] = u * un[n - 1];
    }

    fjk[0][0] = 1.0;

    let mut index = 0;

    #[allow(clippy::needless_range_loop)]
    for k in 0..=16 {
        let start = if k == 0 { 1 } else { 0 };
        for j in start..=16 - k {
            let d = j + 2 * k;
            let fjkm = FJKM[index];
            fjk[j][k] = un[d] * pol(fjkm, d, v);
            index += 1;
        }
    }
}

fn startingpser(mu: f64, x: f64, y: f64) -> i32 {
    let mulnmu = mu * mu.ln();
    let lnx = x.ln();
    let lny = y.ln();
    let mut n = if x < 2.0 { x + 5.0 } else { 1.5 * x };

    let mut n1 = 0.0;
    let mut a = 0;
    let mut b = 0;
    while (n - n1).abs() > 1.0 {
        n1 = n;
        n = ps(mu, mulnmu, lnx, y, lny, n, a, b);
    }

    n += 1.0;
    if mu + n > y {
        if y > mu {
            a = 1;
        } else {
            b = 1;
        }
        n1 = 0.0;
        while (n - n1).abs() > 1.0 {
            n1 = n;
            n = ps(mu, mulnmu, lnx, y, lny, n, a, b);
        }
    }

    n as i32 + 1
}

#[allow(clippy::too_many_arguments)]
fn ps(
    mu: f64,
    mulnmu: f64,
    lnx: f64,
    y: f64,
    lny: f64,
    n: f64,
    a: i32,
    b: i32,
) -> f64 {
    let lneps = EPSS.ln();

    if (a == 0) && (b == 0) {
        (n - lneps) / (n.ln() - lnx)
    } else if (a == 0) && (b == 1) {
        (2.0 * n - lneps + mulnmu - mu * (mu + n).ln())
            / (n.ln() - lnx - lny + (mu + n).ln())
    } else if (a == 1) && (b == 0) {
        (2.0 * n - lneps - y + mu * lny - mu * (mu + n).ln() + mu)
            / (n.ln() - lnx - lny + (mu + n).ln())
    } else {
        0.0
    }
}

fn hypfun(x: f64) -> (f64, f64) {
    let ax = x.abs();
    if ax < 0.21 {
        let y = if ax < 0.07 { x * x } else { x * x / 9.0 };

        let f = 2.0 + y * (y * 28.0 + 2520.0) / (y * (y + 420.0) + 15120.0);
        let f2 = f * f;
        let mut sinh = 2.0 * x * f / (f2 - y);
        let mut cosh = (f2 + y) / (f2 - y);

        if ax >= 0.07 {
            let ss = 2.0 * sinh / 3.0;
            let f = ss * ss;
            sinh *= 1.0 + f / 3.0;
            cosh *= 1.0 + f;
        }

        (sinh, cosh)
    } else {
        let y = x.exp();
        let f = 1.0 / y;
        let cosh = (y + f) / 2.0;
        let sinh = (y - f) / 2.0;

        (sinh, cosh)
    }
}

/// Computes the Upper Incomplete Gama(1/2 - n, x), x >= 0,
/// n = 0, 1, 2, ...
fn ignega(n: i32, x: f64, _eps: f64) -> f64 {
    let a = 0.5 - (n as f64);
    let delta = EPSS / 100.0;
    if x > 1.5 {
        let mut p = 0.0;
        let mut q = (x - 1.0 - a) * (x + 1.0 - a);
        let mut r = 4.0 * (x + 1.0 - a);
        let mut s = 1.0 - a;
        let mut ro = 0.0;
        let mut t = 1.0;
        let mut k = 0;
        let mut g = 1.0;
        while ((t / g) > delta) && (k < 1000) {
            p += s;
            q += r;
            r += 8.0;
            s += 2.0;
            let tau = p * (1.0 + ro);
            ro = tau / (q - tau);
            t *= ro;
            g += t;
            k += 1;
        }

        g * (a * x.ln()).exp() / (x + 1.0 - a)
    } else {
        let mut t = 1.0;
        let mut s = 1.0 / a;
        let mut k = 1;
        while ((t / s).abs() > delta) && (k < 1000) {
            t = -x * t / (k as f64);
            s += t / ((k as f64) + a);
            k += 1;
        }

        let mut g = SQRT_PI;
        for k in 0..n {
            g /= 0.5 - ((k + 1) as f64);
        }
        if x > 0.0 {
            g = x.exp() * (g - (a * x.ln()).exp() * s);
        }

        g
    }
}

fn startkbes(x: f64, eps: f64) -> i32 {
    let del = if eps < MACHTOL {
        -(MACHTOL / 2.0).ln()
    } else {
        -(eps / 2.0).ln()
    };

    let p = x + del;
    let q = p / x;
    let (r, y);
    if q < 2.0 {
        r = ((q + 1.0) / (q - 1.0)).ln() / 2.0;
        y = r * (1.0 + 2.0 / (1.0 + r * (q + 1.0 / q)));
    } else {
        r = 2.0 * x / p;
        let r2 = r * r;
        y = r * (1.0 + r2 * r2 / 45.0);
    }
    let (s, _c) = hypfun(y);

    1 + (x / (2.0 * s * s)) as i32
}

fn startijbes(x: f64, n: i32, t: i32, eps: f64) -> i32 {
    if x <= 0.0 {
        return 0;
    }

    let mut s = 2 * t - 1;
    let del = if eps < MACHTOL {
        -(MACHTOL / 2.0).ln()
    } else {
        -(eps / 2.0).ln()
    };

    let mut p = del / x - (t as f64);
    let r = (n as f64) / x;
    let r = if (r > 1.0) || (t == 1) {
        let q = (r * r + (s as f64)).sqrt();
        r * (q + r).ln() - q
    } else {
        0.0
    };

    let mut q = del / (2.0 * x) + r;

    let r = if p > q { p } else { q };
    let y = alfinv(t, r, p, q);
    (p, q) = hypfun(y);

    s = if t == 0 {
        (x * q) as i32 + 1
    } else {
        (x * p) as i32 + 1
    };
    if s % 2 > 0 {
        s += 1;
    }

    s
}

fn alfinv(t: i32, r: f64, _p: f64, q: f64) -> f64 {
    let (mut a, mut b) = if ((t as f64) + r) < 2.7 {
        let (a, b) = if t == 0 {
            let a = ((3.0 * r).ln() / 3.0).exp();
            let a2 = a * a;
            let b = a * (1.0 + a2 * (-1.0 / 30.0 + 0.004312 * a2));

            (a, b)
        } else {
            let a = (2.0 * (1.0 + r)).sqrt();
            let a2 = a * a;
            let b = a / (1.0 + a2 / 8.0);

            (a, b)
        };

        (a, b)
    } else {
        let a = (0.7357589 * (r + (t as f64))).ln();
        let lna = a.ln() / a;
        let b = 1.0 + a + a.ln() * (1.0 / a - 1.0) + 0.5 * lna * lna;

        (a, b)
    };

    while (a / b - 1.0).abs() > 1.0e-2 {
        a = b;
        (b, _) = fi(a, r, t, q);
    }

    b
}

fn falfa(al: f64, r: f64, t: i32) -> (f64, f64) {
    let (sh, ch) = hypfun(al);
    if t == 1 {
        let falfa = al * sh / ch - 1.0 - r / ch;
        let df = (sh + (al + r * sh) / ch) / ch;

        (falfa, df)
    } else {
        let falfa = al - (sh + r) / ch;
        let df = sh * (r + sh) / (ch * ch);

        (falfa, df)
    }
}

fn fi(al: f64, r: f64, t: i32, _q: f64) -> (f64, f64) {
    let (p, q) = falfa(al, r, t);
    let fi = al - p / q;

    (fi, q)
}

/// recipgam(x) = 1 / gamma(x + 1) = 1 + x * (q + x * r);
///
/// -0.5 <= x <= 0.5
fn recipgam(x: f64) -> f64 {
    let (q, r);

    #[allow(clippy::approx_constant)]
    if x == 0.0 {
        q = 0.5772156649015328606e-0;
        r = -0.6558780715202538811e-0;
    } else {
        let c = [
            1.142022680371167841,
            -6.5165112670736881e-3,
            -3.087090173085368e-4,
            3.4706269649043e-6,
            -6.9437664487e-9,
            -3.67795399e-11,
            1.356395e-13,
            3.68e-17,
            -5.5e-19,
        ];
        let tx = 2.0 * x;
        let t = 2.0 * tx * tx - 1.0;
        q = chepolsum(8, t, &c);
        let c = [
            -1.270583625778727532,
            2.05083241859700357e-2,
            -7.84761097993185e-5,
            -5.377798984020e-7,
            3.8823289907e-9,
            -2.6758703e-12,
            -2.39860e-14,
            3.80e-17,
            4e-20,
        ];
        r = chepolsum(8, t, &c);
    }

    1.0 + x * (q + x * r)
}

fn xpowy(x: f64, y: f64) -> f64 {
    x.powf(y)
}

fn fractio(x: f64, n: usize, r: &[f64], s: &[f64]) -> f64 {
    let mut a = r[n];
    let mut b = 1.0;
    for k in (0..n).rev() {
        a = a * x + r[k];
        b = b * x + s[k];
    }

    a / b
}

fn zetaxy(x: f64, y: f64) -> f64 {
    let z = y - x - 1.0;
    let x2 = x * x;
    let x3 = x2 * x;
    let x4 = x3 * x;
    let x5 = x4 * x;
    let x6 = x5 * x;
    let x7 = x6 * x;
    let x8 = x7 * x;
    let x9 = x8 * x;
    let x10 = x9 * x;
    let x2p1 = 2.0 * x + 1.0;
    if z.abs() < 0.05 {
        let ck = [
            1.0,
            -(1.0 / 3.0) * (3.0 * x + 1.0),
            (1.0 / 36.0) * (72.0 * x2 + 42.0 * x + 7.0),
            -(1.0 / 540.0) * (2700.0 * x3 + 2142.0 * x2 + 657.0 * x + 73.0),
            (1.0 / 12960.0)
                * (1331.0
                    + 15972.0 * x
                    + 76356.0 * x2
                    + 177552.0 * x3
                    + 181440.0 * x4),
            -(1.0 / 272160.0)
                * (22409.0
                    + 336135.0 * x
                    + 2115000.0 * x2
                    + 7097868.0 * x3
                    + 13105152.0 * x4
                    + 11430720.0 * x5),
            (1.0 / 5443200.0)
                * (6706278.0 * x
                    + 52305684.0 * x2
                    + 228784392.0 * x3
                    + 602453376.0 * x4
                    + 935038080.0 * x5
                    + 718502400.0 * x6
                    + 372571.0),
            -(1.0 / 16329600.0)
                * (953677.0
                    + 20027217.0 * x
                    + 186346566.0 * x2
                    + 1003641768.0 * x3
                    + 3418065864.0 * x4
                    + 7496168976.0 * x5
                    + 10129665600.0 * x6
                    + 7005398400.0 * x7),
            (1.0 / 783820800.0)
                * (39833047.0
                    + 955993128.0 * x
                    + 1120863744000.0 * x8
                    + 10332818424.0 * x2
                    + 66071604672.0 * x3
                    + 275568952176.0 * x4
                    + 776715910272.0 * x5
                    + 1472016602880.0 * x6
                    + 1773434373120.0 * x7),
            -(1.0 / 387991296000.0)
                * (17422499659.0
                    + 470407490793.0 * x
                    + 3228423729868800.0 * x8
                    + 1886413681152000.0 * x9
                    + 5791365522720.0 * x2
                    + 42859969263000.0 * x3
                    + 211370902874640.0 * x4
                    + 726288467241168.0 * x5
                    + 1759764571151616.0 * x6
                    + 2954947944510720.0 * x7),
            (1.0 / 6518253772800.0)
                * (261834237251.0
                    + 7855027117530.0 * x
                    + 200149640441008128.0 * x8
                    + 200855460151664640.0 * x9
                    + 109480590367948800.0 * x10
                    + 108506889674064.0 * x2
                    + 912062714644368.0 * x3
                    + 5189556987668592.0 * x4
                    + 21011917557260448.0 * x5
                    + 61823384007654528.0 * x6
                    + 132131617757148672.0 * x7),
        ];

        let z2 = z / (x2p1 * x2p1);
        let mut s = 1.0;
        let mut t = 1.0_f64;
        let mut k = 1;
        while (t.abs() > 1.0e-15) && (k < 11) {
            t = ck[k] * z2.powi(k as i32);
            s += t;
            k += 1;
        }

        -z / x2p1.sqrt() * s
    } else {
        let w: f64 = (1.0 + 4.0 * x * y).sqrt();
        let mut zeta = (2.0 * (x + y - w - (2.0 * y / (1.0 + w)).ln())).sqrt();
        if x + 1.0 < y {
            zeta = -zeta;
        }

        zeta
    }
}

fn chepolsum(n: usize, t: f64, ak: &[f64]) -> f64 {
    let mut u0 = 0.0;
    let mut u1 = 0.0;
    let mut u2 = 0.0;
    let tt = t + t;
    for k in (0..=n).rev() {
        u2 = u1;
        u1 = u0;
        u0 = tt * u1 - u2 + ak[k];
    }

    (u0 - u2) / 2.0
}

fn oddchepolsum(n: usize, x: f64, ak: &[f64]) -> f64 {
    if n == 0 {
        ak[0] * x
    } else if n == 1 {
        x * (ak[0] + ak[1] * (4.0 * x * x - 3.0))
    } else {
        let y = 2.0 * (2.0 * x * x - 1.0);
        let mut r = ak[n];
        let mut h = ak[n - 1] + r * y;
        for k in (0..=n - 2).rev() {
            let s = r;
            r = h;
            h = ak[k] + r * y - s;
        }

        x * (h - r)
    }
}

fn logoneplusx(t: f64) -> f64 {
    if (-0.2928 < t) && (t < 0.4142) {
        let mut p = TWO_EXP_1_4;
        p = (p - 1.0) / (p + 1.0);
        let mut pj = p;
        let mut ck = [0.0; 101];
        ck[0] = pj;
        let p2 = p * p;
        let mut j = 1;
        let mut c = 1.0_f64;
        while (c.abs() > 1.0e-20) && (j < 1000) {
            pj *= p2;
            c = pj / (2.0 * (j as f64) + 1.0);
            ck[j] = c;
            j += 1;
        }
        let x = t / (2.0 + t) * (1.0 + p2) / (2.0 * p);

        4.0 * oddchepolsum(j - 1, x, &ck)
    } else {
        (1.0 + t).ln()
    }
}

/// (x-sin(x))/(x^3/6)
fn xminsinx(x: f64) -> f64 {
    if x.abs() > 1.0 {
        6.0 * (x - x.sin()) / (x * x * x)
    } else {
        const FK: [f64; 9] = [
            1.95088260487819821294e-0,
            -0.244124470324439564863e-1,
            0.14574198156365500e-3,
            -0.5073893903402518e-6,
            0.11556455068443e-8,
            -0.185522118416e-11,
            0.22117315e-14,
            -0.2035e-17,
            0.15e-20,
        ];
        let t = 2.0 * x * x - 1.0;

        chepolsum(8, t, &FK)
    }
}

#[allow(clippy::too_many_arguments)]
fn trapsum(
    a: f64,
    b: f64,
    h: f64,
    d: f64,
    xis2: f64,
    mu: f64,
    wxis: f64,
    ys: f64,
) -> f64 {
    let mut s = 0.0;
    let mut b0 = b;
    let (mut aa, bb) = if d == 0.0 {
        let inte = integrand(a, &mut b0, xis2, mu, wxis, ys);
        s = inte / 2.0;

        (a + h, b - h / 2.0)
    } else {
        (a + d, b)
    };

    while (aa < bb) && (aa < b0) {
        let inte = integrand(aa, &mut b0, xis2, mu, wxis, ys);
        s += inte;
        aa += h;
    }

    s * h
}

fn trap(a: f64, b: f64, e: f64, xis2: f64, mu: f64, wxis: f64, ys: f64) -> f64 {
    let mut h = (b - a) / 8.0;
    let mut p = trapsum(a, b, h, 0.0, xis2, mu, wxis, ys);
    let mut nc = 0;
    let mut v = 1.0;
    while ((v > e) && (nc < 10)) || (nc <= 2) {
        nc += 1;
        let q = trapsum(a, b, h, h / 2.0, xis2, mu, wxis, ys);
        if q.abs() > 0.0 {
            v = (p / q - 1.0).abs();
        } else {
            v = 0.0;
        }
        h /= 2.0;
        p = (p + q) / 2.0;
    }

    p
}

fn integrand(
    theta: f64,
    b0: &mut f64,
    xis2: f64,
    mu: f64,
    wxis: f64,
    ys: f64,
) -> f64 {
    let eps: f64 = 1.0e-16;
    let lneps = eps.ln();

    if theta > (*b0) {
        0.0
    } else if theta.abs() < 1.0e-10 {
        let rtheta = (1.0 + wxis) / (2.0 * ys);
        let theta2 = theta * theta;
        let psitheta = -wxis * theta2 * 0.5;

        rtheta / (1.0 - rtheta) * (mu * psitheta).exp()
    } else {
        let theta2 = theta * theta;
        let sintheta = theta.sin();
        let costheta = theta.cos();
        let ts = theta / sintheta;
        let s2 = sintheta * sintheta;
        let wx = (ts * ts + xis2).sqrt();
        let xminsinxtheta = xminsinx(theta);
        let mut p = xminsinxtheta * theta2 * ts / 6.0;
        let term1 =
            (p * (ts + 1.0) - theta2 - s2 * xis2) / (costheta * wx + wxis);
        p = p * (1.0 + (ts + 1.0) / (wx + wxis)) / (1.0 + wxis);
        let term2 = -logoneplusx(p);
        p = term1 + term2;
        let psitheta = p;
        let mut f = mu * psitheta;
        if f > lneps {
            f = f.exp();
        } else {
            f = 0.0;
            (*b0) = theta.min(*b0);
        }
        let rtheta = (ts + wx) / (2.0 * ys);
        let sinth = (theta / 2.0).sin();
        p = (2.0 * theta * sinth * sinth
            - xminsinxtheta * theta2 * theta / 6.0)
            / (2.0 * ys * s2);
        let dr = p * (1.0 + ts / wx);
        p = (dr * sintheta + (costheta - rtheta) * rtheta)
            / (rtheta * (rtheta - 2.0 * costheta) + 1.0);
        let ft = p;

        f * ft
    }
}

/// Computes the series expansion for Q.
fn qser(mu: f64, x: f64, y: f64) -> CdfOutput {
    let cdf_result = cdf_gam_c(1, mu, y);
    let mut q = cdf_result.q;
    let mut ierr = cdf_result.ierr;
    let mut q0 = q;
    let lh0 = mu * y.ln() - y - loggam(mu + 1.0);
    if (lh0 > DWARF.ln()) && (x < 100.0) {
        let mut h0 = lh0.exp();
        let mut n = 0;
        let xy = x * y;
        let delta = EPSS / 100.0;
        while (q0 / q > delta) && (n < 1000) {
            q0 = x * (q0 + h0) / ((n as f64) + 1.0);
            h0 = xy * h0 / (((n as f64) + 1.0) * (mu + (n as f64) + 1.0));
            q += q0;
            n += 1;
        }
        q *= (-x).exp();
        let p = 1.0 - q;

        CdfOutput { p, q, ierr: 0 }
    } else {
        // Computing Q forward
        let x1 = y;
        let mut s = 0.0;
        let mut k = 0;
        let mut m = 0;
        while (k < 10000) && (m == 0) {
            let a = mu + k as f64;
            let cdf_result = cdf_gam_c(1, a, x1);
            let _p1 = cdf_result.p;
            let q1 = cdf_result.q;
            ierr = cdf_result.ierr;
            let t = dompart(k as f64, x, false) * q1;
            s += t;
            k += 1;
            if (s == 0.0) && (k > 150) {
                m = 1;
            }
            if s > 0.0 && ((t / s) < 1.0e-16) && (k > 10) {
                m = 1;
            }
        }
        if ierr == 0 {
            let q = s;
            let p = 1.0 - q;

            CdfOutput { p, q, ierr: 0 }
        } else {
            CdfOutput {
                p: 1.0,
                q: 0.0,
                ierr: 1,
            }
        }
    }
}

/// Computes backward the series expansion for P
fn pser(mu: f64, x: f64, y: f64) -> CdfOutput {
    let xy = x * y;
    let nnmax = startingpser(mu, x, y);
    let mut n = 1 + nnmax;
    let lh0 = -x - y + (n as f64) * x.ln() + ((n as f64) + mu) * y.ln()
        - loggam(mu + (n as f64) + 1.0)
        - loggam((n as f64) + 1.0);
    if lh0 < DWARF.ln() {
        let x1 = y;
        let expo = (-x).exp();
        let mut s = 0.0;
        let mut k = startingpser(mu, x, y) + 1;
        let mut ierr = 0;
        while (k > 0) && (ierr == 0) {
            let a = mu + (k as f64);
            let facto = factor(x, k);
            let cdf_result = cdf_gam_c(1, a, x1);
            let p1 = cdf_result.p;
            let _q1 = cdf_result.q;
            ierr = cdf_result.ierr;
            let t = facto * p1;
            s += t;
            k -= 1;
        }

        if ierr == 0 {
            let cdf_result = cdf_gam_c(1, mu, x1);
            let p1 = cdf_result.p;
            let _q1 = cdf_result.q;
            let _ierr = cdf_result.ierr;
            s += p1;
            let p = s * expo;
            let q = 1.0 - p;

            CdfOutput { p, q, ierr: 0 }
        } else {
            CdfOutput {
                p: 0.0,
                q: 1.0,
                ierr: 1,
            }
        }
    } else {
        let mut h0 = lh0.exp();
        let cdf_result = cdf_gam_c(1, mu + (n as f64), y);
        let p = cdf_result.p;
        let ierr = cdf_result.ierr;

        if ierr == 0 {
            let mut p1 =
                p * (-x + (n as f64) * x.ln() - loggam((n as f64) + 1.0)).exp();
            let mut p = 0.0;
            while n > 0 {
                h0 = h0 * (n as f64) * (mu + (n as f64)) / xy;
                p1 = (n as f64) * p1 / x + h0;
                p += p1;
                n -= 1;
            }
            let q = 1.0 - p;

            CdfOutput { p, q, ierr: 0 }
        } else {
            CdfOutput {
                p: 0.0,
                q: 1.0,
                ierr: 1,
            }
        }
    }
}

/// Computes the backward recursion for P
fn prec(mu: f64, x: f64, y: f64) -> CdfOutput {
    let b = 1.0;
    let nu = y - x + b * b + b * (2.0 * (x + y) + b * b).sqrt();

    let n1 = mu as usize;
    let n2 = nu as usize + 2;
    let n3 = n2 - n1;

    let mur = mu + (n3 as f64);
    let xi = 2.0 * (x * y).sqrt();
    let mut cmu = (y / x).sqrt() * fc(mur, xi);

    // Numerical quadrature
    let cdf_result = cdf_pq_trap(mur + 1.0, x, y);
    let mut p1 = cdf_result.p;
    let mut ierr = cdf_result.ierr;
    let cdf_result = cdf_pq_trap(mur + 0.0, x, y);
    let mut p0 = cdf_result.p;
    let _q = cdf_result.q;
    if cdf_result.ierr != 0 {
        ierr = cdf_result.ierr;
    }

    if ierr == 0 {
        let mut p = 0.0;
        for n in 0..n3 {
            p = ((1.0 + cmu) * p0 - p1) / cmu;
            p1 = p0;
            p0 = p;
            cmu = y / (mur - (n as f64) - 1.0 + x * cmu);
        }

        let q = 1.0 - p;

        CdfOutput { p, q, ierr: 0 }
    } else {
        CdfOutput {
            p: 0.0,
            q: 1.0,
            ierr: 1,
        }
    }
}

/// Computes the forward recursion for Q
fn qrec(mu: f64, x: f64, y: f64) -> CdfOutput {
    let b = 1.0;
    let nu = y - x + b * (b - (2.0 * (x + y) + b * b).sqrt());

    if nu < 5.0 {
        if x < 200.0 {
            qser(mu, x, y)
        } else {
            prec(mu, x, y)
        }
    } else {
        let n1 = mu as usize;
        let n2 = nu as usize - 1;
        let n3 = n1 - n2;

        let mur = mu - (n3 as f64);
        let xi = 2.0 * (x * y).sqrt();
        let mut cmu = [0.0_f64; 301];
        cmu[0] = (y / x).sqrt() * fc(mu, xi);
        for n in 1..=n3 {
            cmu[n] = y / (mu - (n as f64) + x * cmu[n - 1]);
        }

        // Numerical quadrature
        let cdf_result = cdf_pq_trap(mur - 1.0, x, y);
        let mut q0 = cdf_result.q;
        let mut ierr = cdf_result.ierr;
        let cdf_result = cdf_pq_trap(mur + 0.0, x, y);
        let mut q1 = cdf_result.q;
        if cdf_result.ierr != 0 {
            ierr = cdf_result.ierr;
        }

        if ierr == 0 {
            let mut q = 0.0;
            for n in 1..=n3 {
                let c = cmu[n3 + 1 - n];
                q = (1.0 + c) * q1 - c * q0;
                q0 = q1;
                q1 = q;
            }
            let p = 1.0 - q;

            CdfOutput { p, q, ierr: 0 }
        } else {
            CdfOutput {
                p: 1.0,
                q: 0.0,
                ierr: 1,
            }
        }
    }
}

/// Computes P_(mu)(x,y) and Q_(mu)(x,y)
/// using an asymptotic expansion for large xi.
fn pqasyxy(mu: f64, x: f64, y: f64) -> CdfOutput {
    let s = if y >= x { 1.0 } else { -1.0 };
    let delta = EPSS / 100.0;
    let xi = 2.0 * (x * y).sqrt();
    let sqxi = xi.sqrt();
    let rho = (y / x).sqrt();
    let sigmaxi = ((y - x) * (y - x)) / (x + y + xi);
    let mulrho = mu * rho.ln();

    if (mulrho < DWARF.ln()) || (mulrho > GIANT.ln()) {
        if s == 1.0 {
            CdfOutput {
                p: 1.0,
                q: 0.0,
                ierr: 1,
            }
        } else {
            CdfOutput {
                p: 0.0,
                q: 1.0,
                ierr: 1,
            }
        }
    } else {
        let rhomu = mulrho.exp();
        let er = errorfunction(sigmaxi.sqrt(), true, true);
        let psi0 = 0.5 * rhomu * er / rho.sqrt();
        let nu = 2.0 * mu - 1.0;
        let rhom = nu * (rho - 1.0);
        let rhop = 2.0 * (rho + 1.0);
        let mu2 = 4.0 * mu * mu;

        let mut c = s * rhomu / (8.0 * PI).sqrt();
        let mut an = sqxi;
        let mut n = 0;
        let mut n0 = 100;

        let mut bn = [1.0_f64; 101];
        bn[0] = 1.0;

        while ((bn[n]).abs() > delta) && (n < n0) {
            n += 1;
            let tnm1 = (2 * n - 1) as f64;
            an = (mu2 - tnm1 * tnm1) * an / (8.0 * (n as f64) * xi);
            bn[n] = an * (rhom - (n as f64) * rhop)
                / (rho * (nu + 2.0 * (n as f64)));
        }
        n0 = n;

        let mut nrec = sigmaxi as usize + 1;
        if nrec > n0 {
            nrec = n0;
        }

        let mut phin = [0.0_f64; 101];
        phin[nrec] = (((nrec as f64) - 0.5) * sigmaxi.ln()).exp()
            * ignega(nrec as i32, sigmaxi, EPSS);
        for n in nrec + 1..=n0 {
            phin[n] = (-sigmaxi * phin[n - 1] + 1.0) / ((n as f64) - 0.5);
        }

        for n in (1..=nrec - 1).rev() {
            phin[n] = (1.0 - ((n as f64) + 0.5) * phin[n + 1]) / sigmaxi;
        }

        let mut pq = psi0;
        for n in 1..=n0 {
            c = -c;
            let psi = c * bn[n] * phin[n];
            pq += psi;
        }
        pq *= (-sigmaxi).exp();

        if s == 1.0 {
            let q = pq;
            let p = 1.0 - q;

            CdfOutput { p, q, ierr: 0 }
        } else {
            let p = pq;
            let q = 1.0 - p;

            CdfOutput { p, q, ierr: 0 }
        }
    }
}

/// Computes P_(mu)(x, y) and Q_(mu)(x, y) by using
/// the large mu asymptotic expansion
fn pqasymu(mu0: f64, x0: f64, y0: f64) -> CdfOutput {
    let mu = mu0 - 1.0;
    let x = x0 / mu;
    let y = y0 / mu;
    let zeta = zetaxy(x, y);
    let a = if zeta < 0.0 { 1 } else { -1 };
    let u = 1.0 / (2.0 * x + 1.0).sqrt();

    let mut fjk = [[0.0; 17]; 17];
    fjkproc16(u, &mut fjk);

    let zeta = (a as f64) * zeta;
    let r = zeta * (mu / 2.0).sqrt();
    let mut psik = [0.0_f64; 18];
    psik[1] = (PI / (2.0 * mu)).sqrt() * errorfunction(-r, true, false);
    let mut s = psik[1];
    let lexpor = -mu * 0.5 * zeta * zeta;
    if (lexpor < DWARF.ln()) || (lexpor > GIANT.ln()) {
        if a == 1 {
            CdfOutput {
                p: 1.0,
                q: 0.0,
                ierr: 1,
            }
        } else {
            CdfOutput {
                p: 0.0,
                q: 1.0,
                ierr: 1,
            }
        }
    } else {
        let r = lexpor.exp();
        psik[0] = 0.0;
        let mut muk = [0.0_f64; 17];
        muk[0] = 1.0;
        let mut bk = s;
        let mut k = 1;
        let mut zetaj = 1.0;
        while ((bk / s).abs() > 1.0e-30) && (k <= 16) {
            muk[k] = mu * muk[k - 1];
            psik[k + 1] = ((k - 1) as f64 * psik[k - 1] + r * zetaj) / mu;
            bk = 0.0;
            let mut b = 1;
            zetaj *= -zeta;

            for j in 0..=k {
                let t = if (a == -1) && (b == -1) { -1 } else { 1 };
                b = -b;
                bk += (t as f64) * fjk[j][k - j] * psik[j + 1] / muk[k - j];
            }

            s += bk;
            k += 1;
        }

        let r = (mu / (2.0 * PI)).sqrt() * s;
        if a == 1 {
            let q = r;
            let p = 1.0 - q;

            CdfOutput { p, q, ierr: 0 }
        } else {
            let p = r;
            let q = 1.0 - p;

            CdfOutput { p, q, ierr: 0 }
        }
    }
}

/// Computes the P and Q using an integral representation which
/// is approximated using the trapezoidal rule
fn cdf_pq_trap(mu: f64, x: f64, y: f64) -> CdfOutput {
    let xs = x / mu;
    let ys = y / mu;
    let xis2 = 4.0 * xs * ys;
    let wxis = (1.0 + xis2).sqrt();
    let a = 0.0;
    let b = 3.0;
    let epstrap = 1.0e-13;
    let pq = trap(a, b, epstrap, xis2, mu, wxis, ys);
    let zeta = zetaxy(xs, ys);
    if (-mu * 0.5 * zeta * zeta) < DWARF.ln() {
        if y > x + mu {
            CdfOutput {
                p: 1.0,
                q: 0.0,
                ierr: 1,
            }
        } else {
            CdfOutput {
                p: 0.0,
                q: 1.0,
                ierr: 1,
            }
        }
    } else {
        let pq = pq * (-mu * 0.5 * zeta * zeta).exp() / PI;
        if zeta < 0.0 {
            let q = pq;
            let p = 1.0 - q;

            CdfOutput { p, q, ierr: 0 }
        } else {
            let p = -pq;
            let q = 1.0 - p;

            CdfOutput { p, q, ierr: 0 }
        }
    }
}

/// 0 < mu < 1:
/// Computes 1 step backward of the TTRR
fn rec1step(mu: f64, x: f64, y: f64) -> CdfOutput {
    let mu2 = mu + 2.0;
    let mu1 = mu + 1.0;
    let xi = 2.0 * (x * y).sqrt();
    let cmu = (y / x).sqrt() * fc(mu1, xi);
    // function values for the TTRR
    if y < x + mu {
        let CdfOutput {
            p: p1, ierr: ierr1, ..
        } = cdf_gam_nc(1, mu2, x, y);
        let CdfOutput {
            p: p0, ierr: ierr2, ..
        } = cdf_gam_nc(1, mu1, x, y);

        if (ierr1 == 0) && (ierr2 == 0) {
            let p = ((1.0 + cmu) * p0 - p1) / cmu;
            let q = 1.0 - p;

            CdfOutput { p, q, ierr: 0 }
        } else {
            CdfOutput {
                p: 0.0,
                q: 1.0,
                ierr: 1,
            }
        }
    } else {
        let CdfOutput {
            q: q1, ierr: ierr1, ..
        } = cdf_gam_nc(1, mu2, x, y);
        let CdfOutput {
            q: q0, ierr: ierr2, ..
        } = cdf_gam_nc(1, mu1, x, y);

        if (ierr1 == 0) && (ierr2 == 0) {
            let q = ((1.0 + cmu) * q0 - q1) / cmu;
            let p = 1.0 - q;

            CdfOutput { p, q, ierr: 0 }
        } else {
            CdfOutput {
                p: 1.0,
                q: 0.0,
                ierr: 1,
            }
        }
    }
}

/// to compute hyperbolic function sinh (x)}
fn sinh(x: f64, eps: f64) -> f64 {
    let ax = x.abs();
    if x == 0.0 {
        0.0
    } else if ax < 0.12 {
        let e = eps * 0.1;
        let x2 = x * x;
        let mut y = 1.0;
        let mut t = 1.0;
        let mut u = 0.0;
        let mut k = 1.0;

        while t > e {
            u = u + 8.0 * k - 2.0;
            k += 1.0;
            t = t * x2 / u;
            y += t;
        }

        x * y
    } else if ax < 0.36 {
        let t = sinh(x * 0.333333333333333333333333333333, eps);

        t * (3.0 + 4.0 * t * t)
    } else {
        let t = x.exp();

        (t - 1.0 / t) * 0.5
    }
}

/// computes (exp(x)-1)/x
fn exmin1(x: f64, eps: f64) -> f64 {
    if x == 0.0 {
        1.0
    } else if !(-0.69..=0.4).contains(&x) {
        (x.exp() - 1.0) / x
    } else {
        let t = x * 0.5;

        t.exp() * sinh(t, eps) / t
    }
}

/// computes (exp(x)-1-x)/(0.5*x*x)
fn exmin1minx(x: f64, eps: f64) -> f64 {
    if x == 0.0 {
        1.0
    } else if x.abs() > 0.9 {
        (x.exp() - 1.0 - x) / (x * x * 0.5)
    } else {
        let t = sinh(x * 0.5, eps);
        let t2 = t * t;

        (2.0 * t2 + (2.0 * t * (1.0 + t2).sqrt() - x)) / (x * x * 0.5)
    }
}

/// x > -1; lnec := ln1 := ln(1 + x) - x
fn lnec(x: f64) -> f64 {
    let z = logoneplusx(x);
    let y0 = z - x;
    let e2 = exmin1minx(z, MACHTOL);
    let s = e2 * z * z / 2.0;
    let r = (s + y0) / (s + 1.0 + z);

    y0 - r * (6.0 - r) / (6.0 - 4.0 * r)
}

fn alfa(x: f64) -> f64 {
    if x > 0.25 {
        x + 0.25
    } else if x >= DWARF {
        let lnx = x.ln();

        #[allow(clippy::approx_constant)]
        let result = -0.6931 / lnx;

        result
    } else {
        #[allow(clippy::approx_constant)]
        let result = -0.6931 / DWARF.ln();

        result
    }
}

/// dompart is approx. of  x^a*exp(-x)/gamma(a+1)
fn dompart(a: f64, x: f64, qt: bool) -> f64 {
    let lnx = x.ln();
    let r = if a <= 1.0 {
        -x + a * lnx
    } else {
        let r = if x == a {
            0.0
        } else {
            let la = x / a;

            a * (1.0 - la + la.ln())
        };

        #[allow(clippy::approx_constant)]
        let value = r - 0.5 * (6.2832 * a).ln();

        value
    };
    let dp = if r < EXP_LOW { 0.0 } else { r.exp() };

    if qt {
        dp
    } else {
        if a < 8.0 {
            (a * lnx - x).exp() / gamma(a + 1.0)
        } else {
            let value = 1.0 / ((a * TWO_PI).sqrt() * gamstar(a));
            let lambda = x / a;
            if (lambda > 0.3) && (lambda < 2.36) {
                let mu = lambda - 1.0;
                let c = lnec(mu);

                value * (a * c).exp()
            } else {
                value * (a * lambda.ln() + a - x).exp()
            }
        }
    }
}

fn pqasymp(a: f64, x: f64, dp: f64, p: bool) -> f64 {
    if dp == 0.0 {
        if p { 0.0 } else { 1.0 }
    } else {
        let s = if p { -1 } else { 1 };
        let mu = (x - a) / a;
        let mut y = -lnec(mu);
        let mut eta = if y < 0.0 { 0.0 } else { (2.0 * y).sqrt() };
        y *= a;
        let mut v = y.abs().sqrt();
        if mu < 0.0 {
            eta = -eta;
            v = -v;
        }
        let u = 0.5 * errorfunction((s as f64) * v, true, false);
        v = (s as f64) * (-y).exp() * saeta(a, eta) / (2.0 * PI * a).sqrt();

        u + v
    }
}

fn saeta(a: f64, eta: f64) -> f64 {
    let eps = EPSS;
    let fm = [
        1.0,
        -1.0 / 3.0,
        1.0 / 12.0,
        -2.0 / 135.0,
        1.0 / 864.0,
        1.0 / 2835.0,
        -139.0 / 777600.0,
        1.0 / 25515.0,
        -571.0 / 261273600.0,
        -281.0 / 151559100.0,
        8.29671134095308601e-7,
        -1.76659527368260793e-7,
        6.70785354340149857e-9,
        1.02618097842403080e-8,
        -4.38203601845335319e-9,
        9.14769958223679023e-10,
        -2.55141939949462497e-11,
        -5.83077213255042507e-11,
        2.43619480206674162e-11,
        -5.02766928011417559e-12,
        1.10043920319561347e-13,
        3.37176326240098538e-13,
        -1.39238872241816207e-13,
        2.85348938070474432e-14,
        -5.13911183424257258e-16,
        -1.97522882943494428e-15,
        8.09952115670456133e-16,
    ];

    let mut bm = [0.0; 27];
    bm[25] = fm[26];
    bm[24] = fm[25];
    for m in (1..=24).rev() {
        bm[m - 1] = fm[m] + (m as f64 + 1.0) * bm[m + 1] / a;
    }

    let mut s = bm[0];
    let mut t = s;
    let mut y = eta;
    let mut m = 1;
    while ((t / s).abs() > eps) && (m < 25) {
        t = bm[m] * y;
        s += t;
        m += 1;
        y *= eta;
    }

    s / (1.0 + bm[1] / a)
}

fn qfraction(a: f64, x: f64, dp: f64) -> f64 {
    let eps = EPSS;
    if dp == 0.0 {
        0.0
    } else {
        let mut p = 0.0;
        let mut q = (x - 1.0 - a) * (x + 1.0 - a);
        let mut r = 4.0 * (x + 1.0 - a);
        let mut s = 1.0 - a;
        let mut ro = 0.0;
        let mut t = 1.0_f64;
        let mut g = 1.0;
        while (t / g).abs() >= eps {
            p += s;
            q += r;
            r += 8.0;
            s += 2.0;
            let tau = p * (1.0 + ro);
            ro = tau / (q - tau);
            t *= ro;
            g += t;
        }

        (a / (x + 1.0 - a)) * g * dp
    }
}

fn qtaylor(a: f64, x: f64, dp: f64) -> f64 {
    let eps = EPSS;
    let lnx = x.ln();
    if dp == 0.0 {
        0.0
    } else {
        let r = a * lnx;
        let mut q = r * exmin1(r, eps);
        let s = a * (1.0 - a) * auxgam(a);
        q *= 1.0 - s;
        let u = s - q;
        let mut p = a * x;
        let mut q = a + 1.0;
        let mut r = a + 3.0;
        let mut t = 1.0_f64;
        let mut v = 1.0;
        while (t / v).abs() > eps {
            p += x;
            q += r;
            r += 2.0;
            t = -p * t / q;
            v += t;
        }
        v = a * (1.0 - s) * ((a + 1.0) * lnx).exp() * v / (a + 1.0);

        u + v
    }
}

fn ptaylor(a: f64, x: f64, dp: f64) -> f64 {
    let eps = EPSS;
    if dp == 0.0 {
        0.0
    } else {
        let mut p = 1.0;
        let mut c = 1.0;
        let mut r = a;
        while (c / p) > eps {
            r += 1.0;
            c = x * c / r;
            p += c;
        }

        p * dp
    }
}

fn eps1(eta: f64) -> f64 {
    if eta.abs() < 1.0 {
        const AK: [f64; 5] = [
            -3.333333333438e-1,
            -2.070740359969e-1,
            -5.041806657154e-2,
            -4.923635739372e-3,
            -4.293658292782e-5,
        ];
        const BK: [f64; 5] = [
            1.000000000000e+0,
            7.045554412463e-1,
            2.118190062224e-1,
            3.048648397436e-2,
            1.605037988091e-3,
        ];

        ratfun(eta, &AK, &BK)
    } else {
        let la = lambdaeta(eta);

        (eta / (la - 1.0)).ln() / eta
    }
}

fn eps2(eta: f64) -> f64 {
    if eta < -5.0 {
        let x = eta * eta;
        let lnmeta = (-eta).ln();

        (12.0 - x - 6.0 * (lnmeta * lnmeta)) / (12.0 * x * eta)
    } else if eta < -2.0 {
        const AK: [f64; 5] = [
            -1.72847633523e-2,
            -1.59372646475e-2,
            -4.64910887221e-3,
            -6.06834887760e-4,
            -6.14830384279e-6,
        ];
        const BK: [f64; 5] = [
            1.00000000000e+0,
            7.64050615669e-1,
            2.97143406325e-1,
            5.79490176079e-2,
            5.74558524851e-3,
        ];

        ratfun(eta, &AK, &BK)
    } else if eta < 2.0 {
        const AK: [f64; 5] = [
            -1.72839517431e-2,
            -1.46362417966e-2,
            -3.57406772616e-3,
            -3.91032032692e-4,
            2.49634036069e-6,
        ];
        const BK: [f64; 5] = [
            1.00000000000e+0,
            6.90560400696e-1,
            2.49962384741e-1,
            4.43843438769e-2,
            4.24073217211e-3,
        ];

        ratfun(eta, &AK, &BK)
    } else if eta < 1000.0 {
        const AK: [f64; 5] = [
            9.99944669480e-1,
            1.04649839762e+2,
            8.57204033806e+2,
            7.31901559577e+2,
            4.55174411671e+1,
        ];
        const BK: [f64; 5] = [
            1.00000000000e+0,
            1.04526456943e+2,
            8.23313447808e+2,
            3.11993802124e+3,
            3.97003311219e+3,
        ];
        let x = 1.0 / eta;

        ratfun(x, &AK, &BK) / (-12.0 * eta)
    } else {
        -1.0 / (12.0 * eta)
    }
}

fn eps3(eta: f64) -> f64 {
    if eta < -8.0 {
        let x = eta * eta;
        let y = (-eta).ln() / eta;

        (-30.0 + eta * y * (6.0 * x * y * y - 12.0 + x)) / (12.0 * eta * x * x)
    } else if eta < -4.0 {
        const AK: [f64; 5] = [
            4.95346498136e-2,
            2.99521337141e-2,
            6.88296911516e-3,
            5.12634846317e-4,
            -2.01411722031e-5,
        ];
        const BK: [f64; 5] = [
            1.00000000000e+0,
            7.59803615283e-1,
            2.61547111595e-1,
            4.64854522477e-2,
            4.03751193496e-3,
        ];

        ratfun(eta, &AK, &BK) / (eta * eta)
    } else if eta < -2.0 {
        const AK: [f64; 5] = [
            4.52313583942e-3,
            1.20744920113e-3,
            -7.89724156582e-5,
            -5.04476066942e-5,
            -5.35770949796e-6,
        ];
        const BK: [f64; 5] = [
            1.00000000000e+0,
            9.12203410349e-1,
            4.05368773071e-1,
            9.01638932349e-2,
            9.48935714996e-3,
        ];

        ratfun(eta, &AK, &BK)
    } else if eta < 2.0 {
        const AK: [f64; 5] = [
            4.39937562904e-3,
            4.87225670639e-4,
            -1.28470657374e-4,
            5.29110969589e-6,
            1.57166771750e-7,
        ];
        const BK: [f64; 5] = [
            1.00000000000e+0,
            7.94435257415e-1,
            3.33094721709e-1,
            7.03527806143e-2,
            8.06110846078e-3,
        ];

        ratfun(eta, &AK, &BK)
    } else if eta < 10.0 {
        const AK: [f64; 5] = [
            -1.14811912320e-3,
            -1.12850923276e-1,
            1.51623048511e+0,
            -2.18472031183e-1,
            7.30002451555e-2,
        ];
        const BK: [f64; 5] = [
            1.00000000000e+0,
            1.42482206905e+1,
            6.97360396285e+1,
            2.18938950816e+2,
            2.77067027185e+2,
        ];
        let x = 1.0 / eta;

        ratfun(x, &AK, &BK) / (eta * eta)
    } else if eta < 100.0 {
        const AK: [f64; 5] = [
            -1.45727889667e-4,
            -2.90806748131e-1,
            -1.33085045450e+1,
            1.99722374056e+2,
            -1.14311378756e+1,
        ];
        const BK: [f64; 5] = [
            1.00000000000e+0,
            1.39612587808e+2,
            2.18901116348e+3,
            7.11524019009e+3,
            4.55746081453e+4,
        ];
        let x = 1.0 / eta;

        ratfun(x, &AK, &BK) / (eta * eta)
    } else {
        let eta3 = eta * eta * eta;

        -(eta).ln() / (12.0 * eta3)
    }
}

/// lambdaeta is the positive number satisfying
/// eta^2 / 2 = lambda - 1 - ln(lambda)
/// with sign(lambda - 1) = sign(eta);
fn lambdaeta(eta: f64) -> f64 {
    let s = eta * eta * 0.5;

    let mut la = if eta == 0.0 {
        1.0
    } else if eta < -1.0 {
        let r = (-1.0 - s).exp();
        const AK: [f64; 6] = [
            1.0,
            1.0,
            1.5,
            2.66666666666666666666666666667,
            5.20833333333333333333333333333,
            10.8,
        ];

        r * (AK[0]
            + r * (AK[1] + r * (AK[2] + r * (AK[3] + r * (AK[4] + r * AK[5])))))
    } else if eta < 1.0 {
        let ak = [
            1.0,
            0.333333333333333333333333333333,
            0.0277777777777777777777777777778,
            -0.00370370370370370370370370370370,
            0.000231481481481481481481481481481,
            0.0000587889476778365667254556143445,
        ];
        let r = eta;

        1.0 + r
            * (ak[0]
                + r * (ak[1]
                    + r * (ak[2] + r * (ak[3] + r * (ak[4] + r * ak[5])))))
    } else {
        let r = 11.0 + s;
        let l = r.ln();
        let la = r + l;
        let r = 1.0 / r;
        let l2 = l * l;
        let l3 = l2 * l;
        let l4 = l3 * l;
        let l5 = l4 * l;

        let ak = [
            1.0,
            (2.0 - l) * 0.5,
            (-9.0 * l + 6.0 + 2.0 * l2) * 0.166666666666666666666666666667,
            -(3.0 * l3 + 36.0 * l - 22.0 * l2 - 12.0)
                * 0.0833333333333333333333333333333,
            (60.0 + 350.0 * l2 - 30.0 * l - 125.0 * l3 + 12.0 * l4)
                * 0.0166666666666666666666666666667,
            -(-120.0 - 274.0 * l4 + 900.0 * l - 1700.0 * l2
                + 1125.0 * l3
                + 20.0 * l5)
                * 0.00833333333333333333333333333333,
        ];

        la + l
            * r
            * (ak[0]
                + r * (ak[1]
                    + r * (ak[2] + r * (ak[3] + r * (ak[4] + r * ak[5])))))
    };

    if ((eta > -3.5) && (eta < -0.03)) || ((eta > 0.03) && (eta < 100.0)) {
        let mut r = 1.0;
        let mut q = la;
        while r > 1.0e-8 {
            la = q * (s + q.ln()) / (q - 1.0);
            r = (q / la - 1.0).abs();
            q = la;
        }
    }

    la
}

/// function g in ln(Gamma(1+x)) = x * (1-x) * g(x), 0 <= x <= 1
fn auxloggam(x: f64) -> f64 {
    if x < -1.0 {
        GIANT
    } else if x.abs() <= DWARF {
        -EUL_MASC
    } else if (x - 1.0).abs() <= MACHTOL {
        EUL_MASC - 1.0
    } else if x < 0.0 {
        -(x * (1.0 + x) * auxloggam(x + 1.0) + logoneplusx(x)) / (x * (1.0 - x))
    } else if x < 1.0 {
        const AK: [f64; 26] = [
            -0.98283078605877425496,
            0.7611416167043584304e-1,
            -0.843232496593277796e-2,
            0.107949372632860815e-2,
            -0.14900748003692965e-3,
            0.2151239988855679e-4,
            -0.319793298608622e-5,
            0.48516930121399e-6,
            -0.7471487821163e-7,
            0.1163829670017e-7,
            -0.182940043712e-8,
            0.28969180607e-9,
            -0.4615701406e-10,
            0.739281023e-11,
            -0.118942800e-11,
            0.19212069e-12,
            -0.3113976e-13,
            0.506284e-14,
            -0.82542e-15,
            0.13491e-15,
            -0.2210e-16,
            0.363e-17,
            -0.60e-18,
            0.98e-19,
            -0.2e-19,
            0.3e-20,
        ];
        let t = 2.0 * x - 1.0;

        chepolsum(25, t, &AK)
    } else if x < 1.5 {
        (logoneplusx(x - 1.0) + (x - 1.0) * (2.0 - x) * auxloggam(x - 1.0))
            / (x * (1.0 - x))
    } else {
        (x.ln() + (x - 1.0) * (2.0 - x) * auxloggam(x - 1.0)) / (x * (1.0 - x))
    }
}

/// function g in 1/gamma(x+1) = 1 + x * (x-1) * g(x), -1 <= x <= 1
fn auxgam(x: f64) -> f64 {
    if x < 0.0 {
        -(1.0 + (1.0 + x) * (1.0 + x) * auxgam(1.0 + x)) / (1.0 - x)
    } else {
        const DR: [f64; 18] = [
            -1.013609258009865776949,
            0.784903531024782283535e-1,
            0.67588668743258315530e-2,
            -0.12790434869623468120e-2,
            0.462939838642739585e-4,
            0.43381681744740352e-5,
            -0.5326872422618006e-6,
            0.172233457410539e-7,
            0.8300542107118e-9,
            -0.10553994239968e-9,
            0.39415842851e-11,
            0.362068537e-13,
            -0.107440229e-13,
            0.5000413e-15,
            -0.62452e-17,
            -0.5185e-18,
            0.347e-19,
            -0.9e-21,
        ];
        let t = 2.0 * x - 1.0;

        chepolsum(17, t, &DR)
    }
}

/// ln(gamma(1+x)), -1 <= x <= 1
fn lngam1(x: f64) -> f64 {
    -logoneplusx(x * (x - 1.0) * auxgam(x))
}

/// Stirling series, function corresponding with
/// asymptotic series for log(gamma(x))
/// that is:  1/(12x)-1/(360x**3)...; x >= 3
fn stirling(x: f64) -> f64 {
    if x < DWARF {
        GIANT
    } else if x < 1.0 {
        lngam1(x) - (x + 0.5) * x.ln() + x - LN_SQRT_TWO_PI
    } else if x < 2.0 {
        lngam1(x - 1.0) - (x - 0.5) * x.ln() + x - LN_SQRT_TWO_PI
    } else if x < 3.0 {
        lngam1(x - 2.0) - (x - 0.5) * x.ln() + x - LN_SQRT_TWO_PI
            + (x - 1.0).ln()
    } else if x < 12.0 {
        const A: [f64; 18] = [
            1.996379051590076518221,
            -0.17971032528832887213e-2,
            0.131292857963846713e-4,
            -0.2340875228178749e-6,
            0.72291210671127e-8,
            -0.3280997607821e-9,
            0.198750709010e-10,
            -0.15092141830e-11,
            0.1375340084e-12,
            -0.145728923e-13,
            0.17532367e-14,
            -0.2351465e-15,
            0.346551e-16,
            -0.55471e-17,
            0.9548e-18,
            -0.1748e-18,
            0.332e-19,
            -0.58e-20,
        ];
        let z = 18.0 / (x * x) - 1.0;

        chepolsum(17, z, &A) / (12.0 * x)
    } else {
        let z = 1.0 / (x * x);
        if x < 1000.0 {
            const C: [f64; 7] = [
                0.25721014990011306473e-1,
                0.82475966166999631057e-1,
                -0.25328157302663562668e-2,
                0.60992926669463371e-3,
                -0.33543297638406e-3,
                0.250505279903e-3,
                0.30865217988013567769,
            ];

            (((((C[5] * z + C[4]) * z + C[3]) * z + C[2]) * z + C[1]) * z
                + C[0])
                / (C[6] + z)
                / x
        } else {
            (((-z * 0.000595238095238095238095238095238
                + 0.000793650793650793650793650793651)
                * z
                - 0.00277777777777777777777777777778)
                * z
                + 0.0833333333333333333333333333333)
                / x
        }
    }
}

fn ratfun(x: f64, ak: &[f64], bk: &[f64]) -> f64 {
    let p = ak[0] + x * (ak[1] + x * (ak[2] + x * (ak[3] + x * ak[4])));
    let q = bk[0] + x * (bk[1] + x * (bk[2] + x * (bk[3] + x * bk[4])));

    p / q
}

#[cfg(test)]
mod tests {
    use super::*;

    const XX: [f64; 18] = [
        0.3, 2.0, 8.0, 25.0, 13.0, 15.0, 47.0, 27.0, 85.0, 20.0, 13.0, 40.0,
        170.0, 35.0, 10.0, -35.3, -50.8, -100.1,
    ];
    const XLOG: [f64; 18] = [
        0.3, 2.5, 8.0, 25.0, 50.0, 110.0, 150.0, 270.0, 350.0, 420.0, 550.0,
        620.0, 700.0, 800.0, 1000.0, 10000.0, 1e5, 1.0e6,
    ];
    const XQUOT: [f64; 9] =
        [70.0, 150.0, 270.0, 350.0, 420.0, 550.0, 620.0, 700.0, 900.0];
    const _XRE: [f64; 18] = [
        -0.49, -0.45, -0.41, -0.37, -0.33, -0.3, -0.29, -0.23, -0.15, -0.05,
        0.01, 0.11, 0.19, 0.23, 0.29, 0.35, 0.39, 0.49,
    ];
    const XERR: [f64; 18] = [
        1.0e-6, 2.3e-5, 5.0e-4, 8.0e-3, 1.0e-2, 4.0e-1, 0.15, 0.25, 0.35, 0.45,
        0.55, 0.65, 0.75, 0.8, 0.9, 1.1, 1.5, 2.3,
    ];
    const XERFC: [f64; 18] = [
        6.0e-4, 5.0e-3, 4.0e-2, 0.15, 0.3, 0.5, 1.0, 2.3, 5.7, 8.0, 10.0, 11.3,
        13.0, 15.0, 17.5, 20.0, 23.1, 25.0,
    ];
    const XGAM: [f64; 18] = [
        0.01, 0.1, 2.0, 15.0, 23.0, 35.0, 47.0, 68.0, 85.0, 90.0, 103.0, 110.3,
        135.0, 145.0, 165.0, 178.0, 183.8, 185.1,
    ];
    const CHI2: [f64; 18] = [
        25.038, 20.337, 21.337, 22.337, 40.646, 44.314, 46.928, 52.620, 25.336,
        31.795, 35.563, 38.885, 41.923, 45.642, 48.290, 54.052, 76.154, 79.490,
    ];
    const AA: [f64; 18] = [
        1.3, 2.0, 8.0, 25.0, 33.0, 55.0, 67.0, 88.0, 95.0, 110.0, 123.0, 130.3,
        145.0, 155.0, 160.0, 170.0, 175.8, 190.1,
    ];
    const YGAMNC: [f64; 18] = [
        0.09, 0.5, 5.0, 25.0, 13.0, 35.0, 207.0, 308.0, 95.0, 150.0, 193.0,
        120.3, 240.0, 338.0, 255.4, 458.0, 370.6, 565.3,
    ];
    const DF: [f64; 18] = [
        1.0, 2.0, 3.0, 4.0, 5.0, 10.0, 15.0, 21.0, 23.0, 24.0, 25.0, 40.0,
        50.0, 60.0, 70.0, 80.0, 90.0, 100.0,
    ];
    const GAMF: [f64; 18] = [
        2.99156898768759062831251651590,
        1.0,
        5040.0,
        620448401733239439360000.0,
        479001600.0,
        87178291200.0,
        0.550262215981208894985030542880e58,
        403291461126605635584000000.0,
        0.331424013456535326699938757913e127,
        121645100408832000.0,
        479001600.0,
        0.203978820811974433586402817399e47,
        0.426906800900470527493925188890e305,
        0.295232799039604140847618609644e39,
        362880.0,
        0.128628685955630759461829329132e-39,
        -0.757669975475440412964344459853e-65,
        -0.686950539921145917085653489839e-157,
    ];
    const LGAMF: [f64; 18] = [
        1.09579799481807552167716814237,
        0.284682870472919159632494669686,
        8.52516136106541430016553103635,
        54.7847293981123191900933440836,
        144.565743946344886008918443063,
        405.622296161144889192464963531,
        600.009470555327428107958698075,
        1239.69396512510085354102638404,
        1698.26681412034702793266758401,
        2114.80598832674076132767192648,
        2918.21918393381036519764194712,
        3364.13028958704068780184378188,
        3883.39975194367719075042814242,
        4545.26611897037378744564638071,
        5905.22042320918121182607691236,
        82099.7174964423772726489580977,
        1051287.70897365689490085801825,
        12815504.5691476116599769717850,
    ];
    const QUOTF: [f64; 18] = [
        0.191279866569365909536723082480e-18,
        1933051680.0,
        0.892214092067974739630834591480e-44,
        7736314314664327391999.99999999,
        0.241861372772089610017657958815e-73,
        0.432916310174154276122903045895e37,
        0.201329282226402563951026429285e-102,
        0.129676211743732301043843490292e52,
        0.414826300340967133217026319726e-132,
        0.768388171129744605596294851471e66,
        0.169214986863282244691038498693e-165,
        0.353497650781202311414556750790e83,
        0.795835368134641377761547997894e-197,
        0.138985410676635723482649357557e99,
        0.317614300173566319964681877413e-229,
        0.190105937084148935022135832720e115,
        0.176449004862056359671533018609e-267,
        0.257603818175952521668065603553e134,
    ];
    const GAMSF: [f64; 18] = [
        1.2662564884936691,
        1.0337188909653103,
        1.010465651061946,
        1.003338716776976,
        1.0016680340707353,
        1.0007578607020506,
        1.000555709081633,
        1.000308689468975,
        1.0002381235202131,
        1.0001984323460138,
        1.000151526613817,
        1.0001344176237346,
        1.0001190546973973,
        1.0001041720867763,
        1.000083336802874,
        1.000008333368053,
        1.0000008333336805,
        1.000000083333337,
    ];
    const _RECIF: [f64; 18] = [
        0.5752366153174464,
        0.618764298851609,
        0.6611176010057371,
        0.7021499577513451,
        0.7417276612576872,
        0.770383183866566,
        0.7797299387539419,
        0.8335470138998202,
        0.8988895448871843,
        0.9695058258025869,
        1.005706528500385,
        1.0555253806213079,
        1.0859118778958716,
        1.097990302223796,
        1.1122955992845303,
        1.1221437264674605,
        1.1263109377869167,
        1.1287380723282776,
    ];
    const ERFF: [f64; 18] = [
        0.0000011283791670951362,
        0.00002595272083862046,
        0.0005641895365319611,
        0.009026840763750333,
        0.011283415555849616,
        0.42839235504666845,
        0.1679959714273635,
        0.2763263901682369,
        0.37938205356231025,
        0.47548171978692355,
        0.563323366325109,
        0.6420293273556719,
        0.7111556336535152,
        0.7421009647076605,
        0.7969082124228322,
        0.8802050695740817,
        0.9661051464753108,
        0.9988568234026434,
    ];
    const ERFCF: [f64; 18] = [
        0.999322972580986,
        0.9943581511799684,
        0.9548888938548753,
        0.8320040285726364,
        0.6713732405408727,
        0.4795001221869535,
        0.15729920705028516,
        0.0011431765973566523,
        7.566211621862487e-16,
        1.1224297172982928e-29,
        2.088487583762545e-45,
        1.744214847996726e-57,
        1.7395573154667244e-75,
        7.212994172451207e-100,
        3.1988638123434806e-135,
        5.3958656116079005e-176,
        4.4007693201925407e-234,
        8.300172571196522e-274,
    ];
    const SERFCF: [f64; 18] = [
        0.999323332337321,
        0.9943830104444875,
        0.9564179389949587,
        0.8509363086677636,
        0.7345993345676552,
        0.6156903441929259,
        0.42758357615580705,
        0.22674156216755917,
        0.09752280879543966,
        0.06998516620088092,
        0.05614099274382259,
        0.049735028760517254,
        0.043271921864609694,
        0.03752960638850576,
        0.03218702473823041,
        0.02817434874105132,
        0.02440097009533324,
        0.022549572432641357,
    ];
    const PX: [f64; 18] = [
        0.0021408354124610863,
        0.004678840160444474,
        0.0010967189678587038,
        0.011164780271550342,
        0.028943392521114174,
        0.0010670052225617138,
        0.0034836452814566085,
        0.011222055785401731,
        0.1515411183688647,
        0.02252140347111433,
        0.029981975385510458,
        0.03429298650044801,
        0.2053951374327591,
        0.21352533982745125,
        0.6618542306964403,
        0.7355459029101765,
        0.7322060070459085,
        0.36631122865551535,
    ];
    const QX: [f64; 18] = [
        0.9978591645875389,
        0.9953211598395555,
        0.9989032810321413,
        0.9888352197284497,
        0.9710566074788858,
        0.9989329947774382,
        0.9965163547185434,
        0.9887779442145983,
        0.8484588816311354,
        0.9774785965288857,
        0.9700180246144895,
        0.965707013499552,
        0.7946048625672408,
        0.7864746601725487,
        0.3381457693035596,
        0.26445409708982354,
        0.26779399295409145,
        0.6336887713444846,
    ];
    const PCHI: [f64; 18] = [
        0.9999994378850235,
        0.9999616401799867,
        0.9999103973772367,
        0.9998282806937289,
        0.9999998893828778,
        0.9999971094703597,
        0.999962189987467,
        0.9998441430107688,
        0.6668029364037691,
        0.867841771648278,
        0.9215028051341907,
        0.47964623933991807,
        0.2153061465507992,
        0.08524918285213509,
        0.02218562439454764,
        0.011500360923221068,
        0.14919691576007615,
        0.06485161722407413,
    ];
    const QCHI: [f64; 18] = [
        5.621149764163912E-7,
        0.00003835982001334733,
        0.00008960262276326297,
        0.00017171930627113386,
        1.1061712223443984E-7,
        0.00000289052964032204,
        0.00003781001253305702,
        0.0001558569892311312,
        0.3331970635962309,
        0.13215822835172203,
        0.07849719486580933,
        0.5203537606600819,
        0.7846938534492008,
        0.9147508171478649,
        0.9778143756054524,
        0.988499639076779,
        0.8508030842399239,
        0.9351483827759258,
    ];
    const PGAMNC: [f64; 18] = [
        0.03527081065336218,
        0.08292976551098219,
        0.048141118973927664,
        0.01180611391640901,
        6.299046673515003E-13,
        8.627569984647915E-11,
        0.9999999994199555,
        0.9999999999999999,
        2.4132952554883887E-10,
        0.0007673449723863583,
        0.029939137043083097,
        3.4774157961333044E-15,
        0.02106496280028332,
        0.9606526606255229,
        0.0004039126207882811,
        0.9999953405634416,
        0.6867702817127359,
        0.9999999999993621,
    ];
    const QGAMNC: [f64; 18] = [
        0.9647291893466379,
        0.9170702344890178,
        0.9518588810260723,
        0.988193886083591,
        0.9999999999993701,
        0.9999999999137243,
        5.800444092964824E-10,
        1.255348192821353E-16,
        0.9999999997586705,
        0.9992326550276136,
        0.970060862956917,
        0.9999999999999966,
        0.9789350371997166,
        0.03934733937447712,
        0.9995960873792117,
        0.000004659436558370328,
        0.3132297182872641,
        6.379774077622865E-13,
    ];
    const PCHINC: [f64; 18] = [
        0.04378794352755403,
        0.07752816233349241,
        0.0913586802838962,
        0.04801284038641763,
        1.7877864948853312e-7,
        0.0000025248182696349618,
        0.9999905233329209,
        0.9999999961057766,
        0.000004687234874510834,
        0.011733585435172873,
        0.08793209578935854,
        1.7159521028615862E-8,
        0.07251079555027014,
        0.8897635768196765,
        0.008471111218215789,
        0.9990910653052695,
        0.6374291107212573,
        0.999999722549898,
    ];
    const QCHINC: [f64; 18] = [
        0.956212056472446,
        0.9224718376665075,
        0.9086413197161038,
        0.9519871596135824,
        0.9999998212213506,
        0.9999974751817303,
        0.000009476667079040102,
        3.894223407052914E-9,
        0.9999953127651255,
        0.9882664145648271,
        0.9120679042106414,
        0.999999982840479,
        0.9274892044497298,
        0.11023642318032341,
        0.9915288887817842,
        0.0009089346947305406,
        0.36257088927874276,
        2.774501020116052E-7,
    ];

    // The original test program writes relative errors instead of asserting.
    // These bounds turn the same checks into Rust tests while retaining the
    // original inputs and precomputed reference values.
    const CENTRAL_RELATIVE_TOLERANCE: f64 = 1.0e-13;
    const CENTRAL_INVERSION_TOLERANCE: f64 = 1.0e-14;
    const NONCENTRAL_RELATIVE_TOLERANCE: f64 = 1.0e-11;
    const AUXILIARY_RELATIVE_TOLERANCE: f64 = 1.0e-12;

    // ****************************************************************
    //                   TEST 1
    // Test of the computed values of the central gamma distribution
    // function P(a,x) and its complementary Q(a,x):
    // comparison against precomputed values
    //      ERR means relative error
    // ****************************************************************
    #[test]
    fn test_1_central_gamma_distribution() {
        for i in 0..18 {
            let a = AA[i];
            let x = XGAM[i];
            let result = cdf_gam_c(1, a, x);

            let err1 = (1.0 - result.p / PX[i]).abs();
            let err2 = (1.0 - result.q / QX[i]).abs();

            assert_eq!(result.ierr, 0, "i={i}, a={a}, x={x}");
            assert!(
                err1 <= CENTRAL_RELATIVE_TOLERANCE,
                "i={i}, a={a}, x={x}, P={}, expected={}, ERR(P)={err1}",
                result.p,
                PX[i],
            );
            assert!(
                err2 <= CENTRAL_RELATIVE_TOLERANCE,
                "i={i}, a={a}, x={x}, Q={}, expected={}, ERR(Q)={err2}",
                result.q,
                QX[i],
            );
        }
    }

    // *********************************************************************
    //                   TEST 2
    // Test of the computed values of the central chi-square distribution
    // function P(nu,chi2) and its complementary Q(nu,chi2):
    // comparison against precomputed values
    //      ERR means relative error
    // *********************************************************************
    #[test]
    fn test_2_central_chi_square_distribution() {
        for i in 0..18 {
            let a = DF[i];
            let x = CHI2[i];
            let result = cdf_gam_c(2, a, x);

            let err1 = (1.0 - result.p / PCHI[i]).abs();
            let err2 = (1.0 - result.q / QCHI[i]).abs();

            assert_eq!(result.ierr, 0, "i={i}, nu={a}, chi2={x}");
            assert!(
                err1 <= CENTRAL_RELATIVE_TOLERANCE,
                "i={i}, nu={a}, chi2={x}, P={}, expected={}, ERR(P)={err1}",
                result.p,
                PCHI[i],
            );
            assert!(
                err2 <= CENTRAL_RELATIVE_TOLERANCE,
                "i={i}, nu={a}, chi2={x}, Q={}, expected={}, ERR(Q)={err2}",
                result.q,
                QCHI[i],
            );
        }
    }

    // ******************************************************************
    //                   TEST 3
    // Test of the inversion routine for the central gamma distribution
    // function. The accuracy of the value obtained in the inversion is
    // checked.
    // ******************************************************************
    #[test]
    fn test_3_inverse_central_gamma_distribution() {
        let mut d0 = 0.0_f64;

        for i in 0..=100 {
            let a = i as f64 * 1.2 + 0.1;

            for j in 1..=100 {
                let x = j as f64 * 0.75;
                let result = cdf_gam_c(1, a, x);

                if result.ierr == 0
                    && let Ok(xr) = inv_cdf_gam_c(1, a, result.p, result.q)
                {
                    let erxr = (1.0 - x / xr).abs();

                    if erxr > d0 {
                        d0 = erxr;
                    }
                }
            }
        }

        // Original testGammaCHI.dat:
        // Max. error (inversion): 1.0930926302998856E-15
        assert!(
            d0 <= CENTRAL_INVERSION_TOLERANCE,
            "Max. error (inversion): {d0}"
        );
    }

    // **********************************************************************
    //                   TEST 4
    // Test of the inversion routine for the central chi-square distribution
    // function. The accuracy of the value obtained in the inversion is
    // checked.
    // **********************************************************************
    #[test]
    fn test_4_inverse_central_chi_square_distribution() {
        let mut d0 = 0.0_f64;

        for i in 1..=100 {
            let a = i as f64;

            for j in 1..=100 {
                let x = j as f64 * 0.75;
                let result = cdf_gam_c(2, a, x);

                if result.ierr == 0
                    && let Ok(xr) = inv_cdf_gam_c(2, a, result.p, result.q)
                {
                    let erxr = (1.0 - x / xr).abs();

                    if erxr > d0 {
                        d0 = erxr;
                    }
                }
            }
        }

        // Original testGammaCHI.dat:
        // Max. error (inversion): 1.1842198228972922E-15
        assert!(
            d0 <= CENTRAL_INVERSION_TOLERANCE,
            "Max. error (inversion): {d0}"
        );
    }

    // ********************************************************************
    //                   TEST 5
    // Test of the computed values of the noncentral gamma distribution
    // function P(mu,x,y) and its complementary Q(mu,x,y):
    // comparison against precomputed values
    //      ERR means relative error
    // ********************************************************************
    #[test]
    fn test_5_noncentral_gamma_distribution() {
        for i in 0..18 {
            let mu = AA[i];
            let x = XGAM[i];
            let y = YGAMNC[i];
            let result = cdf_gam_nc(1, mu, x, y);

            let err1 = (1.0 - result.p / PGAMNC[i]).abs();
            let err2 = (1.0 - result.q / QGAMNC[i]).abs();

            assert_eq!(result.ierr, 0, "i={i}, mu={mu}, x={x}, y={y}");
            assert!(
                err1 <= NONCENTRAL_RELATIVE_TOLERANCE,
                "i={i}, mu={mu}, x={x}, y={y}, P={}, expected={}, ERR(P)={err1}",
                result.p,
                PGAMNC[i],
            );
            assert!(
                err2 <= NONCENTRAL_RELATIVE_TOLERANCE,
                "i={i}, mu={mu}, x={x}, y={y}, Q={}, expected={}, ERR(Q)={err2}",
                result.q,
                QGAMNC[i],
            );
        }
    }

    // *********************************************************************
    //                   TEST 6
    // Test of the computed values of the noncentral chi-square distribution
    // function P(nu,chi2,y) and its complementary Q(nu,chi2,y):
    // comparison against precomputed values
    //      ERR means relative error
    // *********************************************************************
    #[test]
    fn test_6_noncentral_chi_square_distribution() {
        for i in 0..18 {
            let mu = AA[i] as i32 as f64 + 1.0;
            let x = XGAM[i];
            let y = YGAMNC[i];
            let result = cdf_gam_nc(2, mu, x, y);

            let err1 = (1.0 - result.p / PCHINC[i]).abs();
            let err2 = (1.0 - result.q / QCHINC[i]).abs();

            assert_eq!(result.ierr, 0, "i={i}, nu={mu}, chi2={x}, y={y}");
            assert!(
                err1 <= NONCENTRAL_RELATIVE_TOLERANCE,
                "i={i}, nu={mu}, chi2={x}, y={y}, P={}, expected={}, ERR(P)={err1}",
                result.p,
                PCHINC[i],
            );
            assert!(
                err2 <= NONCENTRAL_RELATIVE_TOLERANCE,
                "i={i}, nu={mu}, chi2={x}, y={y}, Q={}, expected={}, ERR(Q)={err2}",
                result.q,
                QCHINC[i],
            );
        }
    }

    // ********************************************************************
    //                   TEST 7
    // Test of the inversion routine for the noncentral gamma distribution
    // function with respect to x. The accuracy of the value obtained
    // in the inversion is checked.
    // ********************************************************************
    #[test]
    fn test_7_inverse_noncentral_gamma_distribution_with_respect_to_x() {
        let mut d0 = 0.0_f64;

        for i in 0..=10 {
            let mu = i as f64 * 10.8 + 0.5;

            for j in 1..=10 {
                let x = j as f64 * 0.75;

                for k in 1..=10 {
                    let y = k as f64 * 0.95;
                    let result = cdf_gam_nc(1, mu, x, y);

                    if result.ierr == 0
                        && let Ok(xr) =
                            inv_cdf_gam_nc(1, 1, mu, result.p, result.q, y)
                    {
                        let erxr = (1.0 - x / xr).abs();

                        if erxr > d0 {
                            d0 = erxr;
                        }
                    }
                }
            }
        }

        // Original testGammaCHI.dat:
        // Max. error (inversion): 7.354117315117037E-13
        assert!(d0 <= 7.354117315117037e-13, "Max. error (inversion): {d0}");
    }

    // *************************************************************************
    //                   TEST 8
    // Test of the inversion routine for the noncentral chi-square distribution
    // function with respect to x. The accuracy of the value obtained
    // in the inversion is checked.
    // *************************************************************************
    #[test]
    fn test_8_inverse_noncentral_chi_square_distribution_with_respect_to_x() {
        let mut d0 = 0.0_f64;

        for i in 0..=20 {
            let mu = 5.0 * i as f64 + 1.0;

            for j in 1..=10 {
                let x = j as f64 * 0.75;

                for k in 1..=10 {
                    let y = k as f64 * 0.9;
                    let result = cdf_gam_nc(2, mu, x, y);

                    if result.ierr == 0
                        && let Ok(xr) =
                            inv_cdf_gam_nc(2, 1, mu, result.p, result.q, y)
                    {
                        let erxr = (1.0 - x / xr).abs();

                        if erxr > d0 {
                            d0 = erxr;
                        }
                    }
                }
            }
        }

        // Original testGammaCHI.dat:
        // Max. error (inversion): 2.5616545560117054E-12
        assert!(d0 <= 2.5616545560117054e-12, "Max. error (inversion): {d0}");
    }

    // ********************************************************************
    //                   TEST 9
    // Test of the inversion routine for the noncentral gamma distribution
    // function with respect to y. The accuracy of the value obtained
    // in the inversion is checked.
    // ********************************************************************
    #[test]
    fn test_9_inverse_noncentral_gamma_distribution_with_respect_to_y() {
        let mut d0 = -1.0_f64;

        for i in 0..=10 {
            let mu = i as f64 * 10.8 + 0.5;

            for j in 1..=10 {
                let x = j as f64 * 0.75;

                for k in 1..=10 {
                    let y = k as f64 * 0.95;
                    let result = cdf_gam_nc(1, mu, x, y);

                    if result.ierr == 0
                        && let Ok(xr) =
                            inv_cdf_gam_nc(1, 2, mu, result.p, result.q, x)
                    {
                        let erxr = (1.0 - y / xr).abs();

                        if erxr > d0 {
                            d0 = erxr;
                        }

                        // Present in the original test program as a second,
                        // redundant computation of erxr.
                        let _erxr = (1.0 - y / xr).abs();
                    }
                }
            }
        }

        // Original testGammaCHI.dat:
        // Max. error (inversion): 7.743513404274982E-13
        assert!(d0 <= 7.743513404274982e-13, "Max. error (inversion): {d0}");
    }

    // *************************************************************************
    //                   TEST 10
    // Test of the inversion routine for the noncentral chi-square distribution
    // function with respect to y. The accuracy of the value obtained
    // in the inversion is checked.
    // *************************************************************************
    #[test]
    fn test_10_inverse_noncentral_chi_square_distribution_with_respect_to_y() {
        let mut d0 = -1.0_f64;

        for i in 0..=20 {
            let mu = 5.0 * i as f64 + 1.0;

            for j in (1..=100).step_by(10) {
                let x = j as f64 * 0.75;

                for k in (1..=100).step_by(10) {
                    let y = k as f64 * 0.9;
                    let result = cdf_gam_nc(2, mu, x, y);

                    if result.ierr == 0
                        && let Ok(xr) =
                            inv_cdf_gam_nc(2, 2, mu, result.p, result.q, x)
                    {
                        let erxr = (1.0 - y / xr).abs();

                        if erxr > d0 {
                            d0 = erxr;
                        }
                    }
                }
            }
        }

        // Original testGammaCHI.dat:
        // Max. error (inversion): 3.7258927497105243E-13
        assert!(d0 <= 3.7258927497105243e-13, "Max. error (inversion): {d0}");
    }

    // ***************************************************
    //                   TEST 11
    // Test of the computed values of the error function
    // erf(x): comparison against precomputed values
    //      ERR means relative error
    // ***************************************************
    #[test]
    fn test_11_error_function() {
        for i in 0..18 {
            let x = XERR[i];
            let erf = errorfunction(x, false, false);
            let err1 = (1.0 - erf / ERFF[i]).abs();

            assert!(
                err1 <= AUXILIARY_RELATIVE_TOLERANCE,
                "i={i}, x={x}, erf(x)={erf}, expected={}, ERR(erf(x))={err1}",
                ERFF[i],
            );
        }
    }

    // ***************************************************
    //                   TEST 12
    // Test of the computed values of the complementary
    // error function erfc(x): comparison against
    // precomputed values
    //      ERR means relative error
    // ***************************************************
    #[test]
    fn test_12_complementary_error_function() {
        for i in 0..18 {
            let x = XERFC[i];
            let erfc = errorfunction(x, true, false);
            let err1 = (1.0 - erfc / ERFCF[i]).abs();

            assert!(
                err1 <= AUXILIARY_RELATIVE_TOLERANCE,
                "i={i}, x={x}, erfc(x)={erfc}, expected={}, ERR(erfc(x))={err1}",
                ERFCF[i],
            );
        }
    }

    // *******************************************************
    //                   TEST 13
    // Test of the computed values of the scaled complementary
    // error function exp(x*x)*erfc(x): comparison against
    // precomputed values
    //      ERR means relative error
    // ******************************************************
    #[test]
    fn test_13_scaled_complementary_error_function() {
        for i in 0..18 {
            let x = XERFC[i];
            let serfc = errorfunction(x, true, true);
            let err1 = (1.0 - serfc / SERFCF[i]).abs();

            assert!(
                err1 <= AUXILIARY_RELATIVE_TOLERANCE,
                "i={i}, x={x}, serfc(x)={serfc}, expected={}, ERR(serfc(x))={err1}",
                SERFCF[i],
            );
        }
    }

    // ***********************************************************
    //                   TEST 14
    // Test of the inversion routine for the complementary error
    // function: the composition of the function with its inverse
    // is the identity
    // ***********************************************************
    #[test]
    fn test_14_inverse_complementary_error_function() {
        let mut d0 = -1.0_f64;

        // The original source has:
        //
        //   DO i=1,100
        //     x=j*0.1_r8;
        //
        // It uses j, not i. After TEST 10, the Fortran DO variable j has
        // the value 101 on normal loop termination. Therefore the literal
        // behavior of the original test is x = 10.1 for all 100 iterations.
        for _i in 1..=100 {
            let x = 101.0 * 0.1;
            let erfc = errorfunction(x, true, false);
            let xr = inverfc(erfc);
            let erxr = (1.0 - x / xr).abs();

            if erxr > d0 {
                d0 = erxr;
            }
        }

        // Original testGammaCHI.dat:
        // Max. error (inversion): 0.
        assert_eq!(d0, 0.0, "Max. error (inversion): {d0}");
    }

    // **************************************************
    //                   TEST 15
    // Test of the computed values of the gamma function:
    // comparison against precomputed values
    //      ERR means relative error
    // ********************************************
    #[test]
    fn test_15_gamma_function() {
        for i in 0..18 {
            let x = XX[i];
            let gam = gamma(x);
            let err1 = (1.0 - gam / GAMF[i]).abs();

            assert!(
                err1 <= AUXILIARY_RELATIVE_TOLERANCE,
                "i={i}, x={x}, gamma(x)={gam}, expected={}, ERR(gamma(x))={err1}",
                GAMF[i],
            );
        }
    }

    // *****************************************************
    //                   TEST 16
    // Test of the computed values of the logarithm of the
    // gamma function: comparison against precomputed values
    //      ERR means relative error
    // *****************************************************
    #[test]
    fn test_16_logarithm_of_gamma_function() {
        for i in 0..18 {
            let x = XLOG[i];
            let lgam = loggam(x);
            let err1 = (1.0 - lgam / LGAMF[i]).abs();

            assert!(
                err1 <= AUXILIARY_RELATIVE_TOLERANCE,
                "i={i}, x={x}, log(gamma(x))={lgam}, expected={}, ERR(log(gamma(x)))={err1}",
                LGAMF[i],
            );
        }
    }

    // *****************************************************
    //                   TEST 17
    // Test of the computed values of the quotient of two
    // gamma function: comparison against precomputed values
    //      ERR means relative error
    // *****************************************************
    #[test]
    fn test_17_quotient_of_two_gamma_functions() {
        let mut j = 0usize;

        for i in 1..=9 {
            j += 1;
            let x = XQUOT[i - 1];
            let y = XQUOT[i - 1] + i as f64 * 10.0;
            let quot = quotgamm(x, y);
            let err1 = (1.0 - quot / QUOTF[j - 1]).abs();

            assert!(
                err1 <= AUXILIARY_RELATIVE_TOLERANCE,
                "i={i}, x={x}, y={y}, quotient={quot}, expected={}, ERR={err1}",
                QUOTF[j - 1],
            );

            j += 1;
            let x = XQUOT[i - 1] + i as f64 * 5.0;
            let y = XQUOT[i - 1];
            let quot = quotgamm(x, y);
            let err1 = (1.0 - quot / QUOTF[j - 1]).abs();

            assert!(
                err1 <= AUXILIARY_RELATIVE_TOLERANCE,
                "i={i}, x={x}, y={y}, quotient={quot}, expected={}, ERR={err1}",
                QUOTF[j - 1],
            );
        }
    }

    // *****************************************************
    //                   TEST 18
    // Test of the computed values of the regulated
    // gamma function gamma*(x):
    // comparison against precomputed values
    //      ERR means relative error
    // *****************************************************
    #[test]
    fn test_18_regulated_gamma_function() {
        for i in 0..18 {
            let x = XLOG[i];
            let gams = gamstar(x);
            let err1 = (1.0 - gams / GAMSF[i]).abs();

            assert!(
                err1 <= AUXILIARY_RELATIVE_TOLERANCE,
                "i={i}, x={x}, gamma*(x)={gams}, expected={}, ERR(gamma*(x))={err1}",
                GAMSF[i],
            );
        }
    }
}
