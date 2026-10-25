# Functions and expressions

This crate demonstrates function declarations, parameters, return values, and blocks that produce values.

## Topics covered

- Function parameters include explicit types.
- A return type appears after `->`; a function can return its final expression without a `return` keyword.
- A trailing semicolon turns an expression into a statement, so it does not return that value.
- A block's final expression can be assigned to a variable or passed as a function argument.
- `print_labeled_measurement` demonstrates multiple parameters with different types.

## Run

From this directory:

```sh
cargo run
```

Try adding a semicolon to the final expression in a value-producing block and observe the compiler diagnostic.
