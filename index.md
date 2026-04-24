# NHS Number

**[documentation](https://docs.rs/nhs-number/)**
•
**[source](https://github.com/GIG-Cymru-NHS-Wales/nhs-number-using-rust)**
•
**[llms.txt](https://raw.githubusercontent.com/GIG-Cymru-NHS-Wales/nhs-number-using-rust/refs/heads/main/llms.txt)**
•
**[crate](https://crates.io/crates/nhs-number)**
•
**[email](mailto:joel.henderson@wales.nhs.uk)**

A National Health Service (NHS) Number is a unique number allocated in a shared
numbering scheme to registered users of the three public health services in
England, Wales, and the Isle of Man.

The NHS Number is the key to the identification of patients, especially in
delivering safe care across provider organisations, and is required in all new
software deployed within the National Health Service (NHS) organizations.

References:

* [National Health Service (NHS)](https://en.wikipedia.org/wiki/National_Health_Service)

* [NHS Number](https://en.wikipedia.org/wiki/NHS_number)

## Syntax

The current system uses a ten-digit number in '3 3 4' format with the final
digit being an error-detecting checksum. An example is 999 123 4560.

## Ranges

Currently issued numbers are in these ranges:

* 300 000 000 to 399 999 999 (England)

* 400 000 000 to 499 999 999 (England, Wales, Isle of Man)

* 600 000 000 to 799 999 999 (England, Wales, Isle of Man)

Unavailable number ranges include:

* 320 000 001 to 399 999 999 (allocated to the Northern Irish system)

* 010 100 0000 to 311 299 9999 (used for CHI numbers in Scotland)

For test purposes, this range is valid but is guaranteed to never be issued:

* 999 000 0000 to 999 999 9999

## Checksum

The checksum is calculated by multiplying each of the first nine digits by 11
minus its position. Using the number 943 476 5919 as an example:

* The first digit is 9. This is multiplied by 10.

* The second digit is 4. This is multiplied by 9.

* And so on until the ninth digit (1) is multiplied by 2.

* The result of this calculation is summed. In this example: (9×10) + (4×9) +
  (3×8) + (4×7) + (7×6) + (6×5) + (5×4) + (9×3) + (1×2) = 299.

* The remainder when dividing this number by 11 is calculated, yielding a number
  in the range 0–10, which would be 2 in this case.

* Finally, this number is subtracted from 11 to give the checksum in the range
  1–11, in this case 9, which becomes the last digit of the NHS Number.

* A checksum of 11 is represented by 0 in the final NHS Number. If the checksum
  is 10 then the number is not valid.

## Examples

```rust
use nhs_number::*;
use std::str::FromStr;

// NHS Number that we can use for testing purposes
let str = "999 123 4560";

// Create a new NHS Number by converting from a string.
let nhs_number = NHSNumber::from_str(str).unwrap();

// Validate a NHS Number using the check digit algorithm.
let valid: bool = nhs_number.validate_check_digit();
```
