# Exercise 01 - Ownership Moves vs Copies

## Goal
Practice identifying moves vs copies and fixing ownership errors with minimal changes.

## Step 1 - Starter Code
Replace `src/main.rs` with the following starter code:

```rust
fn main() {
    let name = String::from("Ada");
    let greeting = format!("Hi, {name}");

    // EX01-A: This move should break later use of `name`.
    let name_moved = name;
    println!("name_moved = {name_moved}");
    println!("name = {name}");

    // EX01-B: Copy types should still work after assignment.
    let x = 42;
    let y = x;
    println!("x = {x}, y = {y}");

    // EX01-C: What happens with `greeting` here?
    let g2 = greeting;
    println!("g2 = {g2}");
    println!("greeting = {greeting}");
}
```

## Step 2 - Observe
Run `cargo check` and read the ownership errors.

## Step 3 - Fix
Make the smallest changes needed to make it compile without deleting any `println!` lines.

Hints:
- Which assignments *move* ownership?
- Which types are `Copy`?
- Can you keep data by borrowing instead of moving?
- If you must duplicate, consider `clone()` and explain why.

When you’re done, explain in 1-2 sentences why each fix works.

## Extra Example - Ending a Mutable Borrow (NLL)
This shows how a mutable borrow is “released” after its last use.

```rust
let mut s = String::from("hi");
let r = &mut s;
r.push_str("!");
// println!("{s}"); // ERROR: cannot use `s` while it is borrowed mutably
println!("{r}");

// After the last use of `r`, we can use `s` again.
println!("{s}");
```
