# Evolution Simulation

A real-time evolution simulation built with **Rust** and **WebAssembly**, running live in the browser.

Birds (triangles) learn to find food (circles) over generations using a **neural network** as their brain and a **genetic algorithm** to evolve and improve their behaviour over time — all without any hard-coded rules.



## Preview


https://github.com/user-attachments/assets/bd6bf59b-76c7-4819-8386-3c53e3205e95


> Birds (white triangles) learn to navigate toward food (green circles) over successive generations.

---

## How It Works

The simulation is based on three core concepts:

### 1. Neural Network (the brain)
Each bird has a brain — a feedforward neural network (FFNN) — that takes in what the bird **sees** and outputs how fast it should go and which way it should turn.

```
Eye (9 cells)  →  Hidden Layer (18 neurons)  →  Output (speed, rotation)
```

The brain starts with completely **random weights** — the birds initially fly around with no purpose. Over time, the genetic algorithm improves the weights.

### 2. Genetic Algorithm (the evolution)
Every generation, the algorithm:
1. **Evaluates** each bird by how much food it ate (fitness score)
2. **Selects** better-performing birds more often (roulette wheel selection)
3. **Crosses over** two parents' neural network weights to produce a child
4. **Mutates** the child's weights slightly to introduce new behaviour

Over many generations, birds get progressively better at finding food.

### 3. Eye (the senses)
Each bird has a field of view split into **9 cells**. Each cell tells the bird how close the nearest food is in that direction:

```
[ 0.0, 0.0, 0.8, 0.0, 0.0, 0.0, 0.3, 0.0, 0.0 ]
         ↑                       ↑
   food close ahead         food to the right
```

---

## Project Structure

```
evolutionsimulation/
│
├── src/                        # Core Rust library
│   ├── lib.rs                  # Exposes all modules as a library
│   ├── main.rs                 # Entry point (empty, logic is in lib)
│   ├── network.rs              # Feedforward neural network
│   ├── genetic_algorithm.rs    # Genetic algorithm (selection, crossover, mutation)
│   ├── simulation.rs           # World, Animal, Food, Simulation logic
│   └── eye.rs                  # Bird's vision / field of view
│
├── simulation-wasm/            # WebAssembly bridge
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs              # Wraps Rust structs with #[wasm_bindgen]
│                               # so JavaScript can call them
│
└── frontend/                        # JavaScript frontend
    ├── index.html              # Page layout
    ├── index.js                # Canvas rendering + animation loop
    ├── bootstrap.js            # Loads index.js asynchronously
    ├── package.json            # npm dependencies
    └── webpack.config.js       # Webpack bundler config
```

---

## Architecture

```
┌─────────────────────────────────────────────┐
│               Browser (www/)                │
│                                             │
│   index.js                                  │
│   - draws canvas                            │
│   - calls simulation.step() each frame      │
│   - renders animals + foods                 │
└─────────────────┬───────────────────────────┘
                  │ imports
┌─────────────────▼───────────────────────────┐
│         WebAssembly Bridge                  │
│         (simulation-wasm/)                  │
│                                             │
│   #[wasm_bindgen] wrappers                  │
│   - Simulation::new()                       │
│   - Simulation::step()                      │
│   - Simulation::world()                     │
└─────────────────┬───────────────────────────┘
                  │ uses
┌─────────────────▼───────────────────────────┐
│           Core Rust Library (src/)          │
│                                             │
│   network.rs         → neural network       │
│   genetic_algorithm  → evolution logic      │
│   simulation.rs      → world physics        │
│   eye.rs             → bird vision          │
└─────────────────────────────────────────────┘
```

---

## Tech Stack

| Technology | Purpose |
|---|---|
| Rust | Core simulation logic |
| WebAssembly | Runs Rust in the browser at near-native speed |
| wasm-bindgen | Connects Rust and JavaScript |
| wasm-pack | Compiles Rust to WebAssembly |
| JavaScript | Canvas rendering and animation |
| Webpack 5 | Bundles JavaScript modules |
| nalgebra | Math library for 2D positions and rotations |
| rand | Random number generation |

---

## Getting Started

### Prerequisites

Make sure you have these installed:

- [Rust](https://rustup.rs/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node.js + npm](https://nodejs.org/)

### Installation

**1. Clone the repository**
```bash
git clone https://github.com/YOUR_USERNAME/evolutionsimulation.git
cd evolutionsimulation
```

**2. Build the WebAssembly package**
```bash
cd simulation-wasm
wasm-pack build --target bundler
```

**3. Install frontend dependencies**
```bash
cd ../frontend
npm install
```

**4. Run the simulation**
```bash
npm run start
```

**5. Open your browser at:**
```
http://localhost:8080
```

---

## Running Tests

From the project root:
```bash
cargo test
```

Tests cover:
- Neural network propagation and random initialization
- Genetic algorithm selection, crossover and mutation
- Bird eye vision and field of view

---

## Concepts Explained

### Why WebAssembly?
JavaScript alone is too slow for running neural networks and physics simulations in real time. Rust compiled to WebAssembly runs at near-native speed in the browser — giving us the best of both worlds: Rust's performance and the browser's rendering.

### Why a Genetic Algorithm instead of Backpropagation?
Backpropagation (the standard way to train neural networks) requires labelled training data — for example "when you see food on the left, turn left." We don't have that data. Instead, we let birds figure it out themselves by evolving over many generations — no labels needed.

### Why separate crates?
The core simulation (`src/`) knows nothing about browsers or JavaScript. The WebAssembly bridge (`simulation-wasm/`) is a thin wrapper on top. This separation means the same simulation logic could also power a CLI tool, a desktop app, or a game engine in the future.

---

## Inspired By

This project is based on the excellent **[Learning to Fly](https://pwy.io/posts/learning-to-fly-pt1/)** tutorial series by [Patryk Wychowaniec](https://pwy.io), adapted into a flat Rust project structure.

---

## License

MIT
