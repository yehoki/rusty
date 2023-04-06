# rusty
All Things rust: Self learning rust

This repo stores all basic things I will be using to learn rust syntax and any general things I think would help me in the future.

## First tutorial

The first tutorial will be followed from [Rust by example](https://doc.rust-lang.org/stable/rust-by-example/index.html) and [The Rust Book](https://doc.rust-lang.org/book/title-page.html)

We begin with the `main()` function which takes in no arguments, and is where the execution of the program begins.

To print any text we can use the `println!` macro - in the Rust programming language, macros are defined by an exclamation mark `!` at the end.
### Cargo

Cargo is Rust's build system and package manager.

`cargo -- version`: Checks the current cargo version

`cargo new {directory_name}`: Makes a new cargo project

`cargo build`: Build the project of the directory you are currently in

When the project is built, it is stored in the `./target/debug/` directory.

`cargo build --release` can be used to build with optimizations

`cargo run`: Compiles the code and runs the resulting executable

`cargo check`: Checks the code to make sure it compiles.

