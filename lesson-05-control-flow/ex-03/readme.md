# Exercise 3: The Treasure Hunt

You're searching a 2D grid for treasure. The grid is represented as an array of arrays where each cell contains:
- `Some(value)` — treasure worth `value` points
- `None` — empty cell

## Requirements

```rust
fn hunt_treasure(grid: [[Option<i32>; 4]; 4]) -> (i32, usize, usize)
```

Returns: `(total_value, last_row, last_col)` — total treasure collected and position of last treasure found.

**Rules:**
1. Search the grid **row by row, but columns in reverse** (use `.rev()`)
2. Use `while let` to extract treasure values from `Some`
3. Use `match` to handle what you find in each cell
4. Use `for` with `.enumerate()` to track row/column indices
5. Use a **labeled loop**: if you find a cell with value `< 0` (a trap!), immediately stop the entire hunt using `break 'hunt`
6. Skip cells with value `0` using `continue` (worthless junk)

## Example Grid

```rust
let grid = [
    [Some(5),  None,      Some(0),  Some(10)],
    [Some(3),  Some(-1),  Some(7),  None    ],  // -1 is a trap!
    [Some(20), Some(15),  Some(8),  Some(2) ],
    [None,     Some(100), Some(50), Some(25)],
];
```

Searching row 0 right-to-left: 10, 0 (skip), None, 5 → collect 15
Searching row 1 right-to-left: None, 7, -1 (TRAP!) → stop immediately

**Result:** `(22, 1, 2)` — collected 22 points, last treasure at row 1, col 2 (the 7)

## Test Cases

```rust
assert_eq!(hunt_treasure(grid), (22, 1, 2));
```

## Concepts Practiced

- `.rev()` for reverse iteration
- `.enumerate()` for indices
- `match` for pattern matching cells
- `while let` (use it creatively)
- Labeled loop with `break 'label`
- `continue` to skip
