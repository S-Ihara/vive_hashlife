# vive_hashlife

Conway's Game of Life using the HashLife algorithm in Rust with WebAssembly, deployable to GitHub Pages.

![Game of Life UI](https://github.com/user-attachments/assets/b169aefc-fa84-405a-8720-3f6dc1057127)

## Features

- 🦀 **HashLife Algorithm** - Implements Bill Gosper's HashLife with quadtree and memoization
- 🌐 **WebAssembly** - Runs in the browser via WASM with near-native speed
- 🎨 **Interactive UI** - Click to toggle cells, real-time visualization
- ⚡ **Exponential speedup** - Memoization provides massive performance gains for large patterns
- 🔍 **Zoom control** - Adjust cell size for better visibility
- 📐 **Classic patterns** - Pre-loaded patterns including:
  - Glider
  - Gosper Glider Gun
  - Pulsar
  - LWSS (Lightweight Spaceship)
  - Acorn
- 📊 **Live statistics** - Track generation count, population, and FPS
- ⌨️ **Keyboard shortcuts** - Quick controls for power users

## Live Demo

Visit the live demo: [https://s-ihara.github.io/vive_hashlife/](https://s-ihara.github.io/vive_hashlife/)

## Controls

### Mouse
- **Click** on canvas to toggle cells

### Keyboard
- **Space** - Play/Pause
- **S** - Step forward one generation
- **C** - Clear all cells
- **R** - Randomize the board

### UI Buttons
- **Play/Pause** - Start or stop the simulation
- **Step** - Advance one generation
- **Clear** - Remove all cells
- **Random** - Fill board with random cells
- **Pattern buttons** - Load classic Game of Life patterns

### Sliders
- **FPS** - Control simulation speed (1-60 FPS)
- **Cell Size** - Adjust zoom level (4-20 pixels)

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Building

1. Clone the repository:
```bash
git clone https://github.com/S-Ihara/vive_hashlife.git
cd vive_hashlife
```

2. Build the WASM module:
```bash
wasm-pack build --target web
```

3. Serve locally:
```bash
python3 -m http.server 8000
```

4. Open http://localhost:8000 in your browser

### Running Tests

```bash
cargo test
```

### Project Structure

```
vive_hashlife/
├── src/
│   ├── lib.rs           # Library entry point
│   ├── hashlife.rs      # Game of Life implementation
│   └── wasm.rs          # WebAssembly bindings
├── index.html           # Web UI
├── Cargo.toml           # Rust dependencies
└── .github/
    └── workflows/
        └── deploy.yml   # GitHub Pages deployment
```

## HashLife Algorithm

This implementation uses the HashLife algorithm invented by Bill Gosper in 1984. HashLife achieves exponential speedup for large patterns through two key techniques:

### Quadtree Structure
- Universe represented as a recursive quadtree
- Each node has 4 quadrants (NW, NE, SW, SE)
- Nodes are immutable and share structure via `Rc<Node>`

### Memoization
- Results for identical subtrees are cached
- Avoids recomputing the same patterns
- Provides exponential speedup for repetitive structures

### Multi-Step Advancement
The algorithm advances by 2^(level-2) generations per step:
- Level 3 (8×8): 2 generations per step
- Level 4 (16×16): 4 generations per step  
- Level 5 (32×32): 8 generations per step

This makes HashLife extremely efficient for simulating large, stable patterns over many generations.

### Base Case
For a 4×4 region (level 2), the algorithm:
1. Extracts the 16 cells
2. Applies Conway's rules to the center 2×2
3. Returns the result after 1 generation

### Recursive Case
For larger regions (level > 2):
1. Divide into 9 overlapping subregions
2. Recursively compute next generation for each
3. Combine results into 4 output quadrants

## Conway's Game of Life Rules

The Game of Life is a cellular automaton devised by mathematician John Conway. It consists of a grid of cells that can be either alive or dead. The state of the grid evolves in discrete time steps according to these rules:

1. Any live cell with fewer than 2 live neighbors dies (underpopulation)
2. Any live cell with 2 or 3 live neighbors lives on to the next generation
3. Any live cell with more than 3 live neighbors dies (overpopulation)
4. Any dead cell with exactly 3 live neighbors becomes a live cell (reproduction)

## Deployment

The project automatically deploys to GitHub Pages when changes are pushed to the `main` branch via GitHub Actions.

## License

This project is open source and available under the MIT License.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## Acknowledgments

- Bill Gosper for inventing the HashLife algorithm
- John Conway for creating the Game of Life
- The Rust and WebAssembly communities for excellent tooling