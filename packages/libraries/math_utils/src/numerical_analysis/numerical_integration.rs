use core::f64;
use std::{cell::Cell, usize};

use thiserror::Error;
use translate_core::{Translatable, TranslatedMessage};

use crate::translation_provider::translation_resolver;

#[derive(Debug, Error)]
pub enum IntegrationError {
    #[error("invalid tolerance")]
    InvalidTolerance,
    #[error("maximum number of subdivisions reached")]
    SubdivisionLimitReached,
    #[error("roundoff error prevents convergence")]
    RoundoffError,
    #[error("extremely bad integrand behavior")]
    BadIntegrandBehavior,
    #[error("extrapolation did not converge")]
    ExtrapolationFailed,
    #[error("the integral is probably divergent or slowly convergent")]
    ProbablyDivergent,
    #[error("the integrand returned a non-finite value at x = {x}: {value}")]
    NonFiniteIntegrand { x: f64, value: f64 },
}

impl Translatable for IntegrationError {
    fn translate(
        &self,
        language: translate_core::Language,
    ) -> Result<String, translate_core::TranslationError> {
        match self {
            IntegrationError::InvalidTolerance => TranslatedMessage::new(
                "numerical_integration.invalid_tolerance",
                translation_resolver,
            ),
            IntegrationError::SubdivisionLimitReached => {
                TranslatedMessage::new(
                    "numerical_integration.subdivision_limit_reached",
                    translation_resolver,
                )
            }
            IntegrationError::RoundoffError => TranslatedMessage::new(
                "numerical_integration.roundoff_error",
                translation_resolver,
            ),
            IntegrationError::BadIntegrandBehavior => TranslatedMessage::new(
                "numerical_integration.bad_integrand_behavior",
                translation_resolver,
            ),
            IntegrationError::ExtrapolationFailed => TranslatedMessage::new(
                "numerical_integration.extrapolation_failed",
                translation_resolver,
            ),
            IntegrationError::ProbablyDivergent => TranslatedMessage::new(
                "numerical_integration.probably_divergent",
                translation_resolver,
            ),
            IntegrationError::NonFiniteIntegrand { x, value } => {
                TranslatedMessage::new(
                    "numerical_integration.non_finite_integrand",
                    translation_resolver,
                )
                .key("x", x)
                .key("value", value)
            }
        }
        .translate(language)
    }
}

/// Numerically approximates the definite integral of `f` over the interval
/// from `lower_bound` to `upper_bound`.
///
/// This function is a convenience wrapper around the internal `DQAGSE`
/// implementation, which uses an adaptive 21-point Gauss-Kronrod quadrature
/// rule together with extrapolation.
pub fn calculate_numerical_integral<F>(
    f: &F,
    lower_bound: f64,
    upper_bound: f64,
) -> Result<f64, IntegrationError>
where
    F: Fn(f64) -> f64,
{
    const ABSOLUTE_TOLERANCE: f64 = 1.0e-12;
    const RELATIVE_TOLERANCE: f64 = 1.0e-12;
    const SUBDIVISION_LIMIT: usize = 100;

    let non_finite_value = Cell::new(None);

    let checked_f = |x: f64| -> f64 {
        if non_finite_value.get().is_some() {
            return f64::NAN;
        }

        match checked_function(f, x) {
            Ok(value) => value,
            Err(error) => {
                non_finite_value.set(Some(error));
                f64::NAN
            }
        }
    };

    let output = dqagse(
        &checked_f,
        lower_bound,
        upper_bound,
        ABSOLUTE_TOLERANCE,
        RELATIVE_TOLERANCE,
        SUBDIVISION_LIMIT,
    );

    if let Some((x, value)) = non_finite_value.get() {
        return Err(IntegrationError::NonFiniteIntegrand { x, value });
    }

    match output.status {
        IntegrationStatus::Converged => Ok(output.result),
        IntegrationStatus::InvalidTolerance => {
            Err(IntegrationError::InvalidTolerance)
        }
        IntegrationStatus::SubdivisionLimitReached => {
            Err(IntegrationError::SubdivisionLimitReached)
        }
        IntegrationStatus::RoundoffError => {
            Err(IntegrationError::RoundoffError)
        }
        IntegrationStatus::BadIntegrandBehavior => {
            Err(IntegrationError::BadIntegrandBehavior)
        }
        IntegrationStatus::ExtrapolationFailed => {
            Err(IntegrationError::ExtrapolationFailed)
        }
        IntegrationStatus::ProbablyDivergent => {
            Err(IntegrationError::ProbablyDivergent)
        }
    }
}

fn checked_function<F>(f: &F, x: f64) -> Result<f64, (f64, f64)>
where
    F: Fn(f64) -> f64,
{
    let value = f(x);

    if value.is_finite() {
        Ok(value)
    } else {
        Err((x, value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integrates_x_squared_from_zero_to_one() {
        let f = |x: f64| x.powi(2);
        let result = calculate_numerical_integral(&f, 0.0, 1.0)
            .expect("integration of x^2 should succeed");

        assert!(
            (result - 1.0 / 3.0).abs() < 1.0e-14,
            "expected approximately 1/3, got {result}"
        );
    }

    #[test]
    fn integrates_two_x_squared_from_zero_to_one() {
        let f = |x: f64| 2.0 * x.powi(2);
        let result = calculate_numerical_integral(&f, 0.0, 1.0)
            .expect("integration of 2x^2 should succeed");

        assert!(
            (result - 2.0 / 3.0).abs() < 1.0e-14,
            "expected approximately 2/3, got {result}"
        );
    }

    #[test]
    fn rejects_one_over_x_at_zero() {
        let f = |x: f64| 1.0 / x;
        let error = calculate_numerical_integral(&f, -1.0, 1.0).unwrap_err();

        assert!(matches!(
            error,
            IntegrationError::NonFiniteIntegrand {
                x: 0.0,
                value
            } if value == f64::INFINITY
        ));
    }
}

/*
    The following code is a rust port of QUADPACK's DQAGSE routines.

    source: https://www.netlib.org/quadpack/

    original implementation: https://www.netlib.org/cgi-bin/netlibfiles.txt?format=txt&filename=quadpack%2Fdqagse.f
*/

#[allow(dead_code)]
struct IntegrationOutput {
    result: f64,
    absolute_error: f64,
    evaluations: usize,
    subintervals: usize,
    status: IntegrationStatus,
}

enum IntegrationStatus {
    Converged = 0,
    SubdivisionLimitReached = 1,
    RoundoffError = 2,
    BadIntegrandBehavior = 3,
    ExtrapolationFailed = 4,
    ProbablyDivergent = 5,
    InvalidTolerance = 6,
}

fn convert_to_status(ier: usize) -> IntegrationStatus {
    match ier {
        0 => IntegrationStatus::Converged,
        1 => IntegrationStatus::SubdivisionLimitReached,
        2 => IntegrationStatus::RoundoffError,
        3 => IntegrationStatus::BadIntegrandBehavior,
        4 => IntegrationStatus::ExtrapolationFailed,
        5 => IntegrationStatus::ProbablyDivergent,
        6 => IntegrationStatus::InvalidTolerance,
        _ => unreachable!("invalid DQAGSE status code: {ier}"),
    }
}

const LIMEXP: usize = 50;

/// Calculates an approximation `result` to the definite integral
///
/// `i = integral of f over (a, b)`
///
/// hopefully satisfying the following accuracy requirement:
///
/// `abs(i - result) <= max(epsabs, epsrel * abs(i))`
///
/// This is a globally adaptive, general-purpose integration routine with
/// extrapolation. It can handle endpoint singularities.
///
/// This is the double-precision version of the original `DQAGSE` Fortran
/// routine.
///
/// # Arguments
///
/// ## On entry
///
/// * `f` - Function defining the integrand `f(x)`.
/// * `a` - Lower limit of integration.
/// * `b` - Upper limit of integration.
/// * `epsabs` - Requested absolute accuracy.
/// * `epsrel` - Requested relative accuracy.
///
///   If
///
///   `epsabs <= 0`
///
///   and
///
///   `epsrel < max(50 * relative machine accuracy, 0.5e-28)`
///
///   the routine terminates with `ier = 6`.
///
/// * `limit` - Upper bound on the number of subintervals in the partition of
///   `(a, b)`.
///
/// ## On return
///
/// * `result` - Approximation to the integral.
/// * `abserr` - Estimate of the modulus of the absolute error, which should
///   equal or exceed `abs(i - result)`.
/// * `neval` - Number of integrand evaluations.
/// * `ier` - Termination status.
///
///   * `ier = 0` - Normal and reliable termination of the routine. It is
///     assumed that the requested accuracy has been achieved.
///   * `ier > 0` - Abnormal termination of the routine. The estimates for the
///     integral and error are less reliable. It is assumed that the requested
///     accuracy has not been achieved.
///
/// * `alist` - Vector of length at least `limit`. Its first `last` elements are
///   the left endpoints of the subintervals in the partition of the given
///   integration range `(a, b)`.
/// * `blist` - Vector of length at least `limit`. Its first `last` elements are
///   the right endpoints of the subintervals in the partition of the given
///   integration range `(a, b)`.
/// * `rlist` - Vector of length at least `limit`. Its first `last` elements are
///   the integral approximations on the subintervals.
/// * `elist` - Vector of length at least `limit`. Its first `last` elements are
///   the moduli of the absolute error estimates on the subintervals.
/// * `iord` - Integer vector of length at least `limit`. Its first `k` elements
///   are pointers to the error estimates over the subintervals, such that
///
///   `elist[iord[0]], ..., elist[iord[k - 1]]`
///
///   form a decreasing sequence, where `k = last` if
///
///   `last <= limit / 2 + 2`
///
///   and `k = limit + 1 - last` otherwise.
/// * `last` - Number of subintervals actually produced during the subdivision
///   process.
///
/// # Error codes
///
/// * `ier = 1` - The maximum number of allowed subdivisions has been reached.
///   More subdivisions can be allowed by increasing `limit` and adjusting the
///   corresponding dimensions. If this yields no improvement, the integrand
///   should be analyzed to determine the integration difficulties. If the
///   position of a local difficulty can be determined, such as a singularity
///   or discontinuity within the interval, it may be beneficial to split the
///   interval at that point and call the integrator on the resulting
///   subranges. Where possible, an appropriate special-purpose integrator
///   designed for the particular difficulty should be used.
/// * `ier = 2` - Roundoff error has been detected, preventing the requested
///   tolerance from being achieved. The error may be underestimated.
/// * `ier = 3` - Extremely bad integrand behavior occurs at some points of the
///   integration interval.
/// * `ier = 4` - The algorithm does not converge. Roundoff error has been
///   detected in the extrapolation table. It is presumed that the requested
///   tolerance cannot be achieved and that the returned result is the best
///   result that can be obtained.
/// * `ier = 5` - The integral is probably divergent or slowly convergent.
///   Divergence can also occur with any other value of `ier`.
/// * `ier = 6` - The input is invalid because
///
///   `epsabs <= 0`
///
///   and
///
///   ` epsrel < max(50 * relative machine accuracy, 0.5e-28)`
///
///   `result`, `abserr`, `neval`, `last`, `rlist[0]`, `iord[0]`, and
///   `elist[0]` are set to zero. `alist[0]` and `blist[0]` are set to `a` and
///   `b`, respectively.
///
/// # Implementation details
///
/// * Original routine: `DQAGSE`
/// * Integration method: globally adaptive integration with extrapolation
/// * Precision: double precision
/// * Date written: 1980-01-01
/// * Revision date: 1983-05-18
///
/// # Authors
///
/// * Robert Piessens, Applied Mathematics and Programming Division,
///   K.U. Leuven
/// * Elise de Doncker, Applied Mathematics and Programming Division,
///   K.U. Leuven
fn dqagse<F>(
    f: &F,
    a: f64,
    b: f64,
    epsabs: f64,
    epsrel: f64,
    limit: usize,
) -> IntegrationOutput
where
    F: Fn(f64) -> f64,
{
    // the dimension of rlist2 is determined by the value of
    // limexp in subroutine dqelg (rlist2 should be of dimension
    // (limexp+2) at least).

    // List of major variables
    //
    // alist - List of left endpoints of all subintervals considered up to now.
    //
    // blist - List of right endpoints of all subintervals considered up to now.
    //
    // rlist(i) - Approximation to the integral over (alist(i), blist(i)).
    //
    // rlist2 - Array of length at least limexp + 2 containing the part of the
    //          epsilon table that is still needed for further computations.
    //
    // elist(i) - Error estimate applying to rlist(i).
    //
    // maxerr - Pointer to the interval with the largest error estimate.
    //
    // errmax - elist(maxerr).
    //
    // erlast - Error on the interval currently being subdivided, before the
    //          subdivision has taken place.
    //
    // area - Sum of the integrals over the subintervals.
    //
    // errsum - Sum of the errors over the subintervals.
    //
    // errbnd - Requested accuracy:
    //          max(epsabs, epsrel * abs(result)).
    //
    // *****1 - Variable for the left interval.
    //
    // *****2 - Variable for the right interval.
    //
    // last - Number of active subintervals.
    //
    // nres - Number of calls to the extrapolation routine.
    //
    // numrl2 - Number of elements currently in rlist2. If an appropriate
    //          approximation to the compounded integral has been obtained, it is
    //          put in rlist2(numrl2) after numrl2 has been increased by one.
    //
    // small - Length of the smallest interval considered up to now, multiplied
    //         by 1.5.
    //
    // erlarg - Sum of the errors over the intervals larger than the smallest
    //          interval considered up to now.
    //
    // extrap - Logical variable denoting that the routine is attempting to perform
    //          extrapolation. That is, before subdividing the smallest interval,
    //          the routine tries to decrease the value of erlarg.
    //
    // noext - Logical variable denoting that extrapolation is no longer allowed
    //         when its value is true.

    // Machine dependent constants
    //
    // epmach is the largest relative spacing.
    // uflow is the smallest positive magnitude.
    // oflow is the largest positive magnitude.
    let epmach = f64::EPSILON; // original: d1mach(4)

    // test on validity of parameters
    let mut ier = 0;
    let mut neval = 0;
    let mut last = 0;
    let mut result = 0.0;
    let mut abserr = 0.0;

    let mut alist = vec![a; limit]; // alist(1) = a
    let mut blist = vec![b; limit]; // blist(1) = b
    let mut rlist = vec![0.0_f64; limit]; // rlist(1) = 0.0d+00
    let mut elist = vec![0.0_f64; limit]; // elist(1) = 0.0d+00

    if epsabs <= 0.0 && epsrel < (50.0 * epmach).max(0.5e-28) {
        ier = 6;
        return IntegrationOutput {
            result,
            absolute_error: abserr,
            evaluations: neval,
            subintervals: last,
            status: convert_to_status(ier),
        };
    }

    // first approximation to the integral
    let uflow = f64::MIN_POSITIVE; // orginal: d1mach(1)
    let oflow = f64::MAX; // orginal: d1mach(2)
    let mut ierro = 0;

    let int_guess = dqk21(f, a, b);

    result = int_guess.result;
    abserr = int_guess.absolute_error;
    let defabs = int_guess.result_abs;
    let resabs = int_guess.result_asc;

    // test on accuracy.
    let dres = result.abs();
    let mut errbnd = epsabs.max(epsrel * dres);
    last = 1;
    rlist[0] = result;
    elist[0] = abserr;
    let mut iord = vec![0usize; limit]; // iord(1) = 1 

    if abserr <= 100.0 * epmach * defabs && abserr > errbnd {
        ier = 2;
    }
    if limit == 1 {
        ier = 1;
    }
    if ier != 0 || (abserr <= errbnd && abserr != resabs) || abserr == 0.0 {
        neval = 42 * last - 21;
        return IntegrationOutput {
            result,
            absolute_error: abserr,
            evaluations: neval,
            subintervals: last,
            status: convert_to_status(ier),
        };
    }

    // initialization
    let mut rlist2 = [0.0_f64; LIMEXP + 2]; // rlist2(1) = result
    rlist2[0] = result;

    let mut res3la = [0.0_f64; 3];

    let mut errmax = abserr;
    let mut maxerr = 0;
    let mut area = result;
    let mut errsum = abserr;
    let mut abserr = oflow;
    let mut nrmax = 0;
    let mut nres = 0;
    let mut numrl2 = 2;
    let mut ktmin = 0;
    let mut extrap = false;
    let mut noext = false;
    let mut iroff1 = 0;
    let mut iroff2 = 0;
    let mut iroff3 = 0;
    let ksgn = if dres >= (1.0 - 50.0 * epmach) * defabs {
        1_i8
    } else {
        -1_i8
    };

    let mut small = 0.0;
    let mut erlarg = 0.0;
    let mut ertest = 0.0;
    let mut correc = 0.0;

    enum MainLoopExit {
        /// jump to label 115
        UseGlobalSum,
        /// jump to label 100
        EnterFinalSelection,
    }

    // main do-loop
    let main_loop_exit = 'main_loop: loop {
        //for last in 2..=limit
        if last >= limit {
            break 'main_loop MainLoopExit::EnterFinalSelection;
        }

        last = last + 1;

        // bisect the subinterval with the nrmax-th largest error estimate.
        let a1 = alist[maxerr];
        let b1 = 0.5 * (alist[maxerr] + blist[maxerr]);
        let a2 = b1;
        let b2 = blist[maxerr];
        let erlast = errmax;

        let integration1 = dqk21(f, a1, b1);
        let area1 = integration1.result;
        let error1 = integration1.absolute_error;
        let defab1 = integration1.result_asc;

        let integration2 = dqk21(f, a2, b2);
        let area2 = integration2.result;
        let error2 = integration2.absolute_error;
        let defab2 = integration2.result_asc;

        // improve previous approximations to integral
        // and error and test for accuracy.
        let area12 = area1 + area2;
        let erro12 = error1 + error2;
        errsum = errsum + erro12 - errmax;
        area = area + area12 - rlist[maxerr];

        if defab1 != error1 && defab2 != error2 {
            if (rlist[maxerr] - area12).abs() <= 0.1e-4 * area12.abs()
                && erro12 >= 0.99 * errmax
            {
                if extrap {
                    iroff2 = iroff2 + 1;
                } else {
                    iroff1 = iroff1 + 1;
                }
            }

            if last > 10 && erro12 > errmax {
                iroff3 = iroff3 + 1;
            }
        }

        rlist[maxerr] = area1;
        rlist[last - 1] = area2;
        errbnd = epsabs.max(epsrel * area.abs());

        // test for roundoff error and eventually set error flag.
        if iroff1 + iroff2 >= 10 || iroff3 >= 20 {
            ier = 2;
        }
        if iroff2 >= 5 {
            ierro = 3;
        }

        // set error flag in the case that the number of subintervals
        // equals limit.
        if last == limit {
            ier = 1;
        }

        // set error flag in the case of bad integrand behaviour
        // at a point of the integration range.
        if (a1.abs()).max(b2.abs())
            <= (1.0 + 100.0 * epmach) * (a2.abs() + 1000.0 * uflow)
        {
            ier = 4;
        }

        // append the newly-created intervals to the list.
        if error2 > error1 {
            alist[maxerr] = a2;
            alist[last - 1] = a1;
            blist[last - 1] = b1;
            rlist[maxerr] = area2;
            rlist[last - 1] = area1;
            elist[maxerr] = error2;
            elist[last - 1] = error1;
        } else {
            alist[last - 1] = a2;
            blist[maxerr] = b1;
            blist[last - 1] = b2;
            elist[maxerr] = error1;
            elist[last - 1] = error2;
        }

        // call subroutine dqpsrt to maintain the descending ordering
        // in the list of error estimates and select the subinterval
        // with nrmax-th largest error estimate (to be bisected next).
        let ordering = dqpsrt(limit, last, maxerr, &elist, &mut iord, nrmax);

        maxerr = ordering.max_error_index;
        errmax = ordering.max_error;
        nrmax = ordering.rank;

        // jump out of do-loop
        if errsum <= errbnd {
            break 'main_loop MainLoopExit::UseGlobalSum;
        }

        // jump out of do-loop
        if ier != 0 {
            break 'main_loop MainLoopExit::EnterFinalSelection;
        }

        if last == 2 {
            small = (b - a).abs() * 0.375;
            erlarg = errsum;
            ertest = errbnd;
            rlist2[1] = area;
            continue;
        }
        if noext {
            continue;
        }

        erlarg = erlarg - erlast;
        if (b1 - a1).abs() > small {
            erlarg = erlarg + erro12;
        }

        if !extrap {
            // test whether the interval to be bisected next is the
            // smallest interval.
            if (blist[maxerr] - alist[maxerr]).abs() > small {
                continue;
            }

            extrap = true;
            nrmax = 1;
        }

        if ierro != 3 && erlarg > ertest {
            // the smallest interval has the largest error.
            // before bisecting decrease the sum of the errors over the
            // larger intervals (erlarg) and perform extrapolation.

            let id = nrmax;
            let jupbnd = if last > (2 + limit / 2) {
                limit + 3 - last
            } else {
                last
            };

            for _k in id..jupbnd {
                maxerr = iord[nrmax];
                errmax = elist[maxerr];

                // jump out of do-loop
                if (blist[maxerr] - alist[maxerr]).abs() > small {
                    continue 'main_loop;
                }
                nrmax = nrmax + 1;
            }
        }

        // perform extrapolation.
        numrl2 = numrl2 + 1;
        rlist2[numrl2 - 1] = area;

        let epsilon = dqelg(numrl2, &mut rlist2, &mut res3la, nres);

        let reseps = epsilon.result;
        let abseps = epsilon.absolute_error;
        nres = epsilon.extrapolation_count;
        numrl2 = epsilon.table_length;

        ktmin = ktmin + 1;
        if ktmin > 5 && abserr < 1e-3 * errsum {
            ier = 5;
        }

        if abseps < abserr {
            ktmin = 0;
            abserr = abseps;
            result = reseps;
            correc = erlarg;
            ertest = epsabs.max(epsrel * reseps.abs());

            // jump out of do-loop
            if abserr <= ertest {
                break 'main_loop MainLoopExit::EnterFinalSelection;
            }
        }

        // prepare bisection of the smallest interval.
        if numrl2 == 1 {
            noext = true;
        }
        if ier == 5 {
            break 'main_loop MainLoopExit::EnterFinalSelection;
        }

        maxerr = iord[0];
        errmax = elist[maxerr];
        nrmax = 0;
        extrap = false;
        small = small * 0.5;
        erlarg = errsum;
    }; // end loop

    enum FinalAction {
        /// label 115
        UseGlobalSum,
        /// label 110
        KeepExtrapolatedAndTestDivergence,
        /// label 130
        KeepExtrapolatedWithoutDivergenceTest,
    }

    // set final result and error estimate.
    let final_action = match main_loop_exit {
        MainLoopExit::UseGlobalSum => FinalAction::UseGlobalSum,

        MainLoopExit::EnterFinalSelection => {
            if abserr == oflow {
                FinalAction::UseGlobalSum
            } else {
                if ier + ierro == 0 {
                    // 110
                    FinalAction::KeepExtrapolatedAndTestDivergence
                } else {
                    if ierro == 3 {
                        abserr = abserr + correc;
                    }
                    if ier == 0 {
                        ier = 3;
                    }

                    if result != 0.0 && area != 0.0 {
                        // 105
                        let extrapolated_relative_error = abserr / result.abs();
                        let global_relative_error = errsum / area.abs();

                        if extrapolated_relative_error > global_relative_error {
                            // 115
                            FinalAction::UseGlobalSum
                        } else {
                            // 110
                            FinalAction::KeepExtrapolatedAndTestDivergence
                        }
                    } else if abserr > errsum {
                        // 115
                        FinalAction::UseGlobalSum
                    } else if area == 0.0 {
                        // 130
                        FinalAction::KeepExtrapolatedWithoutDivergenceTest
                    } else {
                        // 110
                        FinalAction::KeepExtrapolatedAndTestDivergence
                    }
                }
            }
        }
    };

    // test on divergence.
    if let FinalAction::KeepExtrapolatedAndTestDivergence = final_action {
        let cancellation_case =
            ksgn == -1 && result.abs().max(area.abs()) <= defabs * 0.01;

        if !cancellation_case {
            let ratio = result / area;

            if ratio < 0.01 || ratio > 100.0 || errsum > area.abs() {
                ier = 6;
            }
        }
    }

    // compute global integral sum.
    if let FinalAction::UseGlobalSum = final_action {
        result = 0.0;
        for k in 0..last {
            result = result + rlist[k];
        }

        abserr = errsum;
    }

    if ier > 2 {
        ier = ier - 1;
    }
    neval = 42 * last - 21;

    IntegrationOutput {
        result,
        absolute_error: abserr,
        evaluations: neval,
        subintervals: last,
        status: convert_to_status(ier),
    }
}

struct EpsilonOutput {
    result: f64,
    absolute_error: f64,
    extrapolation_count: usize,
    table_length: usize,
}

/// Determines the limit of a given sequence of approximations by means of
/// P. Wynn's epsilon algorithm. An estimate of the absolute error is also
/// given.
///
/// The condensed epsilon table is computed. Only those elements needed for
/// the computation of the next diagonal are preserved.
///
/// This is the double-precision version of the original `DQELG` Fortran
/// routine.
///
/// # Arguments
///
/// * `n` - `epstab[n]` contains the new element in the first column of the
///   epsilon table.
/// * `epstab` - Vector of length 52 containing the elements of the two lower
///   diagonals of the triangular epsilon table. The elements are numbered
///   starting at the right-hand corner of the triangle.
/// * `result` - Resulting approximation to the integral.
/// * `abserr` - Estimate of the absolute error computed from `result` and the
///   three previous results.
/// * `res3la` - Vector of length 3 containing the last three results.
/// * `nres` - Number of calls to the routine. This should be zero on the first
///   call.
///
/// # Implementation details
///
/// * Original routine: `DQELG`
/// * Algorithm: epsilon algorithm
/// * Precision: double precision
/// * Revision date: 1983-05-18
///
/// # Authors
///
/// * Robert Piessens, Applied Mathematics and Programming Division,
///   K.U. Leuven
/// * Elise de Doncker, Applied Mathematics and Programming Division,
///   K.U. Leuven
fn dqelg(
    n: usize,
    epstab: &mut [f64],
    res3la: &mut [f64],
    nres: usize,
) -> EpsilonOutput {
    let mut n: usize = n;

    // List of major variables
    //
    // e0, e1, e2, e3 - The four elements on which the computation of a new
    //                  element in the epsilon table is based:
    //
    //                              e0
    //                        e3    e1    new
    //                              e2
    //
    // newelm - Number of elements to be computed in the new diagonal.
    //
    // error - error = abs(e1 - e0) + abs(e2 - e1) + abs(new - e2)
    //
    // result - The element in the new diagonal with the least value of error.

    // Machine-dependent constants
    //
    // epmach - The largest relative spacing.
    //
    // oflow - The largest positive magnitude.
    //
    // limexp - The maximum number of elements the epsilon table can contain.
    //          If this number is reached, the upper diagonal of the epsilon
    //          table is deleted.

    let epmach = f64::EPSILON; // original: d1mach(4)
    let oflow = f64::MAX; // original: d1mach(2)

    let nres = nres + 1;
    let mut abserr = oflow;
    let mut result = epstab[n - 1];

    if n < 3 {
        abserr = abserr.max(5.0 * epmach * result.abs());
        return EpsilonOutput {
            result,
            absolute_error: abserr,
            extrapolation_count: nres,
            table_length: n,
        };
    }

    epstab[n + 1] = epstab[n - 1];
    let newelm = (n - 1) / 2;
    epstab[n - 1] = oflow;
    let num = n;
    let mut k1 = n;

    for i in 1..=newelm {
        let k2 = k1 - 1;
        let k3 = k1 - 2;
        let res = epstab[k1 + 1];
        let e0 = epstab[k3 - 1];
        let e1 = epstab[k2 - 1];
        let e2 = res;
        let e1abs = e1.abs();
        let delta2 = e2 - e1;
        let err2 = delta2.abs();
        let tol2 = (e2.abs()).max(e1abs) * epmach;
        let delta3 = e1 - e0;
        let err3 = delta3.abs();
        let tol3 = e1abs.max(e0.abs()) * epmach;

        if err2 <= tol2 && err3 <= tol3 {
            // if e0, e1 and e2 are equal to within machine
            // accuracy, convergence is assumed.
            // result = e2
            // abserr = abs(e1-e0) + abs(e2-e1)

            result = res;
            abserr = err2 + err3;

            // jump out of do-loop
            abserr = abserr.max(5.0 * epmach * result.abs());
            return EpsilonOutput {
                result,
                absolute_error: abserr,
                extrapolation_count: nres,
                table_length: n,
            };
        }

        let e3 = epstab[k1 - 1];
        epstab[k1 - 1] = e1;
        let delta1 = e1 - e3;
        let err1 = delta1.abs();
        let tol1 = e1abs.max(e3.abs()) * epmach;

        // if two elements are very close to each other, omit
        // a part of the table by adjusting the value of n
        if err1 <= tol1 || err2 <= tol2 || err3 <= tol3 {
            n = i + i - 1;
            // jump out of do-loop
            break;
        }

        let ss = 1.0 / delta1 + 1.0 / delta2 - 1.0 / delta3;
        let epsinf = (ss * e1).abs();

        // test to detect irregular behaviour in the table, and
        // eventually omit a part of the table adjusting the value
        // of n.
        if epsinf <= 0.1e-3 {
            n = i + i - 1;
            // jump out of do-loop
            break;
        }

        // compute a new element and eventually adjust
        // the value of result.
        let res = e1 + 1.0 / ss;
        epstab[k1 - 1] = res;
        k1 = k1 - 2;
        let error = err2 + (res - e2).abs() + err3;

        if error <= abserr {
            abserr = error;
            result = res;
        }
    }

    // shift the table.
    if n == LIMEXP {
        n = 2 * (LIMEXP / 2) - 1;
    }

    let mut ib = 1;
    if (num / 2) * 2 == num {
        ib = 2;
    }

    let ie = newelm + 1;
    for _i in 1..=ie {
        let ib2 = ib + 2;
        epstab[ib - 1] = epstab[ib2 - 1];
        ib = ib2;
    }

    if num != n {
        let mut indx = num - n + 1;
        for i in 1..=n {
            epstab[i - 1] = epstab[indx - 1];
            indx = indx + 1;
        }
    }

    if nres < 4 {
        res3la[nres - 1] = result;
        abserr = oflow;
        abserr = abserr.max(5.0 * epmach * result.abs());
        return EpsilonOutput {
            result,
            absolute_error: abserr,
            extrapolation_count: nres,
            table_length: n,
        };
    }

    // compute error estimate
    abserr = (result - res3la[2]).abs()
        + (result - res3la[1]).abs()
        + (result - res3la[0]).abs();
    res3la[0] = res3la[1];
    res3la[1] = res3la[2];
    res3la[2] = result;
    abserr = abserr.max(5.0 * epmach * result.abs());

    EpsilonOutput {
        result,
        absolute_error: abserr,
        extrapolation_count: nres,
        table_length: n,
    }
}

struct ErrorOrderingOutput {
    max_error_index: usize,
    max_error: f64,
    rank: usize,
}

/// Maintains the descending order of the local error estimates resulting from
/// the interval subdivision process.
///
/// At each call, two error estimates are inserted using a sequential search:
/// top-down for the largest error estimate and bottom-up for the smallest error
/// estimate.
///
/// This is the double-precision version of the original `DQPSRT` Fortran
/// ordering routine.
///
/// # Arguments
///
/// The parameters are described by their meaning on return.
///
/// * `limit` - Maximum number of error estimates the list can contain.
/// * `last` - Number of error estimates currently in the list.
/// * `maxerr` - Points to the `nrmax`-th largest error estimate currently in
///   the list.
/// * `ermax` - The `nrmax`-th largest error estimate:
///
///   `ermax = elist[maxerr]`
///
/// * `elist` - Vector of length `last` containing the error estimates.
/// * `iord` - Integer vector of length `last`. Its first `k` elements contain
///   pointers to the error estimates such that
///
///   `elist[iord[0]], ..., elist[iord[k]]`
///
///   form a decreasing sequence, where
///
///   `k = last`
///
///   if
///
///   `last <= limit / 2 + 2`
///
///   and
///
///   `k = limit + 1 - last`
///
///   otherwise.
/// * `nrmax` - Satisfies
///
///   `maxerr = iord[nrmax]`
///
/// # Implementation details
///
/// * Original routine: `DQPSRT`
/// * Purpose: sequential sorting
/// * Precision: double precision
/// * Revision date: 1981-01-01
///
/// # Authors
///
/// * Robert Piessens, Applied Mathematics and Programming Division,
///   K.U. Leuven
/// * Elise de Doncker, Applied Mathematics and Programming Division,
///   K.U. Leuven
fn dqpsrt(
    limit: usize,
    last: usize,
    maxerr: usize,
    elist: &[f64],
    iord: &mut [usize],
    nrmax: usize,
) -> ErrorOrderingOutput {
    let mut nrmax = nrmax;

    // check whether the list contains more than
    // two error estimates.
    if last <= 2 {
        iord[0] = 0;

        if last == 2 {
            iord[1] = 1;
        }

        // set maxerr and ermax.
        let maxerr = iord[nrmax];
        let ermax = elist[maxerr];
        return ErrorOrderingOutput {
            max_error_index: maxerr,
            max_error: ermax,
            rank: nrmax,
        };
    }

    // this part of the routine is only executed if, due to a
    // difficult integrand, subdivision increased the error
    // estimate. in the normal case the insert procedure should
    // start after the nrmax-th largest error estimate.
    let errmax = elist[maxerr];
    if nrmax != 0 {
        for _i in 0..nrmax {
            let isucc = iord[nrmax - 1];
            // jump out of do-loop
            if errmax <= elist[isucc] {
                break;
            }

            iord[nrmax] = isucc;
            nrmax = nrmax - 1;
        }
    }

    // compute the number of elements in the list to be maintained
    // in descending order. this number depends on the number of
    // subdivisions still allowed.

    let jupbn = if last > (limit / 2 + 2) {
        limit + 3 - last
    } else {
        last
    };
    let errmin = elist[last - 1];

    // insert errmax by traversing the list top-down,
    // starting comparison from the element elist(iord(nrmax+1)).
    let jbnd = jupbn - 2;
    let ibeg = nrmax + 1;

    if ibeg > jbnd {
        iord[jbnd] = maxerr;
        iord[jupbn - 1] = last - 1;

        // set maxerr and ermax.
        let maxerr = iord[nrmax];
        let ermax = elist[maxerr];

        return ErrorOrderingOutput {
            max_error_index: maxerr,
            max_error: ermax,
            rank: nrmax,
        };
    }

    let mut index = None;
    for i in ibeg..=jbnd {
        let isucc = iord[i];
        // jump out of do-loop
        if errmax >= elist[isucc] {
            index = Some(i);
            break;
        }
        iord[i - 1] = isucc;
    }

    if index.is_none() {
        iord[jbnd] = maxerr;
        iord[jupbn - 1] = last - 1;

        // set maxerr and ermax.
        let maxerr = iord[nrmax];
        let ermax = elist[maxerr];

        return ErrorOrderingOutput {
            max_error_index: maxerr,
            max_error: ermax,
            rank: nrmax,
        };
    }

    let i = index.unwrap();

    // insert errmin by traversing the list bottom-up.
    iord[i - 1] = maxerr;
    let mut is_early_jump = false;
    let mut k = jbnd;

    for _j in i..=jbnd {
        let isucc = iord[k];

        // jump out of do-loop
        if errmin < elist[isucc] {
            is_early_jump = true;
            break;
        }

        iord[k + 1] = isucc;
        k = k - 1;
    }

    if is_early_jump {
        iord[k + 1] = last - 1;
    } else {
        iord[i] = last - 1;
    }

    // set maxerr and ermax.
    let maxerr = iord[nrmax];
    let ermax = elist[maxerr];

    ErrorOrderingOutput {
        max_error_index: maxerr,
        max_error: ermax,
        rank: nrmax,
    }
}

/// Represents the results of
///
/// `
/// i = integral of f over (a, b)
/// `
///
/// together with an error estimate, and
///
/// `
/// j = integral of abs(f) over (a, b)
/// `
struct QuadratureEstimate {
    /// `result` - Approximation to the integral `i`.
    result: f64,
    /// `abserr` - Estimate of the modulus of the absolute error, which should
    /// not exceed `abs(i - result)`.
    absolute_error: f64,
    /// `resabs` - Approximation to the integral `j`.
    result_abs: f64,
    /// `resasc` - Approximation to the integral of
    ///   `abs(f - i / (b - a))` over `(a, b)`.
    result_asc: f64,
}

/// Computes
///
/// `
/// i = integral of f over (a, b)
/// `
///
/// together with an error estimate, and
///
/// `
/// j = integral of abs(f) over (a, b)
/// `
///
/// The result is computed by applying the 21-point Kronrod rule (`resk`),
/// obtained by optimal addition of abscissae to the 10-point Gauss rule
/// (`resg`).
///
/// This is the double-precision version of the original `DQK21` Fortran
/// integration routine.
///
/// # Arguments
///
/// * `f` - Function defining the integrand `f(x)`.
/// * `a` - Lower limit of integration.
/// * `b` - Upper limit of integration.
///
/// # Returns
///
/// Returns the following values:
///
/// * `result` - Approximation to the integral `i`.
/// * `abserr` - Estimate of the modulus of the absolute error, which should
///   not exceed `abs(i - result)`.
/// * `resabs` - Approximation to the integral `j`.
/// * `resasc` - Approximation to the integral of
///   `abs(f - i / (b - a))` over `(a, b)`.
///
/// # Implementation details
///
/// * Original routine: `DQK21`
/// * Integration rule: 21-point Gauss-Kronrod rule
/// * Precision: double precision
/// * Date written: 1980-01-01
/// * Revision date: 1983-05-18
/// * Source: https://www.netlib.org/quadpack/dqk21.f
///
/// # Authors
///
/// * Robert Piessens, Applied Mathematics and Programming Division,
///   K.U. Leuven
/// * Elise de Doncker, Applied Mathematics and Programming Division,
///   K.U. Leuven
fn dqk21<F>(f: &F, a: f64, b: f64) -> QuadratureEstimate
where
    F: Fn(f64) -> f64,
{
    let mut fv1 = [0.0_f64; 10];
    let mut fv2 = [0.0_f64; 10];

    // The abscissae and weights are given for the interval (-1, 1).
    // Because of symmetry, only the positive abscissae and their
    // corresponding weights are given.
    //
    // xgk - Abscissae of the 21-point Kronrod rule.
    //       xgk(2), xgk(4), ...: abscissae of the 10-point Gauss rule.
    //       xgk(1), xgk(3), ...: abscissae which are optimally added
    //       to the 10-point Gauss rule.
    //
    // wgk - Weights of the 21-point Kronrod rule.
    //
    // wg - Weights of the 10-point Gauss rule.
    //
    // Gauss quadrature weights and Kronrod quadrature abscissae and weights
    // as evaluated with 80 decimal digit arithmetic by L. W. Fullerton,
    // Bell Labs, Nov. 1981.
    const WG: [f64; 5] = [
        0.0666713443_0868813759_3568809893_332e0,
        0.1494513491_5058059314_5776339657_697e0,
        0.2190863625_1598204399_5534934228_163e0,
        0.2692667193_0999635509_1226921569_469e0,
        0.2955242247_1475287017_3892994651_338e0,
    ];
    const XGK: [f64; 11] = [
        0.9956571630_2580808073_5527280689_003e0,
        0.9739065285_1717172007_7964012084_452e0,
        0.9301574913_5570822600_1207180059_508e0,
        0.8650633666_8898451073_2096688423_493e0,
        0.7808177265_8641689706_3717578345_042e0,
        0.6794095682_9902440623_4327365114_874e0,
        0.5627571346_6860468333_9000099272_694e0,
        0.4333953941_2924719079_9265943165_784e0,
        0.2943928627_0146019813_1126603103_866e0,
        0.1488743389_8163121088_4826001129_720e0,
        0.0000000000_0000000000_0000000000_000e0,
    ];
    const WGK: [f64; 11] = [
        0.0116946388_6737187427_8064396062_192e0,
        0.0325581623_0796472747_8818972459_390e0,
        0.0547558965_7435199603_1381300244_580e0,
        0.0750396748_1091995276_7043140916_190e0,
        0.0931254545_8369760553_5065465083_366e0,
        0.1093871588_0229764189_9210590325_805e0,
        0.1234919762_6206585107_7958109831_074e0,
        0.1347092173_1147332592_8054001771_707e0,
        0.1427759385_7706008079_7094273138_717e0,
        0.1477391049_0133849137_4841515972_068e0,
        0.1494455540_0291690566_4936468389_821e0,
    ];

    // list of major variables
    //
    // centr    -   mid point of the interval
    // hlgth    -   half-length of the interval
    // absc     -   abscissa
    // fval*    -   function value
    // resg     -   result of the 10-point gauss formula
    // resk     -   result of the 21-point kronrod formula
    // reskh    -   approximation to the mean value of f over (a,b),
    //              i.e. to i/(b-a)

    // machine dependent constants
    //
    // epmach is the largest relative spacing.
    // uflow is the smallest positive magnitude.
    let epmach = f64::EPSILON; // original: d1mach(4)
    let uflow = f64::MIN_POSITIVE; // original: d1mach(1)

    let centr = 0.5 * (a + b);
    let hlgth = 0.5 * (b - a);
    let dhlgth = hlgth.abs();

    // compute the 21-point kronrod approximation to
    // the integral, and estimate the absolute error.

    let mut resg = 0.0;
    let fc = f(centr);
    let mut resk = WGK[10] * fc;
    let mut resabs = resk.abs();

    for j in 0..5 {
        let jtw = 2 * j + 1;
        let absc = hlgth * XGK[jtw];
        let fval1 = f(centr - absc);
        let fval2 = f(centr + absc);
        fv1[jtw] = fval1;
        fv2[jtw] = fval2;
        let fsum = fval1 + fval2;
        resg = resg + WG[j] * fsum;
        resk = resk + WGK[jtw] * fsum;
        resabs = resabs + WGK[jtw] * (fval1.abs() + fval2.abs());
    }

    for j in 0..5 {
        let jtwm1 = 2 * j;
        let absc = hlgth * XGK[jtwm1];
        let fval1 = f(centr - absc);
        let fval2 = f(centr + absc);
        fv1[jtwm1] = fval1;
        fv2[jtwm1] = fval2;
        let fsum = fval1 + fval2;
        resk = resk + WGK[jtwm1] * fsum;
        resabs = resabs + WGK[jtwm1] * (fval1.abs() + fval2.abs());
    }

    let reskh = resk * 0.5;
    let mut resasc = WGK[10] * (fc - reskh).abs();

    for j in 0..10 {
        resasc =
            resasc + WGK[j] * ((fv1[j] - reskh).abs() + (fv2[j] - reskh).abs());
    }

    let result = resk * hlgth;
    resabs = resabs * dhlgth;
    resasc = resasc * dhlgth;
    let mut abserr = ((resk - resg) * hlgth).abs();

    if resasc != 0.0 && abserr != 0.0 {
        abserr = resasc * (1.0_f64).min((200.0 * abserr / resasc).powf(1.5));
    }

    if resabs > uflow / (50.0 * epmach) {
        abserr = ((epmach * 50.0) * resabs).max(abserr)
    }

    QuadratureEstimate {
        result,
        absolute_error: abserr,
        result_abs: resabs,
        result_asc: resasc,
    }
}
