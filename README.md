# 3D Grid Simulation

A high-performance, scalable three-dimensional grid simulation written in Rust with **real-time 3D visualization**. This project implements a 3D variant of cellular automaton rules (similar to Conway's Game of Life) with optimizations for low latency and scalability.

> **⚠️ Note for Dev Container Users**: This application creates a graphical 3D window and cannot run in headless environments like GitHub Codespaces. See [RUNNING.md](RUNNING.md) for how to run it on your local machine.

## Features

- **Real-time 3D Visualization**: Interactive graphical window showing the simulation in 3D
- **High Performance**: Optimized Rust implementation with minimal overhead
- **Interactive Controls**: Mouse-controlled rotation, pause/resume, and adjustable speeds
- **Scalable Architecture**: Designed to handle large grids efficiently
- **Configurable**: Easily adjust grid dimensions, simulation rules, and parameters
- **Extensible**: Clean API for adding custom cell types and simulation logic
- **Memory Efficient**: Contiguous memory layout for cache-friendly operations

## Screenshots

The simulation displays alive cells as colored cubes in a 3D space with:
- Color-coded depth perception (cells further away have different colors)
- Auto-rotating camera view
- Real-time cell count and status display
- Wireframe outlines for better depth perception

## Project Structure

```
.
├── Cargo.toml              # Rust project configuration
├── config.toml             # Simulation configuration
├── README.md               # This file
├── RUNNING.md              # Detailed instructions for running the visualization
├── WEB_BUILD.md            # Instructions for web/WASM build
├── run_visualization.sh    # Quick start script
└── src/
    └── main.rs             # Main simulation code with 3D graphics
```

## Configuration

The simulation behavior is defined in `config.toml`:

- **Grid dimensions**: Adjust width, height, and depth
- **Simulation rules**: Configure survival/birth conditions
- **Performance**: Enable parallel processing for large grids
- **Output**: Control logging and verbosity

## Building and Running

### Prerequisites

- Rust toolchain (rustc 1.70+)
- Cargo package manager
- OpenGL support (usually pre-installed on Linux/macOS/Windows)

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```

A graphical window will pop up showing the 3D simulation in real-time!

**Note for Dev Containers/Remote Environments:**
If you're running in a container or remote server without display access, see [RUNNING.md](RUNNING.md) for detailed instructions on X11 forwarding or alternative visualization methods.

## Controls

- **Mouse Drag**: Rotate the 3D view
- **SPACE**: Pause/Resume the simulation
- **R**: Toggle auto-rotation
- **ESC**: Quit the application

The simulation runs continuously with an auto-rotating camera view showing alive cells as colorful 3D cubes.

## Usage

The basic simulation creates a 10x10x10 grid and runs for 10 steps. To customize:

1. Edit `config.toml` for configuration changes
2. Modify `src/main.rs` to change initialization patterns
3. Adjust simulation rules in the `update()` method

## Simulation Rules

The default implementation uses a 3D Game of Life variant with 26-connectivity:

- **Survival**: Living cells with 4-5 neighbors survive
- **Birth**: Dead cells with exactly 5 neighbors become alive
- **Death**: All other cells die or remain empty

These rules can be customized in the code.

## Scaling for Large Grids

For large-scale simulations:

1. Enable the `rayon` dependency in `Cargo.toml`
2. Implement parallel iteration in the `update()` method
3. Compile with `--release` flag for optimizations
4. Consider memory-mapped files for very large grids

## Performance Tips

- Use `--release` builds for production simulations
- Profile with `cargo flamegraph` to identify bottlenecks
- Consider GPU acceleration with CUDA/OpenCL for massive grids
- Implement sparse grid representations for mostly-empty spaces

## Future Enhancements

- [x] 3D graphical visualization
- [x] Interactive camera controls
- [ ] Parallel processing with rayon
- [ ] GPU acceleration
- [ ] Advanced visualization with shadows and lighting
- [ ] Sparse grid optimization
- [ ] Save/load simulation states
- [ ] Network synchronization for distributed simulations
- [ ] Custom rule configuration from file
- [ ] VR support for immersive visualization

## License

MIT License - Feel free to use and modify for your projects.
