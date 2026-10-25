# Slices

This crate introduces borrowed views into strings and arrays. A slice refers to existing data rather than copying it.

## Topics covered

- A range such as `0..5` includes the start and excludes the end.
- `&str` is a borrowed string slice. It works with both string literals and slices borrowed from `String`.
- `first_word` returns a slice borrowed from its input, so it can be used as text without returning an index.
- Rust tracks the borrow: while an immutable slice is in use, conflicting mutation of its source is not allowed.
- Array slices borrow a contiguous range of elements.
- Rust strings are UTF-8; byte offsets are not always character boundaries. Prefer string operations such as `split_whitespace` when searching text.

## Run

From this directory:

```sh
cargo run
```

The examples are in [`src/main.rs`](src/main.rs). Change the slice ranges or test `first_word` with different text.
