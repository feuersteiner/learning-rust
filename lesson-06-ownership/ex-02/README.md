# Exercise 02 - Ownership and Functions

## Goal
Practice moves and borrows across function boundaries.

## Step 1 - Starter Code
Apply the starter code in `src/main.rs`.

## Step 2 - Observe
Run `cargo check` and read each ownership error.

## Step 3 - Fix
Make the smallest changes needed to make it compile without deleting any `println!` lines.

Hints:
- When does passing to a function move ownership?
- How do you pass a borrow instead?
- Can you return ownership instead of cloning?

## Competitive Programming Exercise - Unique Words Frequency

**Input**:
- First line: `n` (number of words)
- Next `n` lines: words (ASCII lowercase)

**Output**:
- Print the number of unique words
- Then print the top 3 most frequent words (descending count; tie-break lexicographically ascending)
- If fewer than 3 unique words, print all

**Constraints**:
- `1 <= n <= 200000`
- word length `1..=30`

**Rules**:
- Read input as a single `String`, split by whitespace
- Use `Vec<String>` for words and `HashMap<String, usize>` for counts
- Avoid `clone()` unless necessary; prefer borrowing and `entry`

**Hints**:
- Parse `n` first, then iterate the rest
- `split_whitespace()` yields `&str` slices
- For sorting, collect into `Vec<(String, usize)>` and use `sort_by`
