# Concise control flow

This crate compares `match` with `if let` when working with enums and `Option`.

## Topics covered

- `match` can handle each variant explicitly and requires exhaustive coverage.
- `if let` is concise when one pattern is interesting; it does not replace the other cases with exhaustive branches.
- The `Quarter` variant carries a `UsState` value, which can be bound when that variant matches.
- `Option<T>` represents an optional value as either `Some(value)` or `None`.

## Run

From this directory:

```sh
cargo run
```

Try replacing the `if let` example with a `match` and explicitly handling the remaining variants.
