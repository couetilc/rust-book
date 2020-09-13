# Chapter 1

Getting set up with the Rust programming language and introducing you to many of
the CLI tools used during software development with Rust.

NOTE: Rust was began as a personal project at Mozilla by Graydon Hoare in 2006.
Three years later, Mozilla began sponsoring the project, and a self-hosting
compiler `rustc` was developed in 2011 using LLVM as a back-end. By 2015, the
first stable release 1.0 arrived. Many commented on Rust's elegance, and its
slow rise in popularity has been credited to its proclivity to change frequently
between versions.

# Installation

Rust versions are managed using the command line tool `rustup` (the rustacean
equivalent to Node.js' `nvm`).

## Installing `rustup` on Linux or MacOS

```sh
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

## Update `rustup` to the latest version

```sh
rustup update
```

## Uninstalling Rust and `rustup`

```sh
rustup self uninstal
```

## Troubleshooting

```sh
rustc --version
```

## Local Documentation

```sh
rustup doc
```

# Hello World!

Rust files end with the `.rs` extension. By convention, words in filenames are
separated by an underscore. So a hello world program written in Rust may be
called `hello_world.rs`.

## Compiling and running a Rust program

```sh
rustc hello_world.rs
./hello_world
```

## Starting point of a Rust program

A Rust program's initial code is defined by a special function, `main`, which is
always the first function invoked by an executable Rust program. One
implementation of the above Hello World! program is then

```rust
fn main () {
  println!("Hello World!");
}
```

# Cargo

Cargo is the build system and package manager for Rust (the equivalent in Node.js
is npm). As a Rust developer, Cargo is your primary tool for writing any size of
Rust software program. Cargo is installed when Rust is installed, and its version
may be managed through the `rustup` tool.

```sh
cargo --version
```

## Creating a project with Cargo

```sh
cargo new $project_name
```

A directory called $project_name is created with a git repo, a Cargo.toml file
analagous to a package.json in Node.js, a src directory meant for your source
code files, and a target directory meant for your build artifacts.

## Building a Cargo project

```sh
cargo build # outputs to target/debug/
```

## Running a Cargo program

```sh
cargo run
```

Cargo will check for changes and recompile your program if any are found.

## Checking a rust program

To compile your code to check for errors, but not output an executable, run

```sh
cargo check
```

## Build a release

```sh
cargo build --release # outputs to target/release/
```
