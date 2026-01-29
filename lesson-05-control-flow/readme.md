# Lesson 05: Control Flow

## Learning Objectives
- [ ] Use `if`, `else if`, `else`
- [ ] Understand `if` as an expression (no ternary needed)
- [ ] Use `match` for pattern matching
- [ ] Use `loop`, `while`, `for`
- [ ] Use `break` and `continue`
- [ ] Use loop labels for nested loops
- [ ] Return values from loops with `break value`

---

## Conditionals

### if / else if / else
```rust
if condition {
    // ...
} else if other_condition {
    // ...
} else {
    // ...
}
```

### if as expression
```rust
let result = if x > 0 { "positive" } else { "negative" };
```

### match
```rust
match value {
    1 => println!("one"),
    2 | 3 => println!("two or three"),
    4..=10 => println!("four to ten"),
    _ => println!("something else"),  // catch-all
}
```

### match as expression
```rust
let result = match x {
    0 => "zero",
    1 => "one",
    _ => "many",
};
```

### match with destructuring
```rust
let point = (3, 4);
match point {
    (0, 0) => println!("origin"),
    (x, 0) => println!("on x-axis at {x}"),
    (0, y) => println!("on y-axis at {y}"),
    (x, y) => println!("at ({x}, {y})"),
}
```

---

## Loops

### loop (infinite)
```rust
loop {
    if done {
        break;
    }
}
```

### loop with return value
```rust
let result = loop {
    if found {
        break 42;
    }
};
```

### while
```rust
while condition {
    // ...
}
```

### while let (pattern matching)
```rust
while let Some(value) = optional {
    // runs while pattern matches
}
```

### for (range)
```rust
for i in 0..5 { }       // 0, 1, 2, 3, 4
for i in 0..=5 { }      // 0, 1, 2, 3, 4, 5
for i in (0..5).rev() { }  // 4, 3, 2, 1, 0
```

### for (collection)
```rust
let arr = [10, 20, 30];
for item in arr {
    println!("{item}");
}
```

### for with index
```rust
for (index, value) in arr.iter().enumerate() {
    println!("{index}: {value}");
}
```

---

## Loop Control

### break
```rust
break;        // exit loop
break 42;     // exit loop with value
break 'label; // exit labeled loop
```

### continue
```rust
continue;         // skip to next iteration
continue 'label;  // skip in labeled loop
```

### loop labels
```rust
'outer: loop {
    'inner: loop {
        break 'outer;     // breaks outer loop
        continue 'outer;  // continues outer loop
    }
}
```

---

## No do-while in Rust

Rust doesn't have `do-while`. The equivalent:

```rust
loop {
    // body runs at least once
    if !condition {
        break;
    }
}
```

---

## Notes
(Add your notes here as you learn)
