# Module practice

This crate demonstrates how Rust modules map to files and nested directories.

## Code map

- `src/main.rs` declares the `garden` module and uses a public item from its child module.
- `src/garden.rs` declares the `vegetables` child module.
- `src/garden/vegetables.rs` defines the public `Asparagus` enum.

The `mod` declarations establish the module tree, while `pub` makes the child module and enum available to their parent.

## Run

From this directory:

```sh
cargo run
```

Try changing the visibility of a module or item and observe how that affects access from `main`.
