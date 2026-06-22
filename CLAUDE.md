# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Purpose

Advent of Code 2025 solutions written in Rust. These are a vehicle for learning idiomatic Rust — Claude's role is to give feedback on idiomatic style, not to solve the puzzles. Prefer suggesting cleaner/more idiomatic Rust over introducing new crates or clever abstractions.

## Repository layout

Each day is its own independent Cargo binary crate in a numbered directory (`01-secret-entrance`, `02-gift-shop`, `03-lobby`, `04-printing-department`, ...). There is no workspace — every command below must be run from inside the relevant day's directory.

Each puzzle text is captured in that day's `README.md` (Problem 1 / Problem 2). The root `README.md` describes the overall project.

## Commands

Run from within a day directory (e.g. `cd 03-lobby`):

```sh
cat resources/input.txt | cargo run          # run against the real puzzle input
cat resources/sample-input.txt | cargo run   # run against the small example from the README
cargo test                                    # run that day's unit tests
cargo test test_get_joltage_for_two_cells     # run a single test by name
cargo clippy                                  # lint (clippy feedback is acted on — see git history)
```

The program reads its input from **stdin**, so it is always piped from a `resources/*.txt` file rather than taking a path argument.

## Solution architecture (consistent across days)

- `src/main.rs` is thin: it reads stdin line by line (`io::stdin().lock().lines()`), trims each line, delegates the real work to domain modules declared with `mod ...;`, and prints results. Part 1 and Part 2 answers are typically computed in the same pass and printed on separate lines (or space-separated).
- Domain logic lives in small, single-purpose modules (e.g. `battery.rs`, `space_calculator.rs`, `series_parser.rs` + `simple_validator.rs` + `advanced_validator.rs`). Parsing functions turn a line into a data structure; pure functions then compute the answer.
- Unit tests live inline at the bottom of each module in a `#[cfg(test)] mod tests { use super::*; ... }` block, asserting against the small examples from the README. This is the established testing convention — keep new logic tested the same way.

When adding a new day, follow this same shape: a thin stdin-driven `main.rs` plus tested domain modules, with `resources/input.txt` and `resources/sample-input.txt` for input.
