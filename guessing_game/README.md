# Guessing game

This crate is a small interactive number-guessing game.

## How it works

1. A secret number is generated in the inclusive range `1..=100`.
2. The program repeatedly reads a line from standard input.
3. The input is trimmed and parsed as an unsigned integer; invalid input is ignored and the prompt repeats.
4. `cmp` and `match` report whether the guess is too small, too big, or correct. A correct guess ends the loop.

The random number generator is provided by the `rand` dependency in `Cargo.toml`.

## Run

From this directory:

```sh
cargo run
```

Enter guesses in the terminal until the game reports a win.
