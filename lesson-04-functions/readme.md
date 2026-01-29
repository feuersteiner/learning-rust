# Lesson 04: Functions

## Learning Objectives
- [x] Define functions with `fn`
- [x] Understand parameters and return types
- [x] Know the difference between statements and expressions
- [x] Use implicit returns (no semicolon)
- [x] Use explicit `return` for early exits

---

## Exercises

### Exercise 1: TBD

---

## Key Concepts

### Function Syntax
```rust
fn function_name(param: Type) -> ReturnType {
    // body
}
```

### Statements vs Expressions
| Type | Description | Example |
|------|-------------|---------|
| Statement | Performs action, returns nothing (`()`) | `let x = 5;` |
| Expression | Evaluates to a value | `5 + 3`, `{ x + 1 }` |

### Implicit vs Explicit Return
```rust
// Implicit: last expression without semicolon
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Explicit: use return keyword (often for early exit)
fn early_exit(x: i32) -> i32 {
    if x < 0 {
        return 0;
    }
    x * 2
}
```