//! # Sorting and collection keys
//!
//! `NHSNumber` derives the common comparison and hashing traits:
//!
//! - `PartialEq` + `Eq` — equality.
//! - `PartialOrd` + `Ord` — ordering.
//! - `Clone` + `Copy` — cheap duplication (the struct is 10 bytes).
//!
//! That means an `NHSNumber` plugs straight into any standard-library API
//! that expects a comparable, ordered, or copyable key:
//!
//! - `Vec::sort` / `Vec::sort_by_key`
//! - `BTreeSet<NHSNumber>`, `BTreeMap<NHSNumber, V>`
//! - `slice::binary_search`
//!
//! Ordering is lexicographic across the ten-element digit array. Because
//! every `NHSNumber` is the same length and stored most-significant digit
//! first, lexicographic order coincides with natural numeric order — so
//! sorted output reads the same way you would read it numerically.
//!
//! Run with:
//!
//! ```sh
//! cargo run --example sorting
//! ```

use nhs_number::NHSNumber;
use std::collections::BTreeSet;
use std::str::FromStr;

fn main() {
    // A deliberately shuffled set of inputs, with one repeated value so we
    // can demonstrate deduplication through `BTreeSet`.
    let inputs: [&str; 5] = [
        "999 123 4560",
        "999 000 0017",
        "999 999 9985",
        "999 500 0004",
        "999 123 4560", // duplicate — the BTreeSet will drop this
    ];

    // Parse each string into an `NHSNumber`, unwrap because every input here
    // is known to be well-formed. In production code you'd propagate the
    // `ParseError` via `?` or handle it explicitly.
    let parsed: Vec<NHSNumber> = inputs
        .iter()
        .map(|s| NHSNumber::from_str(s).unwrap())
        .collect();

    // === 1. Sort a Vec with `Vec::sort` ===
    //
    // `sort` calls `Ord::cmp` on the elements. Because `Ord` is derived for
    // `NHSNumber`, the comparison is element-wise on the digit array, which
    // matches numeric order. We clone so we don't mutate `parsed` in place —
    // the clone is cheap because `NHSNumber: Copy`, but `Vec::clone` itself
    // still has to allocate, so move where you can in hot paths.
    let mut sorted: Vec<NHSNumber> = parsed.clone();
    sorted.sort();
    println!("sorted ascending:");
    for n in &sorted {
        println!("  {}", n);
    }

    // === 2. Deduplicate + order with `BTreeSet` ===
    //
    // `BTreeSet<NHSNumber>` keeps elements unique and in order by the derived
    // `Ord`. We `.copied()` to turn `Iterator<Item = &NHSNumber>` (from
    // `.iter()`) into `Iterator<Item = NHSNumber>`, which is what
    // `FromIterator<NHSNumber>` for `BTreeSet<NHSNumber>` expects. This is
    // cheap because `NHSNumber: Copy`.
    let set: BTreeSet<NHSNumber> = parsed.iter().copied().collect();
    println!();
    println!("unique + sorted:");
    for n in &set {
        println!("  {}", n);
    }

    // === 3. Sanity checks on the results ===
    //
    // After deduplication we have four distinct numbers.
    assert_eq!(set.len(), 4);

    // `BTreeSet::iter().next()` yields the smallest element (lowest by `Ord`);
    // `.next_back()` yields the largest, since the iterator implements
    // `DoubleEndedIterator`.
    let smallest: &NHSNumber = set.iter().next().unwrap();
    let biggest: &NHSNumber = set.iter().next_back().unwrap();
    assert_eq!(smallest.to_string(), "999 000 0017");
    assert_eq!(biggest.to_string(), "999 999 9985");

    println!();
    println!("smallest: {}", smallest);
    println!("biggest:  {}", biggest);
    println!("ok");
}
