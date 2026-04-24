# AGENTS.md

Guidance for AI coding agents (Claude Code, Copilot, Cursor, Aider, etc.) working
in this repository.

## Project snapshot

- **Crate:** `nhs-number`
- **Purpose:** Model, parse, format, and validate National Health Service (NHS)
  Numbers for NHS England, NHS Wales, and NHS Isle of Man.
- **Language:** Rust (edition 2024)
- **License:** MIT OR Apache-2.0 OR GPL-2.0 OR GPL-3.0 OR BSD-3-Clause
- **Runtime dependencies:** `rand`, `serde` (with `derive`)
- **Repository:** https://github.com/GIG-Cymru-NHS-Wales/nhs-number-using-rust
- **Crate:** https://crates.io/crates/nhs-number
- **Docs:** https://docs.rs/nhs-number/

## Repository layout

```
.
├── AGENTS.md              # This file (instructions for AI agents)
├── CITATION.cff           # Citation metadata
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── Cargo.toml
├── README.md
├── cspell.json            # Spell-check dictionary
├── docs/                  # Reference documentation
│   ├── api/index.md       # API reference
│   ├── checksum/index.md  # Check-digit algorithm
│   ├── faq/index.md       # Frequently asked questions
│   ├── ranges/index.md    # Valid NHS Number ranges
│   └── usage/index.md     # Usage guide
├── examples/              # Runnable `cargo run --example <name>` examples
│   ├── README.md          # Index of examples
│   ├── basic_usage.rs
│   ├── parsing.rs
│   ├── validation.rs
│   ├── testable.rs
│   ├── sorting.rs
│   ├── generate_valid.rs
│   └── bulk_processing.rs
├── help/
│   └── releasing/         # Release checklist
├── llms.txt               # LLM-friendly crate docs (generated)
├── llms.json              # Machine-readable crate docs (generated)
└── src/
    ├── lib.rs             # Crate root, `NHSNumber` struct, free functions
    ├── from_str.rs        # `FromStr` parser
    ├── parse_error.rs     # `ParseError` type
    └── testable.rs        # Testable range 999 000 0000 – 999 999 9999
```

## Core data model

```rust
pub struct NHSNumber {
    pub digits: [i8; 10],
}
```

Each digit is a single decimal (0–9) stored as `i8`. The canonical display
format is `"DDD DDD DDDD"` (three, three, four, with single spaces).

## Key APIs

Methods on `NHSNumber`:

- `NHSNumber::new(digits: [i8; 10]) -> NHSNumber`
- `NHSNumber::check_digit(&self) -> i8`
- `NHSNumber::calculate_check_digit(&self) -> i8`
- `NHSNumber::validate_check_digit(&self) -> bool`
- `NHSNumber::testable_random_sample() -> NHSNumber`

Trait implementations:

- `Display`, `Into<String>` — format as `"DDD DDD DDDD"`.
- `FromStr` — parse `"DDDDDDDDDD"` or `"DDD DDD DDDD"`.
- `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`,
  `Serialize`, `Deserialize`.

Free functions (equivalent to the methods above):

- `format(digits) -> String`
- `check_digit(digits) -> i8`
- `calculate_check_digit(digits) -> i8`
- `validate_check_digit(digits) -> bool`
- `testable_random_sample() -> NHSNumber`

Constants (in `testable` module):

- `TESTABLE_MIN` — `999 000 0000`
- `TESTABLE_MAX` — `999 999 9999`
- `TESTABLE_RANGE_INCLUSIVE`

## Common commands

```sh
cargo build                      # Build
cargo build --release            # Release build
cargo test                       # Run unit + doctest suite
cargo test -- --nocapture        # Show println!() from tests
cargo doc --no-deps --open       # Build and open rustdoc
cargo run --example basic_usage  # Run an example
cargo clippy --all-targets       # Lint (if rustup has clippy)
cargo fmt                        # Format
```

## Test pattern

Unit tests live alongside the code they test, inside `#[cfg(test)] mod tests`.
Doc-tests live in `///` rustdoc comments and run with `cargo test`.

Tests follow a consistent `actual` / `expect` pattern:

```rust
let actual = ...;
let expect = ...;
assert_eq!(actual, expect);
```

When adding a new public API, **always** add both a unit test and a doc-test.

## Style conventions

- Use 4-space indentation.
- Prefer explicit types on bindings when they improve readability
  (`let actual: NHSNumber = …;`).
- Follow the existing doc comment style:
  - One-sentence summary on the first line.
  - Blank line.
  - `Example:` section with a runnable doc-test.
  - Blank line.
  - Cross-reference to the equivalent method or function.
- Use `#[allow(dead_code)]` on public items only if they are re-exported but not
  called internally (existing pattern in this crate).
- Prefer `[i8; 10]` over `Vec<i8>` — digits are always fixed-length.

## Constraints for agents

1. **Never invent an NHS Number.** Use only:
   - Documentation examples from the Wikipedia article on NHS numbers
     (e.g. `943 476 5919`, `987 654 4321`).
   - The testable range `999 000 0000 – 999 999 9999`, which is valid but
     guaranteed to never be issued.
2. **Do not weaken the check-digit validator.** Patients' safety relies on
   correct validation. Changing the algorithm requires a reference to the
   [NHS Number specification](https://en.wikipedia.org/wiki/NHS_number).
3. **Do not commit generated files blindly.** `llms.txt` and `llms.json` are
   regenerated from `cargo doc` output via `rustdoc-llms` during the release
   process (see `help/releasing/README.md`).
4. **Keep the public API stable.** This crate is published on crates.io; any
   breaking change needs a major version bump per semver.
5. **Preserve multi-license headers.** All source files are multi-licensed
   (MIT, Apache-2.0, GPL-2.0, GPL-3.0, BSD-3-Clause).

## Release process

See `help/releasing/README.md`. Summary:

1. Bump `version` in `Cargo.toml`.
2. `cargo build --release && cargo test && cargo doc`.
3. Regenerate LLM docs: `rustdoc-llms && cp target/doc/nhs_number.json llms.json && cp target/doc/llms.txt llms.txt`.
4. Commit, tag (matching `Cargo.toml` version), push tags, `cargo publish`.

## Contact

Joel Parker Henderson — joel.henderson@wales.nhs.uk
