# Advent of Code 2024 puzzles in Rust

Here are my solutions to Advent of Code 2024 puzzles.

## Project's objective

It's my personal goal to learn Rust. I started learning it just few days before AoC firing off.
I hope to progress in Rust while solving engaging algorithmic riddles.

## Usage
I used JetBrain's template for RustRover
[advent-of-code-rust-template](https://github.com/bravit/advent-of-code-rust-template)
to initialize this repository.

Each days' puzzle is solved in its respective file in `bin/`: `01.rs`, `02.rs` etc.

There is also a template file `NN.rs`. It can be treated as a starting point for each new day:
   - Open the `bin` folder, copy and paste the `NN.rs` file into it, and give it the corresponding name (`01.rs`, `02.rs`, etc.).
   - In the `input` folder, create and fill the input data file (`01.txt`, `02.txt`, etc.).
   - Fill in the `DAY` constant in the freshly created file.
   - Run the current day's solution to check if it compiles (you can use the gutter icon next to the `main` function).
   - Fill in `<TEST-INPUT>`.
   - Write the expected answer for the test data in the `assert_eq` statement in *Part 1*.
   - Now you're ready to write your solution in the `part1` function (inside `main`).
   - Use `Shift+F10` (Win/Linux) or `Ctrl-R` (macOS) to re-run the same program.
   - When you're done with the first part of the puzzle, use folding to hide *Part 1*.
   - Uncomment *Part 2*, fill in the test data assertion, and start solving it.
