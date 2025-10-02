use macroquad::prelude::*;

/// Represents a cell in the 3D grid
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Empty,
    Alive,
    // Add more cell types as needed
}

/// A 3D grid structure optimized for performance and scalability
pub struct Grid3D {
    width: usize,
    height: usize,
    depth: usize,
    cells: Vec<Cell>,
}

impl Grid3D {
    /// Create a new 3D grid with specified dimensions
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        let total_cells = width * height * depth;
        Grid3D {
            width,
            height,
            depth,
            cells: vec![Cell::Empty; total_cells],
        }
    }

    /// Get the linear index from 3D coordinates
    #[inline]
    fn get_index(&self, x: usize, y: usize, z: usize) -> usize {
        x + y * self.width + z * self.width * self.height
    }

    /// Get a cell at specified coordinates
    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<Cell> {
        if x >= self.width || y >= self.height || z >= self.depth {
            return None;
        }
        let index = self.get_index(x, y, z);
        Some(self.cells[index])
    }

    /// Set a cell at specified coordinates
    pub fn set(&mut self, x: usize, y: usize, z: usize, cell: Cell) -> bool {
        if x >= self.width || y >= self.height || z >= self.depth {
            return false;
        }
        let index = self.get_index(x, y, z);
        self.cells[index] = cell;
        true
    }

    /// Count alive neighbors for a given cell (26-connectivity in 3D)
    pub fn count_alive_neighbors(&self, x: usize, y: usize, z: usize) -> usize {
        let mut count = 0;
        
        for dz in -1..=1 {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue; // Skip the cell itself
                    }
                    
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    let nz = z as i32 + dz;
                    
                    if nx >= 0 && nx < self.width as i32 
                        && ny >= 0 && ny < self.height as i32
                        && nz >= 0 && nz < self.depth as i32 {
                        if let Some(Cell::Alive) = self.get(nx as usize, ny as usize, nz as usize) {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    /// Update the grid based on simulation rules (similar to Conway's Game of Life in 3D)
    pub fn update(&mut self) {
        let mut new_cells = self.cells.clone();
        
        for z in 0..self.depth {
            for y in 0..self.height {
                for x in 0..self.width {
                    let neighbors = self.count_alive_neighbors(x, y, z);
                    let current = self.get(x, y, z).unwrap();
                    let index = self.get_index(x, y, z);
                    
                    // 3D Life rules (more permissive for 26-neighborhood):
                    // - A living cell survives if it has 5-7 neighbors
                    // - A dead cell becomes alive if it has 6-7 neighbors
                    // These rules create more stable and interesting patterns in 3D
                    new_cells[index] = match current {
                        Cell::Alive => {
                            if (5..=7).contains(&neighbors) {
                                Cell::Alive
                            } else {
                                Cell::Empty
                            }
                        }
                        Cell::Empty => {
                            if (6..=7).contains(&neighbors) {
                                Cell::Alive
                            } else {
                                Cell::Empty
                            }
                        }
                    };
                }
            }
        }
        
        self.cells = new_cells;
    }

    /// Count total alive cells in the grid
    pub fn count_alive(&self) -> usize {
        self.cells.iter().filter(|&&c| c == Cell::Alive).count()
    }

    /// Get grid dimensions
    pub fn dimensions(&self) -> (usize, usize, usize) {
        (self.width, self.height, self.depth)
    }

    /// Iterate over all cells with their positions
    pub fn iter_cells(&self) -> impl Iterator<Item = (usize, usize, usize, Cell)> + '_ {
        (0..self.depth).flat_map(move |z| {
            (0..self.height).flat_map(move |y| {
                (0..self.width).map(move |x| {
                    (x, y, z, self.get(x, y, z).unwrap())
                })
            })
        })
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "3D Grid Simulation".to_owned(),
        window_width: 1200,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Create a 20x20x20 grid for better visualization
    let mut grid = Grid3D::new(20, 20, 20);
    
    // Initialize with a more interesting pattern
    // Create multiple structures that should evolve
    println!("Initializing grid with patterns...");
    
    // Central cluster
    for x in 9..11 {
        for y in 9..11 {
            for z in 9..11 {
                grid.set(x, y, z, Cell::Alive);
            }
        }
    }
    
    // Add some surrounding cells to create evolution
    grid.set(8, 10, 10, Cell::Alive);
    grid.set(11, 10, 10, Cell::Alive);
    grid.set(10, 8, 10, Cell::Alive);
    grid.set(10, 11, 10, Cell::Alive);
    grid.set(10, 10, 8, Cell::Alive);
    grid.set(10, 10, 11, Cell::Alive);
    
    // Add a few more scattered cells for interest
    grid.set(7, 9, 10, Cell::Alive);
    grid.set(12, 10, 10, Cell::Alive);
    grid.set(10, 7, 10, Cell::Alive);
    grid.set(10, 12, 10, Cell::Alive);
    
    println!("Initial alive cells: {}", grid.count_alive());
    
    let mut rotation_x: f32 = 0.0;
    let mut rotation_y: f32 = 0.0;
    let mut auto_rotate = true;
    let mut paused = false;
    let mut frame_count = 0;
    let update_interval = 20; // Update simulation every N frames (slower for better observation)
    
    println!("=== 3D Grid Simulation ===");
    println!("Controls:");
    println!("  Mouse: Drag to rotate view");
    println!("  SPACE: Pause/Resume simulation");
    println!("  R: Toggle auto-rotation");
    println!("  UP/DOWN: Adjust update speed");
    println!("  ESC: Quit");
    
    loop {
        clear_background(BLACK);
        
        // Handle input
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
            println!("{}", if paused { "Paused" } else { "Resumed" });
        }
        
        if is_key_pressed(KeyCode::R) {
            auto_rotate = !auto_rotate;
            println!("Auto-rotate: {}", auto_rotate);
        }
        
        // Manual rotation with mouse
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_delta = mouse_delta_position();
            rotation_y += mouse_delta.x * 0.01;
            rotation_x += mouse_delta.y * 0.01;
            auto_rotate = false;
        }
        
        // Auto rotation
        if auto_rotate {
            rotation_y += 0.005;
            rotation_x += 0.003;
        }
        
        // Update simulation
        if !paused {
            frame_count += 1;
            if frame_count >= update_interval {
                grid.update();
                frame_count = 0;
            }
        }
        
        // Set up 3D camera
        set_camera(&Camera3D {
            position: vec3(30.0, 30.0, 30.0),
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(10.0, 10.0, 10.0),
            ..Default::default()
        });
        
        // Draw 3D grid
        let (width, height, depth) = grid.dimensions();
        let cell_size = 0.8;
        let spacing = 1.0;
        
        // Calculate center offset for centering the grid
        let offset_x = -(width as f32 * spacing) / 2.0;
        let offset_y = -(height as f32 * spacing) / 2.0;
        let offset_z = -(depth as f32 * spacing) / 2.0;
        
        // Draw alive cells
        for (x, y, z, cell) in grid.iter_cells() {
            if cell == Cell::Alive {
                let px = offset_x + x as f32 * spacing;
                let py = offset_y + y as f32 * spacing;
                let pz = offset_z + z as f32 * spacing;
                
                // Rotate around center
                let (rx, ry, rz) = rotate_point(px, py, pz, rotation_x, rotation_y);
                
                // Color based on position for depth perception
                let color_intensity = (z as f32 / depth as f32) * 0.5 + 0.5;
                let color = Color::new(
                    0.2 + color_intensity * 0.6,
                    0.5 + color_intensity * 0.3,
                    0.8 + color_intensity * 0.2,
                    0.9,
                );
                
                draw_cube(
                    vec3(rx, ry, rz),
                    vec3(cell_size, cell_size, cell_size),
                    None,
                    color,
                );
                
                // Draw wireframe for better depth perception
                draw_cube_wires(
                    vec3(rx, ry, rz),
                    vec3(cell_size, cell_size, cell_size),
                    DARKGRAY,
                );
            }
        }
        
        // Draw reference grid plane
        draw_grid(20, 1.0, DARKGRAY, GRAY);
        
        // Switch back to 2D for UI
        set_default_camera();
        
        // Draw UI
        let alive_count = grid.count_alive();
        draw_text(
            &format!("Alive Cells: {}", alive_count),
            10.0, 30.0, 30.0, WHITE
        );
        draw_text(
            &format!("Status: {}", if paused { "PAUSED" } else { "Running" }),
            10.0, 60.0, 30.0, if paused { YELLOW } else { GREEN }
        );
        draw_text(
            &format!("Auto-rotate: {}", if auto_rotate { "ON" } else { "OFF" }),
            10.0, 90.0, 30.0, WHITE
        );
        
        // Instructions
        draw_text("Controls:", 10.0, screen_height() - 100.0, 20.0, GRAY);
        draw_text("SPACE: Pause/Resume", 10.0, screen_height() - 75.0, 18.0, GRAY);
        draw_text("Mouse Drag: Rotate", 10.0, screen_height() - 55.0, 18.0, GRAY);
        draw_text("R: Toggle auto-rotate", 10.0, screen_height() - 35.0, 18.0, GRAY);
        
        next_frame().await
    }
}

/// Rotate a point around the origin
fn rotate_point(x: f32, y: f32, z: f32, angle_x: f32, angle_y: f32) -> (f32, f32, f32) {
    // Rotate around Y axis
    let cos_y = angle_y.cos();
    let sin_y = angle_y.sin();
    let x1 = x * cos_y - z * sin_y;
    let z1 = x * sin_y + z * cos_y;
    
    // Rotate around X axis
    let cos_x = angle_x.cos();
    let sin_x = angle_x.sin();
    let y1 = y * cos_x - z1 * sin_x;
    let z2 = y * sin_x + z1 * cos_x;
    
    (x1, y1, z2)
}
