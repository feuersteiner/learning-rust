# Curriculum

Full concept checklist per lesson, distilled from `README.md`.

## Phase 1: Basics

### 00 - Console I/O & Paths

Directory: `lesson-00/`

Concepts:
- Console input (`std::io::stdin`, `read_line`)
- Console output (`print!`, `println!`, `flush`)
- Parsing and casting (`trim`, `parse::<T>()`)
- Paths and namespaces (`::`)
- Modules vs types (associated items)

### 01 - Hello World & Cargo

Directory: `lesson-01-hello-cargo/`

Concepts:
- Installing Rust with `rustup`
- Creating projects with `cargo new`
- Understanding `Cargo.toml` and `Cargo.lock`
- `cargo build`, `cargo run`, `cargo check`
- The `main()` function entry point

### 02 - Variables & Mutability

Directory: `lesson-02-variables/`

Concepts:
- Immutability by default (`let`)
- Mutable variables (`let mut`)
- Constants (`const`) vs variables
- Shadowing
- Type annotations

### 03 - Data Types

Directory: `lesson-03-types/`

Concepts:
- Scalar types: integers (`i32`, `u64`, etc.), floats (`f32`, `f64`), `bool`, `char`
- Compound types: tuples, arrays
- Type inference
- Numeric operations and overflow behavior

### 04 - Functions

Directory: `lesson-04-functions/`

Concepts:
- Function definitions with `fn`
- Parameters and return types
- Statements vs expressions
- Implicit returns (no semicolon)
- Early returns with `return`

### 05 - Control Flow

Directory: `lesson-05-control-flow/`

Concepts:
- `if`, `else if`, `else`
- Using `if` as an expression
- `loop`, `while`, `for`
- `break` and `continue`
- Loop labels for nested loops
- Returning values from loops

## Phase 2: Ownership & Memory

### 06 - Ownership

Directory: `lesson-06-ownership/`

Concepts:
- The three ownership rules
- Stack vs heap memory
- Move semantics
- `Copy` trait for simple types
- Scope and automatic memory cleanup (`Drop`)

### 07 - References & Borrowing

Directory: `lesson-07-references/` (planned)

Concepts:
- Immutable references (`&T`)
- Mutable references (`&mut T`)
- The borrowing rules
- Preventing data races at compile time
- Reference scope (Non-Lexical Lifetimes)

### 08 - Slices

Directory: `lesson-08-slices/` (planned)

Concepts:
- String slices (`&str`)
- Array slices (`&[T]`)
- The relationship between `String` and `&str`
- Slice as a "view" into data
- String literals are slices

## Phase 3: Structs & Enums

### 09 - Structs

Directory: `lesson-09-structs/` (planned)

Concepts:
- Defining and instantiating structs
- Field init shorthand
- Struct update syntax (`..`)
- Tuple structs
- Unit-like structs
- Methods with `impl` blocks
- Associated functions (`Self::new()`)

### 10 - Enums

Directory: `lesson-10-enums/` (planned)

Concepts:
- Defining enums
- Enums with data (variants)
- The `Option<T>` enum
- No null in Rust
- Enum methods

### 11 - Pattern Matching

Directory: `lesson-11-pattern-matching/` (planned)

Concepts:
- The `match` expression
- Exhaustive matching
- Catch-all with `_` and `other`
- `if let` for concise matching
- `while let` loops
- Destructuring in patterns

## Phase 4: Error Handling

### 12 - Recoverable Errors with Result

Directory: `lesson-12-result/` (planned)

Concepts:
- `Result<T, E>` enum
- `Ok` and `Err` variants
- `unwrap()` and `expect()`
- The `?` operator for propagation
- Combining `?` with `Option`

### 13 - Unrecoverable Errors with panic!

Directory: `lesson-13-panic/` (planned)

Concepts:
- When to use `panic!`
- Stack unwinding vs abort
- `RUST_BACKTRACE=1`
- Designing APIs: Result vs panic

## Phase 5: Collections & Iterators

### 14 - Vectors

Directory: `lesson-14-vectors/` (planned)

Concepts:
- Creating vectors (`Vec<T>`)
- Adding and removing elements
- Accessing elements (indexing vs `.get()`)
- Iterating over vectors
- Vectors and ownership

### 15 - Strings

Directory: `lesson-15-strings/` (planned)

Concepts:
- `String` vs `&str`
- Creating and updating strings
- UTF-8 encoding
- Why you can't index strings
- Iterating over characters and bytes
- String slicing (with care)

### 16 - HashMaps

Directory: `lesson-16-hashmaps/` (planned)

Concepts:
- Creating `HashMap<K, V>`
- Inserting, accessing, updating
- Entry API for conditional updates
- Ownership and HashMaps
- Custom types as keys

### 17 - Iterators

Directory: `lesson-17-iterators/` (planned)

Concepts:
- The `Iterator` trait
- `iter()`, `iter_mut()`, `into_iter()`
- Iterator adaptors: `map`, `filter`, `take`, `skip`
- Consuming adaptors: `collect`, `sum`, `fold`
- Chaining iterators
- Lazy evaluation

## Phase 6: Traits & Generics

### 18 - Generics

Directory: `lesson-18-generics/` (planned)

Concepts:
- Generic functions
- Generic structs and enums
- Generic method implementations
- Monomorphization (zero-cost abstractions)

### 19 - Traits

Directory: `lesson-19-traits/` (planned)

Concepts:
- Defining traits
- Implementing traits for types
- Default implementations
- Traits as parameters (`impl Trait`)
- Trait bounds
- The orphan rule
- Common traits: `Debug`, `Clone`, `Default`, `PartialEq`

### 20 - Trait Bounds & where Clauses

Directory: `lesson-20-trait-bounds/` (planned)

Concepts:
- Multiple trait bounds (`+`)
- `where` clauses for readability
- Conditional implementations
- Returning types that implement traits

## Phase 7: Modules & Crates

### 21 - Modules

Directory: `lesson-21-modules/` (planned)

Concepts:
- Module system overview
- `mod` keyword
- File structure (`mod.rs` vs `filename.rs`)
- `pub` for visibility
- `use` for bringing items into scope
- Re-exporting with `pub use`

### 22 - Crates & Dependencies

Directory: `lesson-22-crates/` (planned)

Concepts:
- Library vs binary crates
- crates.io ecosystem
- Adding dependencies in `Cargo.toml`
- Semantic versioning
- Features and optional dependencies
- Workspaces for multi-crate projects

## Phase 8: Concurrency

### 23 - Threads

Directory: `lesson-23-threads/` (planned)

Concepts:
- Spawning threads with `thread::spawn`
- `JoinHandle` and waiting for threads
- `move` closures for ownership transfer
- Thread safety guarantees

### 24 - Message Passing

Directory: `lesson-24-channels/` (planned)

Concepts:
- Channels (`mpsc`)
- `Sender` and `Receiver`
- Sending values between threads
- Multiple producers

### 25 - Shared State

Directory: `lesson-25-shared-state/` (planned)

Concepts:
- `Mutex<T>` for mutual exclusion
- `Arc<T>` for atomic reference counting
- Deadlock prevention
- `RwLock` for reader-writer locks
- `Send` and `Sync` traits

## Phase 9: Advanced Patterns

### 26 - Lifetimes

Directory: `lesson-26-lifetimes/` (planned)

Concepts:
- Why lifetimes exist
- Lifetime annotations (`'a`)
- Lifetime elision rules
- Lifetimes in structs
- `'static` lifetime
- Lifetime bounds on generics

### 27 - Closures

Directory: `lesson-27-closures/` (planned)

Concepts:
- Closure syntax
- Type inference for closures
- Capturing environment: `Fn`, `FnMut`, `FnOnce`
- `move` closures
- Closures as function parameters and return types

### 28 - Smart Pointers

Directory: `lesson-28-smart-pointers/` (planned)

Concepts:
- `Box<T>` for heap allocation
- `Rc<T>` for reference counting
- `RefCell<T>` for interior mutability
- `Weak<T>` to prevent reference cycles
- `Deref` and `Drop` traits

### 29 - Unsafe Rust

Directory: `lesson-29-unsafe/` (planned)

Concepts:
- When and why to use `unsafe`
- Raw pointers
- Calling unsafe functions
- Safe abstractions over unsafe code
- FFI basics

### 30 - Macros

Directory: `lesson-30-macros/` (planned)

Concepts:
- Declarative macros (`macro_rules!`)
- Macro patterns and repetition
- Procedural macros overview
- Common macros: `vec!`, `println!`, `derive`

## Phase 10: Capstone

### 31 - Capstone: Build a CLI Tool

Directory: `lesson-31-capstone-cli/` (planned)

Goal components:
- Argument parsing (with `clap`)
- File I/O
- Error handling with custom error types
- Structs and enums for data modeling
- Iterators for data processing
- Tests

Suggested project ideas:
- A grep clone
- A todo list manager
- A Markdown to HTML converter
- A file organizer
