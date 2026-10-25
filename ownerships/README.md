# Ownership

This crate demonstrates Rust's move and copy behavior, scope, and transferring ownership through function calls.

## Topics covered

- Simple integer values such as `i32` implement `Copy`, so assigning one to another creates a usable copy.
- `String` does not implement `Copy`; assigning it to another binding moves ownership.
- A function can take ownership of a value and return it to transfer ownership back to its caller.
- A value is dropped when its owner leaves scope.

## Run

From this directory:

```sh
cargo run
```

Try using `s1` after `let s2 = s1;` to see how the compiler prevents use after a move.
