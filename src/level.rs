use crate::enemy::EnemyType;
use crate::map::Map;
use crate::tower::{Tower, TowerType};
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_level)
            .add_systems(Update, check_wave_progress);
    }
}

#[derive(Resource)]
pub struct Level {
    pub current_level: u32,
    pub path: Vec<Vec2>,
    pub waves: Vec<Wave>,
    pub current_wave: usize,
}

pub struct Wave {
    pub enemy_counts: Vec<(EnemyType, u32)>,
    pub spawn_interval: f32,
    pub wave_delay: f32,
}

fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Define map dimensions and path
    // let grid_size = Vec2::new(64.0, 60.0);
    let map = Map {
        grid_size: Vec2::new(64.0, 60.0),
        dimensions: UVec2::new(20, 15),
        path_tiles: vec![UVec2::new(0, 5), UVec2::new(1, 5) /* ... path tiles */],
        buildable_tiles: vec![/* ... */],
        start: UVec2::new(0, 5),
        end: UVec2::new(19, 5),
    };

    // Spawn the background grass texture tiled across the map
    for y in 0..map.dimensions.y {
        for x in 0..map.dimensions.x {
            let world_pos = map.grid_to_world(UVec2::new(x, y));

            // Spawn grass background tile
            commands.spawn((
                Sprite {
                    image: asset_server.load("grass_background.png"),
                    ..default()
                },
                Transform::from_translation(world_pos.extend(0.0)),
            ));
        }
    }
    // Generate buildable tiles by excluding path tiles
    // ...

    // Spawn map tiles visually
    for y in 0..map.dimensions.y {
        for x in 0..map.dimensions.x {
            let pos = UVec2::new(x, y);
            let world_pos = map.grid_to_world(pos);

            let tile_type = if map.path_tiles.contains(&pos) {
                "path_tile"
            } else if map.buildable_tiles.contains(&pos) {
                "buildable_tile"
            } else {
                "blocked_tile"
            };

            // Spawn the appropriate tile sprite
            // commands.spawn((
            //     Sprite {
            //     image: asset_server.load(format!("{}.png", tile_type)),
            //     ..default()
            // },
            //     Transform::from_translation(world_pos.extend(0.0)),
            // ));
        }
    }

    commands.insert_resource(map);
}
fn check_wave_progress() {
    // Check if current wave is complete and spawn next wave
}

fn place_tower(
    commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    map: Res<Map>,
    asset_server: Res<AssetServer>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        // Get cursor position in world space
        let (camera, camera_transform) = camera_q.single();
        let window = windows.single();

        //er if let Some(world_position) = window.cursor_position()
        //     .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor)))
        //     // .map(|ray| ray.truncate())
        // {
        //     // Convert to grid position
        //     let grid_pos = map.world_to_grid(world_position);
        //
        //     // Check if buildable
        //     if map.is_buildable(grid_pos) {
        //         // Spawn tower at grid position
        //         let world_pos = map.grid_to_world(grid_pos);
        //
        //         let texture = asset_server.load("tower.png");
        //         commands.spawn((Sprite {
        //             image: texture.clone(),
        //             ..default()
        //         },
        //
        //             Transform::from_translation(world_pos.extend(10.0)),
        //             Tower {
        //                 tower_type: TowerType::Basic,
        //                 range: 150.0,
        //                 damage: 10.0,
        //                 fire_rate: 1.0,
        //                 last_shot: 0.0,
        //                 level: 1,
        //                 target: None,
        //             }
        //         ));
        //     }
        // }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LevelData {
    pub path: Vec<Vec2>,
    pub waves: Vec<WaveData>,
    // other level data
}
