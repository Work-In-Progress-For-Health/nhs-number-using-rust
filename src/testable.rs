use crate::NHSNumber;
use rand::RngExt;
use std::ops::RangeInclusive;
use std::sync::LazyLock;

/// Get the NHS Number testable range minimum value.
/// This number is valid but is never going to be issued.
///
/// Example:
///
/// ```rust
/// use nhs_number::NHSNumber;
/// use nhs_number::testable::TESTABLE_MIN;
/// let nhs_number = NHSNumber { digits: [9, 9, 9, 0, 1, 2, 3, 4, 5, 6] };
/// assert!(nhs_number >= *TESTABLE_MIN);
/// ```
///
#[allow(dead_code)]
pub static TESTABLE_MIN: LazyLock<NHSNumber> = LazyLock::new(|| NHSNumber {
    digits: [9, 9, 9, 0, 0, 0, 0, 0, 0, 0],
});

/// Get the NHS Number testable range maximum value.
/// This number is valid but is never going to be issued.
///
/// Example:
///
/// ```rust
/// use nhs_number::NHSNumber;
/// use nhs_number::testable::TESTABLE_MAX;
/// let nhs_number = NHSNumber { digits: [9, 9, 9, 0, 1, 2, 3, 4, 5, 6] };
/// assert!(nhs_number <= *TESTABLE_MAX);
/// ```
///
#[allow(dead_code)]
pub static TESTABLE_MAX: LazyLock<NHSNumber> = LazyLock::new(|| NHSNumber {
    digits: [9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
});

/// Get the NHS Number testable range.
/// This range is valid but is never going to be issued.
///
/// Example:
/// ```rust
///  use nhs_number::{NHSNumber, testable::*};
/// let nhs_number = NHSNumber { digits: [9, 9, 9, 0, 1, 2, 3, 4, 5, 6] };
///  assert!(TESTABLE_RANGE_INCLUSIVE.contains(&nhs_number));
/// ```
#[allow(dead_code)]
pub static TESTABLE_RANGE_INCLUSIVE: LazyLock<RangeInclusive<NHSNumber>> =
    LazyLock::new(|| RangeInclusive::new(*TESTABLE_MIN, *TESTABLE_MAX));

/// Generate a NHS Number testable range random sample.
/// The generated number is valid but is never going to be issued.
///
/// Example:
///
/// ```rust
/// use nhs_number::{NHSNumber, testable::*};
/// let nhs_number = testable_random_sample();
/// assert!(nhs_number >= *TESTABLE_MIN);
/// assert!(nhs_number <= *TESTABLE_MAX);
/// ```
///
#[allow(dead_code)]
pub fn testable_random_sample() -> NHSNumber {
    let mut rng = rand::rng();
    NHSNumber {
        digits: [
            9,
            9,
            9,
            rng.random_range(0..=9) as i8,
            rng.random_range(0..=9) as i8,
            rng.random_range(0..=9) as i8,
            rng.random_range(0..=9) as i8,
            rng.random_range(0..=9) as i8,
            rng.random_range(0..=9) as i8,
            rng.random_range(0..=9) as i8,
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        let a = testable_random_sample();
        assert!(a >= *TESTABLE_MIN);
        assert!(a <= *TESTABLE_MAX);
    }
}
