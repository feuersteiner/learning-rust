# 🦀 Rust Learning Curriculum

A structured, hands-on curriculum to learn Rust from scratch and become autonomous in building real projects.

---

## My Background

- **C#**: Grew up with it - familiar with strong typing, structs, enums, generics
- **C**: College - understand memory concepts (stack vs heap, pointers)
- **TypeScript**: Current daily driver - appreciate type safety and good tooling

---

## 📚 How This Learning Loop Works

1. **Read the concept** - Each section introduces a Rust concept with key ideas to understand
2. **Navigate to the exercise directory** - Each concept has a dedicated `XX-concept-name/` folder
3. **Ask the AI agent for an exercise** - Request a practical exercise for that concept
4. **Implement the solution** - Write the code yourself, struggle, learn
5. **Get feedback** - Ask the AI to review your solution and explain improvements
6. **Move to the next concept** - Once comfortable, proceed to the next topic

> 💡 **Tip**: Don't rush. Rust has a steep learning curve, but understanding each concept deeply will pay off.

---

## 🗺️ Curriculum Overview

| Phase | Topics | Goal |
|-------|--------|------|
| **Phase 1** | Basics | Understand Rust syntax and core concepts |
| **Phase 2** | Ownership & Memory | Master Rust's unique memory model |
| **Phase 3** | Structs & Enums | Build custom data types |
| **Phase 4** | Error Handling | Write robust, recoverable code |
| **Phase 5** | Collections & Iterators | Work with data efficiently |
| **Phase 6** | Traits & Generics | Write reusable, polymorphic code |
| **Phase 7** | Modules & Crates | Organize code and use dependencies |
| **Phase 8** | Concurrency | Write safe parallel code |
| **Phase 9** | Advanced Patterns | Lifetimes, smart pointers, macros |
| **Phase 10** | Capstone Project | Build something real |

---

## 📖 Phase 1: The Basics

### 01 - Hello World & Cargo
**Directory**: `01-hello-cargo/`

**Concepts**:
- Installing Rust with `rustup`
- Creating projects with `cargo new`
- Understanding `Cargo.toml` and `Cargo.lock`
- `cargo build`, `cargo run`, `cargo check`
- The `main()` function entry point

**Key Commands**:
```bash
cargo new project_name
cargo run
cargo build --release
```

---

### 02 - Variables & Mutability
**Directory**: `02-variables/`

**Concepts**:
- Immutability by default (`let`)
- Mutable variables (`let mut`)
- Constants (`const`) vs variables
- Shadowing
- Type annotations

**Key Insight**: Rust defaults to immutability for safety. You opt-in to mutability explicitly.

---

### 03 - Data Types
**Directory**: `03-data-types/`

**Concepts**:
- Scalar types: integers (`i32`, `u64`, etc.), floats (`f32`, `f64`), `bool`, `char`
- Compound types: tuples, arrays
- Type inference
- Numeric operations and overflow behavior

**Key Insight**: Rust is statically typed—every value has a known type at compile time.

---

### 04 - Functions
**Directory**: `04-functions/`

**Concepts**:
- Function definitions with `fn`
- Parameters and return types
- Statements vs expressions
- Implicit returns (no semicolon)
- Early returns with `return`

**Key Insight**: The last expression in a function (without `;`) is the return value.

---

### 05 - Control Flow
**Directory**: `05-control-flow/`

**Concepts**:
- `if`, `else if`, `else`
- Using `if` as an expression
- `loop`, `while`, `for`
- `break` and `continue`
- Loop labels for nested loops
- Returning values from loops

---

## 📖 Phase 2: Ownership & Memory

### 06 - Ownership
**Directory**: `06-ownership/`

**Concepts**:
- The three ownership rules
- Stack vs heap memory
- Move semantics
- `Copy` trait for simple types
- Scope and automatic memory cleanup (`Drop`)

**The Three Rules**:
1. Each value has an owner
2. Only one owner at a time
3. Value is dropped when owner goes out of scope

**Key Insight**: This is Rust's superpower—memory safety without garbage collection.

---

### 07 - References & Borrowing
**Directory**: `07-references/`

**Concepts**:
- Immutable references (`&T`)
- Mutable references (`&mut T`)
- The borrowing rules
- Preventing data races at compile time
- Reference scope (Non-Lexical Lifetimes)

**The Borrowing Rules**:
- Many immutable references OR one mutable reference
- References must always be valid

---

### 08 - Slices
**Directory**: `08-slices/`

**Concepts**:
- String slices (`&str`)
- Array slices (`&[T]`)
- The relationship between `String` and `&str`
- Slice as a "view" into data
- String literals are slices

---

## 📖 Phase 3: Structs & Enums

### 09 - Structs
**Directory**: `09-structs/`

**Concepts**:
- Defining and instantiating structs
- Field init shorthand
- Struct update syntax (`..`)
- Tuple structs
- Unit-like structs
- Methods with `impl` blocks
- Associated functions (`Self::new()`)

---

### 10 - Enums
**Directory**: `10-enums/`

**Concepts**:
- Defining enums
- Enums with data (variants)
- The `Option<T>` enum
- No null in Rust!
- Enum methods

---

### 11 - Pattern Matching
**Directory**: `11-pattern-matching/`

**Concepts**:
- The `match` expression
- Exhaustive matching
- Catch-all with `_` and `other`
- `if let` for concise matching
- `while let` loops
- Destructuring in patterns

---

## 📖 Phase 4: Error Handling

### 12 - Recoverable Errors with Result
**Directory**: `12-result/`

**Concepts**:
- `Result<T, E>` enum
- `Ok` and `Err` variants
- `unwrap()` and `expect()`
- The `?` operator for propagation
- Combining `?` with `Option`

---

### 13 - Unrecoverable Errors with panic!
**Directory**: `13-panic/`

**Concepts**:
- When to use `panic!`
- Stack unwinding vs abort
- `RUST_BACKTRACE=1`
- Designing APIs: Result vs panic

**Key Insight**: Use `Result` for expected failures, `panic!` for bugs.

---

## 📖 Phase 5: Collections & Iterators

### 14 - Vectors
**Directory**: `14-vectors/`

**Concepts**:
- Creating vectors (`Vec<T>`)
- Adding and removing elements
- Accessing elements (indexing vs `.get()`)
- Iterating over vectors
- Vectors and ownership

---

### 15 - Strings
**Directory**: `15-strings/`

**Concepts**:
- `String` vs `&str`
- Creating and updating strings
- UTF-8 encoding
- Why you can't index strings
- Iterating over characters and bytes
- String slicing (with care!)

---

### 16 - HashMaps
**Directory**: `16-hashmaps/`

**Concepts**:
- Creating `HashMap<K, V>`
- Inserting, accessing, updating
- Entry API for conditional updates
- Ownership and HashMaps
- Custom types as keys

---

### 17 - Iterators
**Directory**: `17-iterators/`

**Concepts**:
- The `Iterator` trait
- `iter()`, `iter_mut()`, `into_iter()`
- Iterator adaptors: `map`, `filter`, `take`, `skip`
- Consuming adaptors: `collect`, `sum`, `fold`
- Chaining iterators
- Lazy evaluation

---

## 📖 Phase 6: Traits & Generics

### 18 - Generics
**Directory**: `18-generics/`

**Concepts**:
- Generic functions
- Generic structs and enums
- Generic method implementations
- Monomorphization (zero-cost abstractions)

---

### 19 - Traits
**Directory**: `19-traits/`

**Concepts**:
- Defining traits
- Implementing traits for types
- Default implementations
- Traits as parameters (`impl Trait`)
- Trait bounds
- The orphan rule
- Common traits: `Debug`, `Clone`, `Default`, `PartialEq`

---

### 20 - Trait Bounds & where Clauses
**Directory**: `20-trait-bounds/`

**Concepts**:
- Multiple trait bounds (`+`)
- `where` clauses for readability
- Conditional implementations
- Returning types that implement traits

---

## 📖 Phase 7: Modules & Crates

### 21 - Modules
**Directory**: `21-modules/`

**Concepts**:
- Module system overview
- `mod` keyword
- File structure (`mod.rs` vs `filename.rs`)
- `pub` for visibility
- `use` for bringing items into scope
- Re-exporting with `pub use`

---

### 22 - Crates & Dependencies
**Directory**: `22-crates/`

**Concepts**:
- Library vs binary crates
- crates.io ecosystem
- Adding dependencies in `Cargo.toml`
- Semantic versioning
- Features and optional dependencies
- Workspaces for multi-crate projects

---

## 📖 Phase 8: Concurrency

### 23 - Threads
**Directory**: `23-threads/`

**Concepts**:
- Spawning threads with `thread::spawn`
- `JoinHandle` and waiting for threads
- `move` closures for ownership transfer
- Thread safety guarantees

---

### 24 - Message Passing
**Directory**: `24-channels/`

**Concepts**:
- Channels (`mpsc`)
- `Sender` and `Receiver`
- Sending values between threads
- Multiple producers

---

### 25 - Shared State
**Directory**: `25-shared-state/`

**Concepts**:
- `Mutex<T>` for mutual exclusion
- `Arc<T>` for atomic reference counting
- Deadlock prevention
- `RwLock` for reader-writer locks
- `Send` and `Sync` traits

---

## 📖 Phase 9: Advanced Patterns

### 26 - Lifetimes
**Directory**: `26-lifetimes/`

**Concepts**:
- Why lifetimes exist
- Lifetime annotations (`'a`)
- Lifetime elision rules
- Lifetimes in structs
- `'static` lifetime
- Lifetime bounds on generics

---

### 27 - Closures
**Directory**: `27-closures/`

**Concepts**:
- Closure syntax
- Type inference for closures
- Capturing environment: `Fn`, `FnMut`, `FnOnce`
- `move` closures
- Closures as function parameters and return types

---

### 28 - Smart Pointers
**Directory**: `28-smart-pointers/`

**Concepts**:
- `Box<T>` for heap allocation
- `Rc<T>` for reference counting
- `RefCell<T>` for interior mutability
- `Weak<T>` to prevent reference cycles
- `Deref` and `Drop` traits

---

### 29 - Unsafe Rust
**Directory**: `29-unsafe/`

**Concepts**:
- When and why to use `unsafe`
- Raw pointers
- Calling unsafe functions
- Safe abstractions over unsafe code
- FFI basics

---

### 30 - Macros
**Directory**: `30-macros/`

**Concepts**:
- Declarative macros (`macro_rules!`)
- Macro patterns and repetition
- Procedural macros overview
- Common macros: `vec!`, `println!`, `derive`

---

## 📖 Phase 10: Capstone Project

### 31 - Capstone: Build a CLI Tool
**Directory**: `31-capstone-cli/`

**Goal**: Build a complete command-line application that combines:
- Argument parsing (with `clap`)
- File I/O
- Error handling with custom error types
- Structs and enums for data modeling
- Iterators for data processing
- Tests

**Suggested Projects**:
- A grep clone
- A todo list manager
- A Markdown to HTML converter
- A file organizer

---

## 🛠️ Recommended Tools

| Tool | Purpose |
|------|---------|
| `rustfmt` | Auto-format your code |
| `clippy` | Linting and suggestions |
| `rust-analyzer` | IDE support (VS Code) |
| `cargo doc` | Generate documentation |
| `cargo test` | Run tests |
| `cargo bench` | Benchmarking |

---

## 📚 Additional Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/) - The official book
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn through examples
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises
- [Exercism Rust Track](https://exercism.org/tracks/rust) - Mentored exercises
- [This Week in Rust](https://this-week-in-rust.org/) - Stay updated

---

## ✅ Progress Tracker

Use this to track your progress:

- [x] 01 - Hello World & Cargo
- [x] 02 - Variables & Mutability
- [x] 03 - Data Types
- [x] 04 - Functions
- [-] 05 - Control Flow
- [ ] 06 - Ownership
- [ ] 07 - References & Borrowing
- [ ] 08 - Slices
- [ ] 09 - Structs
- [ ] 10 - Enums
- [ ] 11 - Pattern Matching
- [ ] 12 - Recoverable Errors with Result
- [ ] 13 - Unrecoverable Errors with panic!
- [ ] 14 - Vectors
- [ ] 15 - Strings
- [ ] 16 - HashMaps
- [ ] 17 - Iterators
- [ ] 18 - Generics
- [ ] 19 - Traits
- [ ] 20 - Trait Bounds & where Clauses
- [ ] 21 - Modules
- [ ] 22 - Crates & Dependencies
- [ ] 23 - Threads
- [ ] 24 - Message Passing
- [ ] 25 - Shared State
- [ ] 26 - Lifetimes
- [ ] 27 - Closures
- [ ] 28 - Smart Pointers
- [ ] 29 - Unsafe Rust
- [ ] 30 - Macros
- [ ] 31 - Capstone: Build a CLI Tool

---

## 🎯 After Completing This Curriculum

You will be able to:
- ✅ Write idiomatic, safe Rust code
- ✅ Understand and work with the ownership system
- ✅ Handle errors gracefully
- ✅ Use the standard library effectively
- ✅ Structure projects with modules and crates
- ✅ Write concurrent programs safely
- ✅ Read and understand most Rust codebases
- ✅ Start building your own projects autonomously

**Next Steps**:
- Explore async Rust with `tokio` or `async-std`
- Build a web API with `axum` or `actix-web`
- Try WebAssembly with Rust
- Contribute to open source Rust projects

---

*Happy learning! 🦀*
