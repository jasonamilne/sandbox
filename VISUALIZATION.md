# 3D Grid Simulation - Visualization Guide

## What the Visualization Shows

### Main Window (1200x800 pixels)

The application creates a window titled "3D Grid Simulation" with:

#### 3D View (Main Area)
- **Alive Cells**: Rendered as colorful 3D cubes
- **Color Scheme**: 
  - Blue-to-purple gradient based on depth (Z-axis)
  - Darker colors = closer cells
  - Lighter colors = farther cells
- **Wireframe**: Gray outlines around each cube for depth perception
- **Reference Grid**: A dark grid plane at the bottom for orientation
- **Auto-Rotation**: Smooth continuous rotation around the grid center

#### On-Screen Display (Top-Left)
```
Alive Cells: 42
Status: Running (or PAUSED in yellow)
Auto-rotate: ON (or OFF)
```

#### Controls Display (Bottom-Left)
```
Controls:
SPACE: Pause/Resume
Mouse Drag: Rotate
R: Toggle auto-rotate
```

## Initial Pattern

The simulation starts with a 20×20×20 grid containing a structured pattern:
- Cells are placed in a cube region (8-12, 8-12, 8-12)
- Pattern: Cells where `(x + y + z) % 3 == 0` are alive
- This creates an interesting 3D structure that evolves

## Simulation Behavior

## Simulation Rules

The default implementation uses a 3D Game of Life variant with 26-connectivity:

- **Survival**: Living cells with 5-7 neighbors survive
- **Birth**: Dead cells with 6-7 neighbors become alive  
- **Death**: All other cells die

Updates happen every 20 frames (~1/3 second at 60 FPS).

## Visual Features

1. **Depth Perception**
   - Color gradient (blue → purple) along Z-axis
   - Wireframe outlines on cubes
   - Reference grid plane

2. **Camera**
   - Position: (30, 30, 30) looking at grid center
   - Auto-rotates at a gentle pace
   - Manual control via mouse drag

3. **Performance**
   - Runs at 60 FPS target
   - Smooth animations
   - Real-time updates

## Example Evolution

```
Initial: ~20 alive cells (structured cluster + extensions)
Step 1-3: Pattern expands outward
Step 4-10: May stabilize, oscillate, or grow
Step 10+: Creates interesting 3D structures
```

The 3D nature means patterns can evolve in ways impossible in 2D,
creating fascinating emergent behaviors!

## Customization

Edit `src/main.rs` to change:
- Grid size: `Grid3D::new(20, 20, 20)`
- Initial pattern: Modify the initialization loops
- Update speed: Change `update_interval` 
- Colors: Modify the color calculation in render loop
- Rules: Change survival/birth conditions in `Grid3D::update()`
