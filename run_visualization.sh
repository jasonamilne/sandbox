#!/bin/bash

echo "=== 3D Grid Simulation ==="
echo ""
echo "Starting the graphical visualization..."
echo ""
echo "Note: If you're running in a container or remote environment,"
echo "make sure X11 forwarding is enabled or use the web version."
echo ""
echo "Controls:"
echo "  - Mouse Drag: Rotate view"
echo "  - SPACE: Pause/Resume"
echo "  - R: Toggle auto-rotation"
echo "  - ESC: Quit"
echo ""

# Source Rust environment
source "$HOME/.cargo/env"

# Run the simulation
cargo run --release
