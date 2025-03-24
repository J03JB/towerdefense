use crate::enemy::EnemyType;
use serde::{Deserialize, Serialize};
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
    let level_json = std::fs::read_to_string("assets/level_01.tmj")
        .expect("Failed to read level file");
    
    let level_data: TiledLevel = serde_json::from_str(&level_json)
        .expect("Failed to parse level JSON");

    let path_tiles = level_data.parse_path();

  // Use info! or debug! instead of println
    info!("Attempting to load level");
    
    // match std::fs::read_to_string("assets/level_01.tmj") {
    //     Ok(level_json) => {
    //         match serde_json::from_str::<TiledLevel>(&level_json) {
    //             Ok(level_data) => {
    //                 info!("Map dimensions: {}x{}", level_data.width, level_data.height);
    //                 info!0("Tile size: {}x{}", level_data.tilewidth, level_data.tileheight);
    //                 info!("First layer data length: {}", level_data.layers[0].data.len());
    //                 info!("path: {:?}", path_tiles);
    //
    //                 // Rest of your existing code
    //             },
    //             Err(e) => error!("Failed to parse level JSON: {}", e)
    //         }
    //     },
    //     Err(e) => error!("Failed to read level file: {}", e)
    // }
  // Hardcoded grid parameters based on your Tiled map
    let tile_width = 40.0; // from your Tiled map
    let tile_height = 23.0;
    
    let mut map = Map {
        grid_size: Vec2::new(tile_width, tile_height),
        dimensions: UVec2::new(level_data.width, level_data.height),
        path_tiles,
        buildable_tiles: vec![/* ... */],
        start: UVec2::new(0, 5),
        end: UVec2::new(19, 5),
        offset: Vec2::ZERO,
    };

    info!("level width: {}", level_data.width);
    info!("level height: {}", level_data.height);
    // Load path tile texture
    // let path_texture = asset_server.load("path.png");

    let offset = Vec2::new(
        (map.dimensions.x as f32 * map.grid_size.x)  / 2.0,
        (map.dimensions.y as f32 * map.grid_size.y)  / 2.0,
            );
    map.offset = offset;

    // Spawn path tiles with manual positioning
    // for tile_pos in &map.path_tiles {
    //     let world_x = tile_pos.x as f32 * tile_width;
    //     let world_y = tile_pos.y as f32 * tile_height;
    //
    //     commands.spawn((Sprite {
    //         image: path_texture.clone(),
    //         ..default()
    //     },
    //
    //         Transform::from_xyz(
    //             world_x + tile_width / 2.0, 
    //             world_y + tile_height / 2.0, 
    //             0.0
    //         ),
    //     ));
    // }

    // let map = Map {
    //     grid_size: Vec2::new(
    //     level_data.tilewidth as f32,
    //     level_data.tileheight as f32
    //     ),
    //     dimensions: UVec2::new(level_data.width, level_data.height),
    //     path_tiles,
    //     // path_tiles: vec![UVec2::new(0, 5), UVec2::new(1, 5) /* ... path tiles */],
    //     buildable_tiles: vec![/* ... */],
    //     start: UVec2::new(0, 5),
    //     end: UVec2::new(19, 5),
    // };

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
                Transform::from_xyz(0.0, 0.0,  -1.0)
                 .with_scale(Vec3::splat(2.0)),
            ));
        }
    }
    // Generate buildable tiles by excluding path tiles
    // ...

    let path_texture = asset_server.load("path.png");
    for tile_pos in &map.path_tiles {
        let world_pos = map.grid_to_world(*tile_pos);

        commands.spawn((Sprite {
            image: path_texture.clone(),
            ..default()
        },
            Transform::from_xyz(world_pos.x, world_pos.y, 0.0),
        ));

    }    // Spawn map tiles visually
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

            // commands.spawn(Sprite {
            //     image: asset_server.load("path.png"),
            //     ..default()
            // });
    //
    //
    //         // Spawn the appropriate tile sprite
    //         // commands.spawn((
    //         //     Sprite {
    //         //     image: asset_server.load(format!("{}.png", tile_type)),
    //         //     ..default()
    //         // },
    //         //     Transform::from_translation(world_pos.extend(0.0)),
    //         // ));
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


#[derive(Deserialize)]
pub struct TiledLevel {
    pub height: u32,
    pub width: u32,
    pub layers: Vec<TiledLayer>,
    pub tilewidth: u32,
    pub tileheight: u32,
}

#[derive(Deserialize)]
pub struct TiledLayer {
    pub data: Vec<u32>,
    pub height: u32,
    pub width: u32,
}

impl TiledLevel {
    pub fn parse_path(&self) -> Vec<UVec2> {
        let mut path_tiles = Vec::new();
        
        for (index, &tile) in self.layers[0].data.iter().enumerate() {
            if tile != 0 {
                let x = (index % self.width as usize) as u32;
                let y = (index / self.width as usize) as u32;
                path_tiles.push(UVec2::new(x, y));
            }
        }
        
        path_tiles
    }
}


