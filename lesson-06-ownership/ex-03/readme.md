# Exercise 03 - Unique Words Frequency

## Goal
Practice ownership with `String`, `&str`, and `HashMap` while processing input at scale.

## Starter

- File: `lesson-06-ownership/ex-03/src/main.rs`
- Copy the starter snippet from here (if applicable)

## Steps

1) Read all input into a single `String`, then split by whitespace.
2) Parse `n`, then consume exactly `n` words into a `HashMap<String, usize>`.
3) Print the unique count and the top 3 most frequent words.
4) Tie-break by lexicographic order ascending for equal counts.

## Hints

- `split_whitespace()` yields `&str` slices you can own with `to_string()`.
- `entry` helps you update counts without extra clones.
- Collect to `Vec<(String, usize)>` then `sort_by` with a custom comparator.

## Completion

- Works for small and large inputs without panics.
- Output format matches the rules exactly.
