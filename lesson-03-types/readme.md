# Lesson 03: Data Types

## Learning Objectives
- [ ] Understand scalar types: integers, floats, booleans, characters
- [ ] Understand compound types: tuples, arrays
- [ ] Know when Rust infers types vs when you must annotate
- [ ] Understand numeric overflow behavior (debug vs release)
- [ ] Know the difference between `String` and `&str`

---

## Exercises

### Exercise 1: The Overflow Detective ✅
**Problem:** Given an `i32` value, determine the smallest signed integer type (`i8`, `i16`, or `i32`) that can hold it.

```rust
smallest_type(100)   → "i8"
smallest_type(-200)  → "i16"
smallest_type(40000) → "i32"
```

**Learned:**
- Integer type limits with `i8::MAX`, `i16::MIN`, etc.
- Type casting with `as i32`
- `.abs()` for absolute value
- `if` as an expression (no `return` needed, no semicolon on value)
- Shadowing is block-scoped

### Exercise 2: Type Annotations
Practice scenarios where type inference works vs where you need annotations.
- Parse a string to a number (requires annotation!)
- Work with numeric literals

### Exercise 3: Tuples
Create and destructure tuples.
- Access elements by index
- Use pattern matching to unpack

### Exercise 4: Arrays
Work with fixed-size arrays.
- Create arrays with explicit types
- Iterate and access elements
- Understand the difference between arrays and vectors (preview)

### Exercise 5: Putting It Together
Build a small program that uses multiple types together.

---

## Key Concepts

### Scalar Types
| Category | Types |
|----------|-------|
| Integers | `i8`, `i16`, `i32`, `i64`, `i128`, `isize` |
| Unsigned | `u8`, `u16`, `u32`, `u64`, `u128`, `usize` |
| Floats   | `f32`, `f64` |
| Boolean  | `bool` |
| Char     | `char` (4 bytes, Unicode scalar) |

### Compound Types
| Type | Syntax | Notes |
|------|--------|-------|
| Tuple | `(i32, f64, char)` | Fixed size, mixed types |
| Array | `[i32; 5]` | Fixed size, same type |

---

## Notes
(Add your notes here as you learn)
