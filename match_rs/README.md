# Matching enum values

This crate uses `match` to handle enum variants and optional values.

## Topics covered

- `value_in_cents` maps each `Coin` variant to its value.
- The `Quarter` variant carries a state, and a specific state can have its own match arm.
- `add` handles both `Some(number)` and `None`, illustrating how `Option<T>` makes absence explicit.
- The dice example uses `_` as a catch-all pattern for values not listed in earlier arms.
- Match arms can contain blocks, print a message, and return a final expression.

## Run

From this directory:

```sh
cargo run
```

Add a new enum variant and let the compiler help identify every `match` that needs another arm.
