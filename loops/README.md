# Loops

This crate demonstrates Rust's three loop forms and how to control iteration.

## Topics covered

- `loop` repeats indefinitely until `break`; a `break` expression can return a value.
- Labeled loops allow `break 'label` to exit an outer loop from an inner loop.
- `while` repeats while a condition is true.
- `for` iterates over an array or a range without manual index management.
- `..` creates a range with an exclusive end; `..=` includes the end.
- Calling `.rev()` iterates a range in reverse.

## Run

From this directory:

```sh
cargo run
```

The examples are in [`src/main.rs`](src/main.rs). Adjust the ranges and conditions to see how the output changes.
