# Exercise 00 - Syntax Reference (I/O)

## Goal

Quickly reference the syntax for reading input, parsing, and printing output in Rust.

## Starter

- File: lesson-00/ex-00-syntax/README.md
- Copy snippets into your scratch file as needed

## Snippets

### Read a line (string)

```rust
use std::io::{self, Write};

let mut input = String::new();
print!("Name: ");
io::stdout().flush().unwrap();
io::stdin().read_line(&mut input).unwrap();
let name = input.trim().to_string();
```

### Parse to number

```rust
let raw = "42";
let n: i32 = raw.trim().parse().unwrap();
```

### Reuse the buffer (clear)

```rust
input.clear();
io::stdin().read_line(&mut input).unwrap();
```

### Print with formatting

```rust
let age = 30u32;
println!("Next year: {}", age + 1);
println!("Rounded: {:.2}", 3.14159);
```

## Completion

- You can explain what each snippet does and when to use it.
