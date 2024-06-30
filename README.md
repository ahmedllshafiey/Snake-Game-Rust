# Snake Game

This is a simple implementation of the classic Snake game using Rust and the Raylib library. The player controls a snake to eat food and grow longer, avoiding collisions with the snake's own body.

## Features

- Control the snake using arrow keys
- Snake grows longer each time it eats food
- The game ends if the snake collides with itself
- Icon for the game window

## Prerequisites

- Rust programming language installed
- Raylib library installed

## Getting Started

### Installing Rust

If you don't have Rust installed, you can install it using `rustup`. Follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

### Cloning the Repository

Clone this repository to your local machine:

```sh
git clone https://github.com/yourusername/snake-game.git
cd snake-game
```

### Building the Project

Run the following command to build the project:

```sh
cargo build --release
```

### Running the Game

After building the project, you can run the game using:

```sh
cargo run --release
```

Make sure the `icon.png` file is in the same directory as the source code. This file is used as the icon for the game window.

## Project Structure

- `src/main.rs`: Main file containing the game logic
- `Cargo.toml`: Cargo configuration file
- `icon.png`: Icon for the game window

## Controls

- Arrow keys: Control the direction of the snake

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Raylib](https://www.raylib.com/) - The graphics library used in this project