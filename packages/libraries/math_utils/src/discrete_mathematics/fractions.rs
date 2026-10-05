use crate::F64Extension;

/// Closest Fraction to self with denominator at most max_denominator.
///
/// # Reference
/// Code: <https://github.com/python/cpython/blob/main/Lib/fractions.py>
/// * `limit_denominator(self, max_denominator=1000000)`
pub fn integer_ratio_with_limit_denominator(
    value: f64,
    max_denominator: Option<u128>,
) -> Option<(i128, u128)> {
    // Algorithm notes: For any real number x, define a *best upper
    // approximation* to x to be a rational number p/q such that:
    //
    //   (1) p/q >= x, and
    //   (2) if p/q > r/s >= x then s > q, for any rational r/s.
    //
    // Define *best lower approximation* similarly.  Then it can be
    // proved that a rational number is a best upper or lower
    // approximation to x if, and only if, it is a convergent or
    // semiconvergent of the (unique shortest) continued fraction
    // associated to x.
    //
    // To find a best rational approximation with denominator <= M,
    // we find the best upper and lower approximations with
    // denominator <= M and take whichever of these is closer to x.
    // In the event of a tie, the bound with smaller denominator is
    // chosen.  If both denominators are equal (which can happen
    // only when MAX_DENOMINATOR == 1 and self is midway between
    // two integers) the lower bound---i.e., the floor of self, is
    // taken.
    let max_denominator = max_denominator.unwrap_or(1_000_000);

    if max_denominator < 1 {
        // max_denominator should be at least 1
        return None;
    }

    let (numerator, denominator) = value.as_integer_ratio()?;

    let is_negative = numerator.is_negative();
    let abs_numerator = numerator.unsigned_abs();

    if denominator <= max_denominator {
        return Some((apply_sign(abs_numerator, is_negative)?, denominator));
    }

    let mut p0 = 0u128;
    let mut q0 = 1u128;
    let mut p1 = 1u128;
    let mut q1 = 0u128;
    let (mut n, mut d) = (abs_numerator, denominator);

    loop {
        let a = n / d;
        let q2 = q0.checked_add(a.checked_mul(q1)?)?;
        if q2 > max_denominator {
            break;
        }
        (p0, q0, p1, q1) = (p1, q1, p0.checked_add(a.checked_mul(p1)?)?, q2);
        (n, d) = (d, n.checked_sub(a.checked_mul(d)?)?);
    }
    let k = (max_denominator - q0) / q1;

    // Determine which of the candidates (p0+k*p1)/(q0+k*q1) and p1/q1 is
    // closer to value. The distance between them is 1/(q1*(q0+k*q1)), while
    // the distance from p1/q1 to value is d/(q1*self._denominator). So we
    // need to compare 2*(q0+k*q1) with denominator/d.
    let lhs = 2u128
        .checked_mul(d)?
        .checked_mul(q0.checked_add(k.checked_mul(q1)?)?)?;
    let choose_p1 = lhs <= denominator;

    // In the event of a tie, CPython chooses the bound with smaller denominator.
    // If both denominators are equal, the lower bound is chosen. Since we compute
    // using abs(value), the lower bound for negative values is `-bound1`.
    let is_equal_denominator_tie = is_negative
        && lhs == denominator
        && q0.checked_add(k.checked_mul(q1)?)? == q1;

    if choose_p1 && !is_equal_denominator_tie {
        Some((apply_sign(p1, is_negative)?, q1))
    } else {
        Some((
            apply_sign(p0.checked_add(k.checked_mul(p1)?)?, is_negative)?,
            q0.checked_add(k.checked_mul(q1)?)?,
        ))
    }
}

fn apply_sign(value: u128, is_negative: bool) -> Option<i128> {
    if is_negative {
        if value == (1u128 << 127) {
            Some(i128::MIN)
        } else {
            let value = i128::try_from(value).ok()?;
            Some(-value)
        }
    } else {
        i128::try_from(value).ok()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn pi_with_max_denominator_10() {
        assert_eq!(
            integer_ratio_with_limit_denominator(PI, Some(10)),
            Some((22, 7))
        );
    }

    #[test]
    fn pi_with_max_denominator_100() {
        assert_eq!(
            integer_ratio_with_limit_denominator(PI, Some(100)),
            Some((311, 99))
        );
    }

    #[test]
    fn float_approximation_recovers_fraction_when_denominator_is_within_limit()
    {
        let value = 4321.0 / 8765.0;

        assert_eq!(
            integer_ratio_with_limit_denominator(value, Some(10_000)),
            Some((4321, 8765))
        );
    }

    #[test]
    fn exact_fraction_when_denominator_is_within_limit() {
        assert_eq!(
            integer_ratio_with_limit_denominator(0.1, None),
            Some((1, 10))
        );
    }

    #[test]
    fn test_negative_edge_case() {
        assert_eq!(
            integer_ratio_with_limit_denominator(-1.5, Some(1)),
            Some((-2, 1))
        );
    }
}
