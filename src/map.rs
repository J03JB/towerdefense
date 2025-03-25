use bevy::prelude::{Component, UVec2, Resource, Vec2, Vec3};

use crate::config::{CELL_SIZE, GRID_HEIGHT, GRID_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(Component)]
pub struct GridCell {
    pub x: usize,
    pub y: usize,
}

#[derive(Resource)]
pub struct Map {
    pub grid_size: Vec2,         // Size of each cell
    pub dimensions: UVec2,       // Map dimensions in cells
    pub path_tiles: Vec<UVec2>,  // List of tiles that form the path
    pub buildable_tiles: Vec<UVec2>, // Tiles where towers can be placed
    pub start: UVec2,            // Enemy spawn point
    pub end: UVec2,              // Enemy goal
}

impl Map {
    pub fn new() -> Self {
        Map {
            grid_size: Vec2::new(CELL_SIZE, CELL_SIZE),
            dimensions: UVec2::new(GRID_WIDTH as u32, GRID_HEIGHT as u32),
            path_tiles: Vec::new(),  // Will be filled based on level data
            buildable_tiles: Vec::new(), // Will be filled later
            start: UVec2::new(0, GRID_HEIGHT as u32 / 2),
            end: UVec2::new(GRID_WIDTH as u32 - 1, GRID_HEIGHT as u32 / 2),
        }
    }

    // Convert grid coordinates to world position considering centered grid
    pub fn grid_to_world(&self, grid_pos: UVec2) -> Vec2 {
        let grid_start_x = -WINDOW_WIDTH / 2.0 + CELL_SIZE / 2.0;
        let grid_start_y = WINDOW_HEIGHT / 2.0 - CELL_SIZE / 2.0;
        
        Vec2::new(
            grid_start_x + grid_pos.x as f32 * CELL_SIZE,
            grid_start_y - grid_pos.y as f32 * CELL_SIZE,
        )
    }
    
    // Convert world position to grid coordinates
    pub fn world_to_grid(&self, world_pos: Vec2) -> UVec2 {
        let grid_start_x = -WINDOW_WIDTH / 2.0 + CELL_SIZE / 2.0;
        let grid_start_y = WINDOW_HEIGHT / 2.0 - CELL_SIZE / 2.0;
        
        let x = ((world_pos.x - grid_start_x) / CELL_SIZE).floor() as u32;
        let y = ((grid_start_y - world_pos.y) / CELL_SIZE).floor() as u32;
        
        UVec2::new(x, y)
    }
    
    // The rest of your Map implementation remains unchanged
    pub fn is_buildable(&self, grid_pos: UVec2) -> bool {
        self.buildable_tiles.contains(&grid_pos)
    }
    
    pub fn get_adjacent_tiles(&self, pos: UVec2) -> Vec<UVec2> {
        let mut adjacent = Vec::new();
        // Add adjacent tiles checking bounds
        // ...
        adjacent
    }
    
    pub fn get_path_positions(&self) -> Vec<Vec2> {
        self.path_tiles.iter().map(|&pos| self.grid_to_world(pos)).collect()
    }
}
