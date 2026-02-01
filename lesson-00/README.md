# Lesson 00: Console Input/Output Basics

Goal: learn Rust equivalents to C# `Console.ReadLine()` and `Console.WriteLine()` and practice parsing and casting user input.

## What to Build

Create a tiny CLI program that:

1) Prompts for a name (string) and echoes it back.
2) Prompts for an age (u32) and prints age next year.
3) Prompts for a decimal number (f64) and prints it rounded to 2 decimals.

## Key Rust APIs (Look These Up)

- `std::io::stdin()`
- `read_line(&mut String)`
- `String::trim()`
- `str::parse::<T>()`
- `println!()` and `print!()`
- `std::io::Write::flush()`

## What Does `::` Mean?

In Rust, `::` is the path separator used to reach items inside modules, types, or namespaces.

Examples:

- `std::io` means the `io` module inside the `std` module.
- `std::io::stdin()` means the `stdin` function inside `std::io`.
- `String::new()` means the `new` associated function on the `String` type.

Think of it as:

- C#: `System.Console.WriteLine` (Rust: `std::io::...` + `println!` macro)
- TypeScript: `namespace.module.function`

## Modules, Types, and Namespaces (Quick Map)

- Modules: organize code and form paths. Example: `std::io` means module `io` inside `std`.
- Types: structs/enums define their own namespaces for associated functions/constants. Example: `String::new()` or `u32::MAX`.
- Namespaces: a general idea in Rust that “items live in a path.” Paths can point to modules, types, traits, functions, constants, etc.

Common patterns:

- `use std::io;` then `io::stdin()`
- `use std::io::Write;` then `std::io::stdout().flush()`
- `use std::fmt::Debug;` then `T: Debug` in a generic bound

## Hints

- `read_line` appends to the buffer. Clear the `String` between reads.
- `trim()` removes trailing newlines so `parse()` works.
- Use `parse::<u32>()?` or `parse::<f64>()?` depending on the input.
- `print!()` does not auto-flush; call `stdout().flush()?` after prompts.

## Suggested Steps

1) Create a new binary crate inside `lesson-00/` (or reuse `cargo new lesson-00` if you prefer).
2) Implement input/output for a string first.
3) Add numeric parsing and simple math.
4) Handle invalid input gracefully (repeat prompt or show an error).

## Exercises

1) Add a prompt for a whole number, parse to `i32`, and print its square.
2) Ask for two numbers, parse to `f64`, and print their sum and average.
3) Ask for a boolean (`true`/`false`) and branch on it.

## Documentation

- Rust Book: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
- std::io docs: https://doc.rust-lang.org/std/io/
- str::parse docs: https://doc.rust-lang.org/std/primitive.str.html#method.parse
