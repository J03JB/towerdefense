use bevy::prelude::*;

use crate::core::config::{CELL_SIZE, GRID_HEIGHT, GRID_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::core::map::Map;
use super::super::resources::EditorData; 

pub fn render_editor_path(editor_data: Res<EditorData>, mut gizmos: Gizmos, map: Option<Res<Map>>) {
    let (grid_size, dimensions) = if let Some(map_res) = map.as_ref() {
        (map_res.grid_size, map_res.dimensions)
    } else {
        (
            Vec2::new(CELL_SIZE, CELL_SIZE),
            UVec2::new(GRID_WIDTH as u32, GRID_HEIGHT as u32),
        )
    };

    let grid_to_world = |grid_pos: UVec2| -> Vec2 {
        if let Some(map_res) = map.as_ref() {
            map_res.grid_to_world(grid_pos)
        } else {
            let grid_start_x = -WINDOW_WIDTH / 2.0 + grid_size.x / 2.0;
            let grid_start_y = WINDOW_HEIGHT / 2.0 - grid_size.y / 2.0;
            Vec2::new(
                grid_start_x + grid_pos.x as f32 * grid_size.x,
                grid_start_y - grid_pos.y as f32 * grid_size.y,
            )
        }
    };


    if editor_data.path.len() >= 2 {
        let path_points: Vec<Vec2> = editor_data
            .path
            .iter()
            .map(|(grid_pos, _)| grid_to_world(*grid_pos))
            .collect();

        gizmos.linestrip_2d(path_points, Color::srgb(0.9, 0.3, 0.7));
    }

    if editor_data.grid_overlay {
        let grid_world_left = -WINDOW_WIDTH / 2.0;
        let grid_world_top = WINDOW_HEIGHT / 2.0;
        let grid_world_width = dimensions.x as f32 * grid_size.x;
        let grid_world_height = dimensions.y as f32 * grid_size.y;
        let grid_world_right = grid_world_left + grid_world_width;
        let grid_world_bottom = grid_world_top - grid_world_height; 

        for x_index in 0..=dimensions.x {
            let x_world = grid_world_left + x_index as f32 * grid_size.x;
            gizmos.line_2d(
                Vec2::new(x_world, grid_world_top),
                Vec2::new(x_world, grid_world_bottom),
                Color::srgba(0.5, 0.5, 0.5, 0.5), 
            );
        }

        for y_index in 0..=dimensions.y {
            let y_world = grid_world_top - y_index as f32 * grid_size.y;
            gizmos.line_2d(
                Vec2::new(grid_world_left, y_world),
                Vec2::new(grid_world_right, y_world),
                Color::srgba(0.5, 0.5, 0.5, 0.5), 
            );
        }
    }
}
