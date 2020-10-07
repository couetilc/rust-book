# Chapter 4: Understanding Ownership

Rust's most unique feature, ownership, enables memory safety guarantees without
a garbage collector.

## What is Ownership?

A central feature of rust, "ownership" is the feature Rust has to replace a
typical garbage collector or manual memory management. Ownership is a set of
rules the compiler checks at compile time.

### The Stack and the Heap

Whether a value is on the stack or heap affects how the language behaves and
what decisions you can make. They are different structures of memory, the stack
stores values in one order, and retrieves them in the reverse order, AKA
"first-in last-out". All data stored on the stack must have a known, fixed size.
If you have data whose size changes or is unknown, you must request a certain
amount of memory on the heap. The memory allocator in Rust returns a pointer to
a spot in the heap and marks it as used, AKA "allocating on the heap".

Pushing to the stack is faaster than allocating on the heap because stacks do
not need to search for an empty spot, they place it on the top of the stack.
There is more bookkeeping with heap operations. Memory access is slower too,
following pointers can cause a processor to jump around in memory inefficiently
and allocating a big portion of the heap can take time. Functions calls push
their local variables onto the stack at execution and pop them off afterwards.

Ownership addresses problems with tracking access to the same memory at
different parts of the code base, minimizing duplicate data on the heap, and
cleaning up unused data. Ownership clarifies the concept of the stack and the
heap in Rust.

## Ownership Rules

1. Each value in Rust has a variable that's called its "owner".
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped

## Variable Scope

A variable's scope is the place in a program where it may be used and accessed.

With regard to ownership, a variable is
1) valid when coming into scope
2) invalid when coming out of scope

## The `String` type

The `String` type is considered more complicated than the previous primitive
data types, as it can have a variable size and must therefore be allocated on
the heap. As it is allocated on the heap, it may be mutated, in contrast to the
immutable string literals often found in programs which are inlined into the
final executable.

## Memory and Allocation

To allocate memory for a mutable and growable piece of text
1) the memory must be requested from the memory allocator at runtime
2) the memory must be returned to the memory allocated when it becomes unused

`String` allocation can be performed in Rust by calling `String::from` on a
string literal. Rust ties deallocation to variable scope. Once a variable on
the heap goes out of scope, the memory is freed. `String` has a method `drop`
that is called when a `String` variable drops out of scope, its purpose is to
free the unused memory.

## Ways Variables and Data Interact: Move

Because variables allocated on the heap are in some sense pointers on the
stack, if you track which variables those pointers are moved into, such as
through an assignment operations, you can efficiently deallocate the memory
after the variables containing its reference pointer are out of scope. By
restricting the use of older variables containing a pointer, so only the newest
variable containing it can be used, problems such as "double free" errors can
be avoided. The memory will only be freed when the newest variable containing
its reference pointer goes out of scope. This allows data to be efficiently
passed and accessed around a program without deep copying data structures or
portions of memory.

## Ways Variables and Data Interact: Clone

You can deep copy heap data by calling the method `clone` on a `String`. Calling
`clone` may execute expensive code.

## Stack-Only Data: Copy

Variables allocated on the stack, such as integers, are not subject to the
rules for moving around pointers to heap data. Rust has a special annotation
called the `Copy` trait that integers implement. Data allocated on the heap
have a `Drop` trait, and no data type can implement both the `Drop` and `Copy`
traits.

## Ownership and Functions

Passing a value to a function as an argument is a form of moving the variable,
and is subject to ownership rules. The compiler will track the pointer as it
moves through the different variable scopes, eventually freeing its memory when
the final variable goes out of scope. Stack variables passed around will simply
be copied.

## Return Values and Scope

Returning a variable from a function also transfers its ownership. It is as if
you assigned the value to another variable, that is, moved it.

Passing ownership around by returning pointer variables from functions is
tedious and clunky, but Rust has a feature called "references" that addresses
this issue with passing heap data to functions.