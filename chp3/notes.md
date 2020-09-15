# Chapter 3: Common Programming Concepts

## Variables and Mutability

By default variables are immutable and declared with `let`. To create a mutable
variable, declare one using `let mut`. Variables declared with `const` must have
a type definition and are immutable. These variables can be declared in any
scope. Shadowing is a simple way to assign different types to the same variable
name within a scope so that `let x = 3; let x = 'three';` are together a valid
statement.

## Data Types

As a statically typed language, every value in Rust has a data type which is
part of one of two sets, scalar or compound (one value or many values). The
compiler will often infer the data type allowing the programmer to omit type
annotations. In fact, Rust requires type annotations for function parameters to
make inference more reliable. If there is any ambiguity as to what type a value
is, the Rust compiler will require a type annotation.

### Scalar Types

A scalar type represents a single value, of which there are four primary forms:
integers, floating-point numbers, booleans, and characters.

#### Integer Types

Integers can be signed or unsigned (prefixed with `i` or prefixed with `u`) and
come in 8, 16, 32, 64, and 128-bit sizes. For example, to declare a variable
type as an unsigned 32-bit value, simply annotate it with `u32`.  There is an
architecture specific integer type called `size` which is rarely used. Rust's
default integer type is `i32`, a signed 32-bit integer, which is generally the
fastest (even on 64-bit systems). Signed numbers are stored using two's
complement representation.

NOTE: Two's complement is the binary system where a N-bit number summed with
it's negative representation equal the number 2^N.  Practically, this scheme is useful
as addition, subtraction, and multiplication will operate the same on positive
and negative numbers. Exceptionally, the most negative number in any N-bit two's
complement scheme cannot be inverted, as its inversion will result in itself
again, in the same sense as how +0 and -0 are the same number. Take care to
account for the most negative number phenomenon in numerical calculations.
As an example, take +3 (011) and -3 (101). When added, 011 + 101 = 1000,
equivalent to 8 in base 10 or 2^3.

##### Integer Overflow

When compiling in debug mode, Rust will panic when integer overflow occurs. When
compiling in release mode, Rust performs two's complement wrapping and will not
panic. Take care to account for overflow when perform numerical calculations.

#### Floating-Point Types

Numbers with decimal points are represented according to the IEEE-754 standard.
There are two floating-point types available, `f32` and `f64`, a single
precision float and double precision float, respectively.

#### Numeric Operations

The following infix numerical operators are supported: addition (`+`),
substraction (`-`), multiplication (`*`), division (`/`), and remainder (`%`).

#### Boolean Type

A boolean has two possible values in Rust, `true` or `false`, and the 1-bit
variable is specified by the type annotation `bool`.

#### Character Type

Rust's `char` type is a 4-bit value representing a Unicode Scalar Value
Recognize `U+2665`, or ♥? These can be accented letters, chinese/japanese/korean
characters, emoji, etc. Now, the concept of a "character" is not clearly defined
in Unicode, so take care when working with UTF encode strings.

### Compound Types

#### Tuple Type

#### Array Type

## Functions

### Function Parameters

### Function Bodies Contain Statements and Expressions

### Functions with Return Values

## Comments

## Control Flow

### `if` Expressions

### Repetition with Loops
