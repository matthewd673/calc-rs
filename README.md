# calc-rs

Simple calculator implemented in Rust to learn the language a bit.

## Usage
`cargo run` in the directory

## Features
- Basic operations (`+`, `-`, `*`, `/`, `^`)
- Decimal numbers
- Variables (assigned with `:`)
  - Including "reserved" variables (`ANS`, `PI`, `RAND`)
- PEMDAS (and grouping with `(` & `)`)
- Comparators (`=`, `>`, `<`)
- Functions

### Basic operations

Basic mathematical operations can be performed with the supported operators: `+`, `-`, `*`, `/`, `^`.

**Example:**

    9 + 10
    = 19


PEMDAS is followed, and groupings (**Ex:** `(1 + 1) * 2`) are prioritized.

### Decimal numbers

Decimal numbers are supported, so long as the decimal is preceded by a digit.

**Example:**

    0.5 * 3
    = 1.5

### Variables

Variables can be assigned with the `:` operator. Once assigned, they behave the same as any other numeric value. Unassigned variables will evaluate to `0`.

**Example:**

    MYVAR : 9
    = 9
    MYVAR + 10
    = 19
    MYVAR * FAKEVAR
    = 0

Additionally, some variables are reserved and have special behavior:

- **`ANS`:** evaluates to the last answer produced (initially `0`)
- **`PI`:** evaluates to `3.14`
- **`RAND`:** evaluates to a random float in the range `[0, 1)`

### Comparators

Comparators return either `0` or `1`, (`1` = true, `0` = false):

- **`A = B`:** returns `1` if surrounding values are equivalent
- **`A > B`:** returns `1` if value `A` is greater than value `B`
- **`A < B`:** returns `1` if value `A` is less than value `B`

### Functions

Functions are similar to variables, however they will evaluate a given expression every time they are referenced. Once assigned, they behave the same as any other numeric value. Unassigned functions will evaluate to `0`. Function names are always preceded with a `!`, otherwise they will be treated as ordinary variables.

**Example:**

    !MYFUNC : ANS * 2 }
    = 0
    9 + 10
    = 19
    !MYFUNC
    = 38
    !NEWFUNC : !MYFUNC + 10 }
    = 0
    10
    = 10
    !NEWFUNC
    = 30

Tokens will continue to be added to the function expression until a `}` is encountered.