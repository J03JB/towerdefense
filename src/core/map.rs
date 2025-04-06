use bevy::{prelude::{Component, Resource, UVec2, Vec2, Vec3}, sprite::TextureAtlas};

use crate::core::config::{CELL_SIZE, GRID_HEIGHT, GRID_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(Component)]
pub struct GridCell {
    pub x: usize,
    pub y: usize,
}

#[derive(Resource)]
pub struct Map {
    pub grid_size: Vec2,             
    pub dimensions: UVec2,           
    pub path_tiles: Vec<UVec2>,      
    pub buildable_tiles: Vec<UVec2>, 
    pub start: UVec2,               
    pub end: UVec2,                 
}

impl Default for Map {
    fn default() -> Self {
        Map {
            grid_size: Vec2::new(CELL_SIZE, CELL_SIZE),
            dimensions: UVec2::new(GRID_WIDTH as u32, GRID_HEIGHT as u32),
            path_tiles: Vec::new(),      // Will be filled based on level data
            buildable_tiles: Vec::new(), // Will be filled later
            start: UVec2::new(0, GRID_HEIGHT as u32 / 2),
            end: UVec2::new(GRID_WIDTH as u32 - 1, GRID_HEIGHT as u32 / 2),
        }
    }
}

impl Map {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn grid_to_world(&self, grid_pos: UVec2) -> Vec2 {
        let grid_origin_x = -WINDOW_WIDTH / 2.0;
        let grid_origin_y = WINDOW_HEIGHT / 2.0;

        let cell_corner_x = grid_origin_x + grid_pos.x as f32 * self.grid_size.x;
        let cell_corner_y = grid_origin_y - grid_pos.y as f32 * self.grid_size.y;

        // Return the center of the cell
        Vec2::new(
            cell_corner_x + self.grid_size.x / 2.0,
            cell_corner_y - self.grid_size.y / 2.0,
        )
    }

    pub fn world_to_grid(&self, world_pos: Vec2) -> UVec2 {
        let grid_origin_x = -WINDOW_WIDTH / 2.0;
        let grid_origin_y = WINDOW_HEIGHT / 2.0;

        let relative_x = world_pos.x - grid_origin_x;
        let relative_y = grid_origin_y - world_pos.y;

        let grid_x = (relative_x / self.grid_size.x)
            .floor()
            .max(0.0) 
            .min((self.dimensions.x - 1) as f32);

        let grid_y = (relative_y / self.grid_size.y)
            .floor()
            .max(0.0) 
            .min((self.dimensions.y - 1) as f32); 

        UVec2::new(grid_x as u32, grid_y as u32)
    }

    pub fn is_buildable(&self, grid_pos: UVec2) -> bool {
        self.buildable_tiles.contains(&grid_pos)
    }

    pub fn get_adjacent_tiles(&self, pos: UVec2) -> Vec<UVec2> {
        let mut adjacent = Vec::new();
        let x = pos.x;
        let y = pos.y;
        let width = self.dimensions.x;
        let height = self.dimensions.y;

        if y > 0 {
            adjacent.push(UVec2::new(x, y - 1));
        }
        if y < height - 1 {
            adjacent.push(UVec2::new(x, y + 1));
        }
        if x > 0 {
            adjacent.push(UVec2::new(x - 1, y));
        }
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
