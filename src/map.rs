use bevy::prelude::{UVec2, Resource, Vec2};
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
    // Convert grid coordinates to world position
    pub fn grid_to_world(&self, grid_pos: UVec2) -> Vec2 {
        Vec2::new(
            grid_pos.x as f32 * self.grid_size.x + self.grid_size.x * 0.5,
            grid_pos.y as f32 * self.grid_size.y + self.grid_size.y * 0.5,
        )
    }
    
    // Convert world position to grid coordinates
    pub fn world_to_grid(&self, world_pos: Vec2) -> UVec2 {
        UVec2::new(
            (world_pos.x / self.grid_size.x) as u32,
            (world_pos.y / self.grid_size.y) as u32,
        )
    }
    
    // Check if a tile is buildable
    pub fn is_buildable(&self, grid_pos: UVec2) -> bool {
        self.buildable_tiles.contains(&grid_pos)
    }
    
    // Get adjacent tiles (useful for pathfinding)
    pub fn get_adjacent_tiles(&self, pos: UVec2) -> Vec<UVec2> {
        let mut adjacent = Vec::new();
        // Add adjacent tiles checking bounds
        // ...
        adjacent
    }
    
    // Get path as world coordinates
    pub fn get_path_positions(&self) -> Vec<Vec2> {
        self.path_tiles.iter().map(|&pos| self.grid_to_world(pos)).collect()
    }
}
