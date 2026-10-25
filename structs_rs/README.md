# Structs

This crate shows how to define and create custom data types with named or positional fields.

## Topics covered

- A named-field struct groups related values, and each instance must initialize every field.
- Struct literal field order does not need to match the definition.
- Field init shorthand uses a local variable when its name matches a field.
- A builder function can centralize default field values.
- Struct update syntax (`..other_instance`) fills remaining fields from another value. Fields that are not `Copy` may be moved.
- Tuple structs provide a distinct type with positional fields.

## Run

From this directory:

```sh
cargo run
```

The examples are in [`src/main.rs`](src/main.rs). Try adding a field to `User` and updating every place that constructs one.
