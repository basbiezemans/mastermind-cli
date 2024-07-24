[![Build binary release](https://github.com/basbiezemans/mastermind-cli/actions/workflows/release.yml/badge.svg)](https://github.com/basbiezemans/mastermind-cli/actions/workflows/release.yml)
[![Clippy](https://github.com/basbiezemans/mastermind-cli/actions/workflows/lint.yml/badge.svg)](https://github.com/basbiezemans/mastermind-cli/actions/workflows/lint.yml)
[![Tests](https://github.com/basbiezemans/mastermind-cli/actions/workflows/tests.yml/badge.svg)](https://github.com/basbiezemans/mastermind-cli/actions/workflows/tests.yml)
[![Rustfmt](https://github.com/basbiezemans/mastermind-cli/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/basbiezemans/mastermind-cli/actions/workflows/rustfmt.yml)

# Mastermind Game
Mastermind code-breaking game as a CLI application

[Mastermind (board game), Wikipedia](https://en.wikipedia.org/wiki/Mastermind_(board_game))

## Play the game
The following command runs the program in quiet mode.
```bash
cargo run --quiet
```

The game has a minimal user interface.
```
+------------------------------------+
| Mastermind, the code-breaking game |
+------------------------------------+
You have 10 turns to guess the code. Good luck!

You have 10 turns left.
Guess:
```

As the codebreaker, you try to guess a 4-digit secret code within ten turns. After each guess, the program (codemaker) provides a hint of solid ● and empty ○ dots. A ● for each correct digit, and a ○ for each digit that occurs in the secret code but in the wrong position.

## Build, test and run
This `make` command will format, lint, and test the code before it runs the program.
```bash
make all
```

## Binary
You can find the binary in the `target` folder and run it with the following command:
```bash
./target/debug/mastermind-cli
```

## Options
```
-t, --turns <TURNS>  [default: 10]
-h, --help           Print help
-V, --version        Print version
```
