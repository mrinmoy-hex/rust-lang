# Variables and data types

This crate introduces Rust bindings and the basic scalar and compound types.

## Topics covered

- Bindings are immutable unless declared with `mut`.
- Shadowing uses `let` to create a new binding, and can change its type.
- Constants use `const`, require a type annotation, and cannot be mutable.
- Scalar examples: floating-point numbers, booleans, and Unicode characters.
- Tuples group values of different types; arrays contain a fixed number of values of one type.
- Use tuple positions (such as `.0`) or destructuring to access tuple values. Array indices start at `0`.

## Run

From this directory:

```sh
cargo run
```

The examples are in [`src/main.rs`](src/main.rs). Try changing a value or its type annotation and run the program again.
