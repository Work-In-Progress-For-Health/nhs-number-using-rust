//! # Testable range
//!
//! The NHS reserves a specific ten-digit range — `999 000 0000` through
//! `999 999 9999` — for testing. Numbers in this range are valid by the
//! check-digit algorithm but are guaranteed never to be issued to a real
//! patient, so they are safe to hard-code in:
//!
//! - unit tests and integration tests,
//! - demos and training material,
//! - fixtures, seed data, and API documentation.
//!
//! This example shows:
//!
//! 1. The three `LazyLock` statics that expose the range bounds and
//!    membership check.
//! 2. Both styles of random sampling (method vs. free function).
//! 3. How to turn a random sample into a *valid* sample by looping on
//!    `validate_check_digit`.
//!
//! Run with:
//!
//! ```sh
//! cargo run --example testable
//! ```

use nhs_number::NHSNumber;
// The three statics describing the testable range. `LazyLock<T>` means the
// value is computed once on first use and then cached — you `*`-dereference
// to get the underlying `T` (here, `NHSNumber` or `RangeInclusive<NHSNumber>`).
use nhs_number::testable::{TESTABLE_MAX, TESTABLE_MIN, TESTABLE_RANGE_INCLUSIVE};

fn main() {
    // === 1. The bounds ===
    //
    // `TESTABLE_MIN` and `TESTABLE_MAX` are `LazyLock<NHSNumber>` statics.
    // They are computed on first access (which happens here when the `*`
    // dereference kicks in) and then reused.
    println!("testable min: {}", *TESTABLE_MIN);
    println!("testable max: {}", *TESTABLE_MAX);

    assert_eq!(TESTABLE_MIN.to_string(), "999 000 0000");
    assert_eq!(TESTABLE_MAX.to_string(), "999 999 9999");

    // === 2. Range membership ===
    //
    // `TESTABLE_RANGE_INCLUSIVE` is a `LazyLock<RangeInclusive<NHSNumber>>`,
    // which means it works with the standard `.contains()` method. Note the
    // `&` in front of `sample` — `RangeInclusive::contains` takes its
    // argument by reference.
    let sample: NHSNumber = NHSNumber::new([9, 9, 9, 0, 1, 2, 3, 4, 5, 6]);
    assert!(TESTABLE_RANGE_INCLUSIVE.contains(&sample));

    // === 3. Random sampling — method style ===
    //
    // `NHSNumber::testable_random_sample()` is an associated function. The
    // first three digits are pinned to 9, 9, 9 so every draw lands in the
    // testable range; the remaining seven are drawn from `rand::rng()`.
    for _ in 0..5 {
        let sample: NHSNumber = NHSNumber::testable_random_sample();
        println!("sample: {}", sample);
        assert!(sample >= *TESTABLE_MIN);
        assert!(sample <= *TESTABLE_MAX);
    }

    // === 4. Random sampling — free-function style ===
    //
    // Identical behaviour to the method form. Same numbers, same guarantees.
    for _ in 0..5 {
        let sample: NHSNumber = nhs_number::testable_random_sample();
        assert!(TESTABLE_RANGE_INCLUSIVE.contains(&sample));
    }

    // === 5. Getting a *valid* testable sample ===
    //
    // Important: the random sampler draws the tenth digit uniformly too, so
    // only ~1 in 10 random samples has a correct check digit. If your test
    // needs a testable number that also passes `validate_check_digit`,
    // resample in a loop until you get one. The loop is cheap (on average
    // ten iterations, each a handful of i8 multiplies) and deterministic
    // enough for test use.
    let valid_sample: NHSNumber = loop {
        let candidate: NHSNumber = NHSNumber::testable_random_sample();
        if candidate.validate_check_digit() {
            break candidate;
        }
    };
    println!("valid testable sample: {}", valid_sample);
    assert!(valid_sample.validate_check_digit());

    // For a deterministic valid sample (no loop, no randomness), pick the
    // first nine digits yourself and compute the tenth with
    // `calculate_check_digit`. See `examples/generate_valid.rs`.

    println!("ok");
}
