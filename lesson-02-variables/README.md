# Lesson 02 - Variables & Mutability

## Concepts Learned

### Immutability by Default
- `let x = 5;` creates an immutable binding — reassignment fails at compile time
- Use `let mut x = 5;` to opt-in to mutability

### Shadowing
- Re-declaring with `let` creates a *new* variable that hides the old one
- Unlike `mut`, shadowing allows changing the **type**:
  ```rust
  let x = "42";       // &str
  let x: i32 = x.parse().unwrap();  // now i32
  ```
- Old value is dropped when shadowed

### `const` vs `let`
| | `let` | `const` |
|---|-------|---------|
| Type annotation | Optional (inferred) | Required |
| Value known | Runtime OK | Compile time only |
| Scope | Inside functions only | Module or function scope |
| Naming convention | snake_case | SCREAMING_SNAKE_CASE |

```rust
const MAX_POINTS: u32 = 100_000;  // OK
const INPUT: String = get_input(); // Error - not known at compile time
```

### `static` vs `const`
- `static` has a fixed memory address; `const` is inlined wherever used
- `static` can be mutable (with `unsafe`), `const` never

### `&str` vs `String` (Preview)
- `"hello"` is a `&str` — a string slice, immutable, baked into the binary
- `String` is heap-allocated, owned, growable
- More in Lesson 08 (Slices)

### Format Strings
- `println!("{a}")` is shorthand for `println!("{}", a)`
- `println!` is a **macro** (note the `!`) — enables compile-time format checking

## Key Insight

Rust defaults to immutability for safety. You opt-in to mutability explicitly.
