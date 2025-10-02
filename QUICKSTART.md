# Quick Start Guide

## What You Have

A complete 3D grid simulation with real-time graphical visualization built in Rust.

## Files Overview

- **`src/main.rs`** - Complete simulation with 3D graphics (macroquad)
- **`Cargo.toml`** - Dependencies and build configuration  
- **`config.toml`** - Simulation parameters
- **`README.md`** - Main documentation
- **`RUNNING.md`** - Detailed run instructions for different environments
- **`VISUALIZATION.md`** - What you'll see in the visualization
- **`WEB_BUILD.md`** - Optional web/WASM build instructions
- **`run_visualization.sh`** - Quick launch script

## Quick Commands

### Build the project:
```bash
source "$HOME/.cargo/env"
cargo build --release
```

### Run the simulation:
```bash
cargo run --release
```

### Run with the script:
```bash
./run_visualization.sh
```

## What It Does

1. **Creates a 20×20×20 3D grid** with cellular automaton cells
2. **Opens a graphical window** showing the grid in 3D
3. **Displays alive cells** as colored cubes with depth-coded colors
4. **Auto-rotates** the camera for a dynamic view
5. **Updates in real-time** following 3D Game of Life rules
6. **Supports interaction** - pause, manual rotation, speed control

## Controls

- **Mouse Drag**: Rotate view
- **SPACE**: Pause/Resume
- **R**: Toggle auto-rotation  
- **ESC**: Quit

## Important Note

Since you're in a dev container, the graphical window requires:
- X11 forwarding enabled, OR
- Running the code on your local machine

See **RUNNING.md** for detailed instructions.

## Next Steps

- Run locally to see the full 3D visualization
- Modify grid size in `src/main.rs`
- Customize colors and patterns
- Add parallel processing with rayon
- Experiment with different cellular automaton rules

Enjoy your 3D grid simulation! 🎮🔵🟣
