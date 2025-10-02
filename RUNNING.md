# Running the 3D Visualization

## ⚠️ Important Note About Dev Containers

This project creates a **graphical 3D window** that requires display access. Since you're running in a GitHub Codespaces/Dev Container environment, the graphical window **cannot be displayed directly**.

## Recommended Options

### Option 1: Run Locally (BEST OPTION) ✅

The best way to see the visualization is to run this on your local machine:

1. **Clone the repository to your local machine:**
   ```bash
   git clone https://github.com/jasonamilne/sandbox.git
   cd sandbox
   ```

2. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

3. **Run the visualization:**
   ```bash
   cargo run --release
   ```

A beautiful 3D window will open showing your simulation!

### Option 2: Build and Download (For Later Viewing)

Build the executable in the container and download it to run locally:

```bash
cargo build --release
# The binary will be at: target/release/grid_simulation
```

Download `target/release/grid_simulation` to your local machine and run it there.

### Option 3: Web Version (Future Enhancement)

A WebAssembly version could be built for browser viewing. See `WEB_BUILD.md` for more information about this future enhancement.

---

## Why Can't I Run It Here?

The simulation uses **macroquad**, a graphics library that requires:
- A display server (X11, Wayland, or native windowing)
- OpenGL/graphics drivers
- Direct access to create windows

Dev containers and Codespaces are **headless environments** (no display), so graphical windows cannot be shown.

The error you'll see:
```
XOpenDisplay() failed!
```

This means the library tried to connect to a display server but couldn't find one (because there isn't one in a container).

**Good news**: The code compiles successfully! It just needs a display to run.

---

## Controls (When Running Locally)

- **Mouse Drag**: Rotate the 3D view
- **SPACE**: Pause/Resume simulation
- **R**: Toggle auto-rotation
- **ESC**: Quit application

## What You'll See

The visualization shows:
- Alive cells as colorful 3D cubes
- Auto-rotating camera view
- Color-coded depth (blue-purple gradient based on Z position)
- Real-time cell count and status
- Wireframe outlines for depth perception
- Interactive 3D grid

## X11 Forwarding (Advanced - Linux/macOS Only)

If you really want to try running in the container with X11 forwarding:

1. On your host machine, allow X11 connections:
   ```bash
   xhost +local:docker
   ```

2. Make sure DISPLAY is set in the container:
   ```bash
   export DISPLAY=:0
   ```

3. Run:
   ```bash
   cargo run --release
   ```

**Note:** This is complex and often doesn't work well with Codespaces/remote containers.

---

## Summary

✅ **Best approach:** Clone to your local machine and run there  
⚠️ **Current environment:** Cannot display graphical windows  
📦 **Alternative:** Build here, download, run locally
