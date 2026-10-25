# Authentication service module practice

This small crate explores how Rust modules can divide an application into authentication utilities, data models, database operations, and a binary entry point.

## Code map

- `src/main.rs` constructs a credentials value and calls the library's authentication function.
- `src/lib.rs` exposes `authenticate` and coordinates a database connection with the login module.
- `src/auth_utils.rs` contains the login function; `src/auth_utils/models.rs` defines `Credentials`.
- `src/database.rs` contains the connection status and database-operation placeholders.
- `src/util.rs` is a separate module example.

## Important limitation

This is a module-organization exercise, not production authentication. The database functions are placeholders, the connection always reports success, and the login function does not validate credentials or return an authentication result. Do not use this crate to protect real accounts or store real credentials.

## Run

From this directory:

```sh
cargo run
```

Follow calls from `main` into `authenticate`, `connect_to_database`, and `login` to see how module paths connect the files.
