# Lesson 05: Control Flow

## Learning Objectives
- [ ] Use `if`, `else if`, `else`
- [ ] Understand `if` as an expression (no ternary needed)
- [ ] Use `loop`, `while`, `for`
- [ ] Use `break` and `continue`
- [ ] Use loop labels for nested loops
- [ ] Return values from loops with `break value`

---

## Key Concepts

### `if` as an Expression
```rust
// No ternary operator — if IS the expression
let result = if x > 0 { "positive" } else { "non-positive" };
```

### Loop Types
| Type | Use Case |
|------|----------|
| `loop` | Infinite loop, exit with `break` |
| `while condition` | Loop while condition is true |
| `for item in collection` | Iterate over collections/ranges |

### Returning Values from Loops
```rust
let result = loop {
    if done {
        break 42;  // loop evaluates to 42
    }
};
```

### Loop Labels (Nested Loops)
```rust
'outer: loop {
    loop {
        break 'outer;  // breaks the outer loop
    }
}
```

### Range Syntax
```rust
for i in 0..5 { }      // 0, 1, 2, 3, 4
for i in 0..=5 { }     // 0, 1, 2, 3, 4, 5
for i in (0..5).rev() { }  // 4, 3, 2, 1, 0
```

---

## Notes
(Add your notes here as you learn)
