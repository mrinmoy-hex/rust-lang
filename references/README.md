# References and borrowing

This crate demonstrates borrowing a `String` mutably so a function can modify it without taking ownership.

## Topics covered

- `&mut String` is a mutable reference; the function can append to the borrowed string.
- The caller retains ownership and can use the value after the function returns.
- Rust allows either one active mutable reference or any number of immutable references to a value at a time.
- A borrow's lifetime is limited to where it is used, which allows later access once the borrow ends.

## Run

From this directory:

```sh
cargo run
```

Try uncommenting the second mutable reference in `main` while the first is still in use; the borrow checker will reject overlapping mutable borrows.
