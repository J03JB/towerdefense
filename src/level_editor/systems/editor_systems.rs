use crate::core::map::Map;
use bevy::prelude::*;

pub fn setup_editor_mode(mut commands: Commands) {
    let map = Map::new();


    let map_width = map.dimensions.x as usize;
    let map_height = map.dimensions.y as usize;

    commands.insert_resource(map);
}
