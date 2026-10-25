# Standalone practice snippets

This directory contains small experiments in Rust structs, methods, enums, ownership, and borrowing, plus `check_words.py`.

## Rust examples

- `enums_03.rs` — enum variants carrying data and `Option` values.
- `enums_with_struct.rs` — an enum stored in a struct, an associated constructor, and a function taking the struct.
- `method_syntax.rs`, `rect_refactor.rs`, and `refactor_rect_2.rs` — methods, associated functions, rectangle calculations, and a width check.
- `rect_test.rs` — `dbg!` output and `Debug` formatting.
- `struct_test.rs` — mutable struct fields and struct update syntax.
- `str_len.rs` — moving a `String` into a function and returning ownership with its length.
- `test1.rs`, `test3.rs`, and `test4.rs` — boolean arguments and borrowing vector elements.
- `struct_test_2.rs` and `test2.rs` — incomplete exercises that refer to declarations or variables not yet provided.

These are independent scratch files rather than a Cargo package. Compile a self-contained Rust example from this directory with:

```sh
rustc method_syntax.rs -o /tmp/method_syntax-example
/tmp/method_syntax-example
```

Choose a different file name to run another example. Some files are intentionally incomplete and will need additional definitions before they compile.

## Python example

`check_words.py` demonstrates passing a list into a small class and modifying that list through another instance. Run it with:

```sh
python3 check_words.py
```
