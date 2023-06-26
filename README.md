# Cube Scramble Generator
## Overview
Cube is a simple yet effective scramble generator for a 3x3 Rubik's Cube, written in Rust. It ensures no two consecutive moves are the same, providing an unpredictable and challenging scramble.
## Functionality
Cube's core function, `generate_scramble`, generates a random sequence of moves. The moves are denoted as "R", "L", "B", "F", "U", "D", "", "'", "2" corresponding to Right, Left, Back, Front, Up, Down, none, counter-clockwise (prime), and a 180-degree turn respectively.
The scramble generator ensures that the same move is not repeated consecutively. By default, the generator outputs 20 moves. However, you can customize this by using the `-c` or `--count` argument when running the program.
## Usage
### Building the Project
Navigate to the project directory and build the project using Cargo:
```sh
cargo build --release
```
### Running the Program
You can run the program with the following command:
```sh
./target/release/cube -c 25
```
This generates a scramble of 25 moves. If you don't specify the `-c` or `--count` option, the program defaults to generating 20 moves.
## Dependencies
This project depends on the `rand` and `argparse` crates. These are specified in the `Cargo.toml` file in the project's root directory.
## Conclusion
Whether you're a beginner or a seasoned Rubik's Cube solver, the Cube scramble generator provides an easy-to-use tool for generating a variety of scrambles. It's flexibility in the number of generated moves allows for a wide range of scramble complexities. Happy cubing!
