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
it's negative representation equalls the number 2^N.  Practically, this scheme
is useful as addition, subtraction, and multiplication will operate the same on
positive and negative numbers. Exceptionally, the most negative number in any
N-bit two's complement scheme cannot be inverted, as its inversion will result
in itself again, in the same sense as how +0 and -0 are the same number. Take
care to account for the most negative number phenomenon in numerical
calculations.  As an example of two's complement representation, take +3 (011)
and -3 (101).  When added, 011 + 101 = 1000, equivalent to 8 in base 10 or 2^3.

##### Integer Overflow

When compiling in debug mode, Rust will panic when integer overflow occurs. When
compiling in release mode, Rust performs two's complement wrapping and will not
panic. Take care to account for overflow when perform numerical calculations.

#### Floating-Point Types

Numbers with decimal points are represented according to the IEEE-754 standard.
There are two floating-point types available, `f32` and `f64`, a single
precision float and double precision float, respectively. The default floating
point type is `f64` as it has similar performance single precisions floats but
has more precision.

#### Numeric Operations

The following infix numerical operators are supported: addition (`+`),
substraction (`-`), multiplication (`*`), division (`/`), and remainder (`%`).

#### Boolean Type

A boolean has two possible values in Rust, `true` or `false`, and the 1-bit
variable is specified by the type annotation `bool`.

#### Character Type

Rust's `char` type is a 4-bit value representing a Unicode Scalar Value.
Recognize `U+2665`, or ♥? These can be accented letters, chinese/japanese/korean
characters, emoji, etc. Now, the concept of a "character" is not clearly defined
in Unicode, so take care when working with UTF encoded strings.

### Compound Types

Rust has two primitive compound types: tuples and arrays.

#### Tuple Type

A tuple groups a number of different data types into a fixed length data
structure that cannot change size. Tuples are declared with the annotation `()`
and contains type declarations in particular order. Tuple elements may be
destructured using pattern matching, or directly accessed by appending a period
to the variable name followed by the element's index (starting at 0).

#### Array Type

An array type is a fixed-length collection of values of the same type allocated
on the stack (vectors are an analagous data type from the standard library that
may change size). Arrays are declared with the annotation `[<type>;<size>]`
(e.g. `[u32; 2]`), or may be initialized using the syntax `let x = [<value>;<size]`
(e.g. `let x = [4; 2] // => [4, 4]`). Array elements may be accessed by appending
the element's index surrounded by brackets to the variable name. During runtime,
if you try to access an element outside the array's length, Rust will panic.

## Functions

Functions can be declared using the `fn` keyword, followed by the name and a
set of parentheses containing its parameters, ended by a pair of brackets
containing the body of the function. The `main` function is the entrypoint for
most Rust programs. The convention for naming functions is to use snake case.

### Function Parameters

Parameters are variables that are part of the function's signature and must have
type annotations. Arguments are the particular values passed through these
parameters by a function call. Function parameters are separated by commas.

### Function Bodies Contain Statements and Expressions

Function bodies are a series of statements and expressions. Statements perform
an action and do not return a value. Expressions, on the other hand, return a
value. If you end a function with an expression and omit the semicolon, the
value of the expression will be returned as the result of the function call. The
compiler will throw an error if you attempt to use a statement as an
expression (by treating it as if it returns a value). Expressions are the
preponderance of instructions in a Rust program. For example, blocks or `{}`,
macros, and arithmetic operations. By appending a semicolon to the end of an
expression, you turn it into a statement.

### Functions with Return Values

A function's return type, if it returns anything, is declared after the parens
with the syntax `-> <type>`.

## Comments

Comments in rust begin with `//` and are the preferred convention. Multi-line
comments exist and begin with `/*` ending in `*/`.

## Control Flow

There are two primary means of control flow in rust, conditional expressions and
looping constructs.

### `if` Expressions

One kind of conditional expression starts with the keyword `if` followed by a
condition then a block, no need for parens. An optional `else` expression may
be appended followed by another block. Multiple conditions can be chained using
the keyword `else if` followed by the next condition and another block after
the initial `if` expression. Note that because `if` is an expression, it may be
used on the right side of a variable assignment.

### Repetition with Loops

Rust has three kinds of loops: `loop`, `while`, and `for`.

#### Repeating Code with `loop`

A `loop` block will execute over and over again until explicity told to stop,
usually by using `break` or `continue`. `loop` is an expression, and so can
return a value by using the `break` keyword followed by the value. A loop is
used by writing `loop` followed by a block expression.

#### Conditional Loops with `while`

A `while` loop will continue executing as long as a particular condition is
met.  A `while` loop is used by writing `while` followed by a condition then a
block expression.

#### Looping through a Collection with `for`

`for` loops are a concise alternative to `while` or `loop` when iterating over
a collection of values. A `for` loop is used by writing `for` followed by a
variable name, `in`, an iterable collection, then a block expression. `for` loops
are the most commonly used looping construct in rust, and it is recommended any
time you use a `loop` or `while` to consider turning it into a `for` loop, as
it requires you to specify variable allocation, iteration, and a termination
condition as part of the construct.

## Recommended programs TODO

- convert temperature between Fahrenheit and Celsius
- Generate the nth Fibonacci number
- Print the lyrics to the Christmas carol "The Twelve Days of Christmas," taking
  advantage of the repetition in the song
