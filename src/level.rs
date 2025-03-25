use crate::enemy::EnemyType;
use serde::{Deserialize, Serialize};
// use crate::map::Map;
use crate::tower::{Tower, TowerType};
use crate::config::{GRID_WIDTH, CELL_SIZE, GRID_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::map::{Map, GridCell};

use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_level);
            // .add_systems(Update, cast_cursor_ray);
            // .add_systems(Update, check_wave_progress);
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
    let mut map = Map::new();
    
    for x in 0..GRID_WIDTH {
        let path_pos = UVec2::new(x as u32, GRID_HEIGHT as u32 / 2);
        map.path_tiles.push(path_pos);
    }
    
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let pos = UVec2::new(x as u32, y as u32);
            if !map.path_tiles.contains(&pos) {
                map.buildable_tiles.push(pos);
            }
        }
    }

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let pos = UVec2::new(x as u32, y as u32);
            let world_pos = map.grid_to_world(pos);
            
            // Determine tile type
            let (texture_handle, zvalue) = if map.path_tiles.contains(&pos) {
                (asset_server.load("path.png"), 1.0)
            } else if map.buildable_tiles.contains(&pos) {
                (asset_server.load("grass.png"), -1.0)
            } else {
                (asset_server.load("blocked.png"), 1.0)
            };
            
            commands.spawn((
                Sprite {
                    image: texture_handle,
                    custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(world_pos.x, world_pos.y, zvalue)),
                GridCell { x, y },
            ));
        }
    }
    info!("Window size: {}x{}", WINDOW_WIDTH, WINDOW_HEIGHT);
    info!("Grid dimensions: {}x{} cells", GRID_WIDTH, GRID_HEIGHT);
    info!("Total grid size: {}x{} pixels", GRID_WIDTH as f32 * CELL_SIZE, GRID_HEIGHT as f32 * CELL_SIZE);

    commands.insert_resource(map);
}

fn check_wave_progress() {
    // Check if current wave is complete and spawn next wave
}

fn cast_cursor_ray(
  windows: Query<&Window>,
  cameras: Query<(&Camera, &GlobalTransform)>,
) {
  let window = windows.single();
  let (camera, position) = cameras.single();

  if let Some(world_position) = window
    .cursor_position()
    .map(|cursor| camera.viewport_to_world(position, cursor))
    .map(|ray| ray.unwrap().origin.truncate())
  {
    info!("World coords: {}/{}", world_position.x, world_position.y);
  }
}

fn place_tower(
    commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    // map: Res<Map>,
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
        //             Transform::from_translation(world_pos.extend(11.0)),
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


