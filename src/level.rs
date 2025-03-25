use crate::enemy::EnemyType;
use serde::{Deserialize, Serialize};
// use crate::map::Map;
use crate::tower::{Tower, TowerType};
// use crate::map::*;

use bevy::prelude::*;

pub struct LevelPlugin;

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
pub const GRID_WIDTH: usize = 27; 
pub const GRID_HEIGHT: usize = 15;
pub const CELL_SIZE: f32 = 48.0;
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

#[derive(Component)]
pub struct GridCell {
    pub x: usize,
    pub y: usize,
}

fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    let grid_start_x = -WINDOW_WIDTH / 2.0 + CELL_SIZE / 2.0;
    let grid_start_y = WINDOW_HEIGHT / 2.0 - CELL_SIZE / 2.0;

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let position = Vec2::new(
                grid_start_x + x as f32 * CELL_SIZE,
                grid_start_y - y as f32 * CELL_SIZE,
            );

            let texture_handle = asset_server.load("grass_test.png");
            commands.spawn((
                Sprite {
                    image: texture_handle,
                    ..default()
                },
                    Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
                GridCell { x, y },
            ));
        }
    }
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


