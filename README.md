# Game of Life in Rust

## Description
This repository contains an implementation of the famous Conway's Game of Life, written in Rust. The primary purpose of this project is to serve as an exercise to learn and practice Rust.

The Game of Life is a cellular automaton devised by the British mathematician John Horton Conway in 1970. It is a zero-player game, meaning its evolution is determined by its initial state, requiring no further input during the game.

## How It Works
The game involves a grid where each cell is either alive or dead. The state of each cell changes per turn based on simple rules involving living neighbors.

This implementation not only follows these rules but also visualizes the game board.

## Modular Design
The project is split into two main modules:

- **GameState**: Manages game logic, encapsulating the core rules and game state

- **View**:Handles the rendering, with different implementations for various platforms in the `view` directory, supporting environments from desktops (with libraries like *Piston*) to web browsers (via *WebAssembly*).


## Technologies Used
- Rust

## Installation and Execution

### Prerequisites

- Rust and Cargo from [Rust website](https://www.rust-lang.org/tools/install).
- For local server: [Node.js and npm](https://nodejs.org/) (for `http-server`).

Before you can run this project, you need to have Rust and Cargo (Rust's package manager) installed on your machine. You can download them from .

### Cloning the Repository

```
 git clone https://github.com/fabelium/Game-of-Life-in-Rust.git
 cd Game-of-Life-in-Rust
```

### Running the Game

Execute in the project's root:

```
cargo run
```

### Compiling for WebAssembly

To run the game in a web browser, you first need to compile it to WebAssembly (Wasm) and then serve it using a web server.

1. Build the project:

    ````
    cargo build --target wasm32-unknown-unknown --release
    wasm-bindgen target/wasm32-unknown-unknown/release/game_of_life.wasm --out-dir ./wasm/assets --target web
    ````
   The first command compiles the Rust code into a WebAssembly (**.wasm**) file, optimized for web browser execution.

   The second command uses `wasm-bindgen` to generate the necessary JavaScript files for loading the **.wasm** file. It also copies the generated **.wasm** file into the `./wasm/assets` folder, making it accessible from the browser:


2. Serve with a web server:

    ````
    cd wasm
    npx http-server
    ````

3. Open the provided URL in your browser (usually http://localhost:8080).

## Contributions
We welcome contributions, especially for code efficiency and new features.

## License
Licensed under the GNU General Public License (GPL)

