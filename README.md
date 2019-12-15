# Advent of Code 2019

This repository contains my solutions to the Advent of Code puzzles for 2019. If
you're new to the concept of Advent of Code, please check [the official
page](https://adventofcode.com/2019/about).

# Project Structure

- The `input/dayXX` file contains each respective puzzle input.
- The `src/dayXX.rs` file contains solutions to each respective day's puzzles.
- The `src/utils.rs` file contains general utilities such as those for reading
  file input.

# Running the Code

Every day's solution is generated under a separate binary for ease of use. This
can be run using `cargo` itself, like follows:

```
$ cargo run day01
Day 01, Part 1: 3352674
Day 01, Part 2: 5026151
```

# License

This project is licensed under the MIT license. Please check the
[license](LICENSE) file for more details.
