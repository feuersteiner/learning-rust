# Exercise 01 - Tiny CLI

## Goal

Practice basic console I/O: read strings, parse numbers, and format output.

## Starter

- File: lesson-00/ex-01-tiny-cli/src/main.rs
- Start from the default `fn main()` and build up step by step

## Steps

1) Prompt for a name, read a line, trim it, and echo it back.
2) Prompt for an age (u32), parse it, and print the age next year.
3) Prompt for a decimal (f64), parse it, and print it rounded to 2 decimals.

## Hints

- `read_line` appends, so `clear()` the buffer between inputs.
- `print!()` needs `stdout().flush()` to show the prompt before input.
- `trim()` removes the trailing newline so `parse()` works.
- Format with `println!("{:.2}", value)` for 2 decimals.

## Completion

- Running `cargo run` prints each prompt, accepts input, and shows the three results.
