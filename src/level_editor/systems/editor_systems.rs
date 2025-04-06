use crate::core::config::{CELL_SIZE, GRID_HEIGHT, GRID_WIDTH};
use crate::core::map::Map;
use bevy::prelude::*;

pub fn setup_editor_mode(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map = Map::new();

    for y in 0..map.dimensions.y {
        for x in 0..map.dimensions.x {
            let world_pos = map.grid_to_world(UVec2::new(x, y));
            
            commands.spawn((
                Sprite {
                    image: asset_server.load("textures/grass.png"),
                    custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(world_pos.x, world_pos.y, 0.0)),
            ));
        }
    }
    commands.insert_resource(map);
}
