# Enums

This crate introduces enums as types whose values can be one of several variants.

## Topics covered

- `IpAddrKind` has separate `V4` and `V6` variants.
- Variants can carry data: `V4` stores four bytes, while `V6` stores a string.
- A function can accept the enum type and handle either kind of address through one interface.
- `Debug` formatting makes the variant and its data visible in the example output.

## Run

From this directory:

```sh
cargo run
```

Try constructing another address value and passing it to `route`.
