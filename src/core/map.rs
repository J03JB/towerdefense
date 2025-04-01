use bevy::prelude::{Component, Resource, UVec2, Vec2, Vec3};

use crate::core::config::{CELL_SIZE, GRID_HEIGHT, GRID_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(Component)]
pub struct GridCell {
    pub x: usize,
    pub y: usize,
}

#[derive(Resource)]
pub struct Map {
    pub grid_size: Vec2,             // Size of each cell
    pub dimensions: UVec2,           // Map dimensions in cells
    pub path_tiles: Vec<UVec2>,      // List of tiles that form the path
    pub buildable_tiles: Vec<UVec2>, // Tiles where towers can be placed
    pub start: UVec2,                // Enemy spawn point
    pub end: UVec2,                  // Enemy goal
}

impl Map {
    pub fn new() -> Self {
        Map {
            grid_size: Vec2::new(CELL_SIZE, CELL_SIZE),
            dimensions: UVec2::new(GRID_WIDTH as u32, GRID_HEIGHT as u32),
            path_tiles: Vec::new(),      // Will be filled based on level data
            buildable_tiles: Vec::new(), // Will be filled later
            start: UVec2::new(0, GRID_HEIGHT as u32 / 2),
            end: UVec2::new(GRID_WIDTH as u32 - 1, GRID_HEIGHT as u32 / 2),
        }
    }

    // Convert grid coordinates to world position (CENTER of the cell)
    pub fn grid_to_world(&self, grid_pos: UVec2) -> Vec2 {
        // World coordinate of the top-left corner of the grid
        let grid_origin_x = -WINDOW_WIDTH / 2.0;
        let grid_origin_y = WINDOW_HEIGHT / 2.0;

        // Calculate the top-left corner of the specific cell
        let cell_corner_x = grid_origin_x + grid_pos.x as f32 * self.grid_size.x;
        let cell_corner_y = grid_origin_y - grid_pos.y as f32 * self.grid_size.y; // Subtract because grid Y increases downwards

        // Return the center of the cell
        Vec2::new(
            cell_corner_x + self.grid_size.x / 2.0,
            cell_corner_y - self.grid_size.y / 2.0, // Subtract half size to get center Y
        )
    }
    // Convert grid coordinates to world position considering centered grid
    // pub fn grid_to_world(&self, grid_pos: UVec2) -> Vec2 {
    //     let grid_start_x = -WINDOW_WIDTH / 2.0 + CELL_SIZE / 2.0;
    //     let grid_start_y = WINDOW_HEIGHT / 2.0 - CELL_SIZE / 2.0;
    //
    //     Vec2::new(
    //         grid_start_x + grid_pos.x as f32 * CELL_SIZE,
    //         grid_start_y - grid_pos.y as f32 * CELL_SIZE,
    //     )
    // }

    // Convert world position to grid coordinates (based on cell boundaries)
    pub fn world_to_grid(&self, world_pos: Vec2) -> UVec2 {
        // World coordinate of the top-left corner of the grid
        let grid_origin_x = -WINDOW_WIDTH / 2.0;
        let grid_origin_y = WINDOW_HEIGHT / 2.0;

        // Calculate position relative to the grid origin
        let relative_x = world_pos.x - grid_origin_x;
        let relative_y = grid_origin_y - world_pos.y; // Inverted because grid Y increases downwards

        // Calculate grid indices using floor. Clamp to grid bounds.
        let grid_x = (relative_x / self.grid_size.x)
            .floor()
            .max(0.0) // Ensure non-negative
            .min((self.dimensions.x - 1) as f32); // Ensure within max width

        let grid_y = (relative_y / self.grid_size.y)
            .floor()
            .max(0.0) // Ensure non-negative
            .min((self.dimensions.y - 1) as f32); // Ensure within max height

        UVec2::new(grid_x as u32, grid_y as u32)
    }

    // Convert world position to grid coordinates
    // pub fn world_to_grid(&self, world_pos: Vec2) -> UVec2 {
    //     let grid_start_x = -WINDOW_WIDTH / 2.0 + CELL_SIZE / 2.0;
    //     let grid_start_y = WINDOW_HEIGHT / 2.0 - CELL_SIZE / 2.0;
    //
    //     let x = ((world_pos.x - grid_start_x) / CELL_SIZE).floor() as u32;
    //     let y = ((grid_start_y - world_pos.y) / CELL_SIZE).floor() as u32;
    //
    //     UVec2::new(x, y)
    // }
    //
    // The rest of your Map implementation remains unchanged
    pub fn is_buildable(&self, grid_pos: UVec2) -> bool {
        self.buildable_tiles.contains(&grid_pos)
    }

    pub fn get_adjacent_tiles(&self, pos: UVec2) -> Vec<UVec2> {
        let mut adjacent = Vec::new();
        let x = pos.x;
        let y = pos.y;
        let width = self.dimensions.x;
        let height = self.dimensions.y;

        // Check North (y-1)
        if y > 0 {
            adjacent.push(UVec2::new(x, y - 1));
        }
        // Check South (y+1)
        if y < height - 1 {
            adjacent.push(UVec2::new(x, y + 1));
        }
        // Check West (x-1)
        if x > 0 {
            adjacent.push(UVec2::new(x - 1, y));
        }
        // Check East (x+1)
        if x < width - 1 {
            adjacent.push(UVec2::new(x + 1, y));
        }

        adjacent
    }
    pub fn get_path_positions(&self) -> Vec<Vec2> {
        self.path_tiles
            .iter()
            .map(|&pos| self.grid_to_world(pos))
            .collect()
    }
}
