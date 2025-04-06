use crate::core::config::{CELL_SIZE, GRID_HEIGHT, GRID_WIDTH};
use crate::core::map::Map;
use crate::core::game_state::GameState;
use crate::entities::enemy::{EnemyType, spawn_enemy};
use crate::entities::pathfinding::{FlowDirection, FlowField};
use crate::levels::level_textures::PathTexture;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_level.run_if(not(in_state(GameState::Editor))))
            .add_systems(Update, spawn_wave_system.run_if(in_state(GameState::Playing)))
            // .add_systems(Update, check_wave_progress)
            .add_event::<WaveCompleteEvent>();
    }
}

#[derive(Resource)]
pub struct Level {
    pub current_level: u32,
    pub waves: Vec<Wave>,
    pub current_wave_index: usize,
    pub wave_in_progress: bool,
    pub spawn_timer: Timer,
    pub enemies_to_spawn: Vec<EnemyType>,
    pub enemies_spawned: usize,
    pub enemies_remaining: usize,
}

pub struct Wave {
    pub enemy_types: Vec<(EnemyType, u32)>, // (type, count)
    pub spawn_interval: f32,                // Time between enemy spawns
    pub wave_delay: f32,                    // Delay before wave starts
}

#[derive(Event)]
pub struct WaveCompleteEvent {
    pub wave_index: usize,
}

#[derive(Resource, Clone, Deserialize)]
pub struct LevelData {
    pub path: Vec<Vec<u32>>, // Stored as [[x, y], [x, y], ...]
    pub path_textures: Vec<PathTexture>,
    pub start: Vec<u32>,                // [x, y]
    pub end: Vec<u32>,                  // [x, y]
    pub buildable_areas: Vec<Vec<u32>>, // [[x, y], [x, y], ...]
    pub dimensions: Vec<u32>,           // [width, height]
}

fn create_map() -> Map {
    let mut path_tiles = Vec::new();
    let mut x = 0;
    let mut y = 10;
    path_tiles.push(UVec2::new(x, y));
    while x < 15 {
        x += 1;
        path_tiles.push(UVec2::new(x, y));
    }
    // Up to (15, 5)
    while y > 5 {
        y -= 1;
        path_tiles.push(UVec2::new(x, y));
    }
    // Right to (30, 5)
    while x < 27 {
        x += 1;
        path_tiles.push(UVec2::new(x, y));
    }
    // Create buildable tiles (all tiles except path and borders)
    let mut buildable_tiles = Vec::new();
    for y_pos in 0..20 {
        for x_pos in 0..40 {
            let pos = UVec2::new(x_pos, y_pos);
            // Skip path tiles and 1-tile buffer around path
            if !is_near_path(&path_tiles, pos, 0) {
                buildable_tiles.push(pos);
            }
        }
    }

    Map {
        grid_size: Vec2::new(CELL_SIZE, CELL_SIZE),
        dimensions: UVec2::new(GRID_WIDTH as u32, GRID_HEIGHT as u32),
        path_tiles,
        buildable_tiles,
        start: UVec2::new(0, 10), // Start at the beginning of the path
        end: UVec2::new(26, 8),  // End at the end of the path
    }
}

// Helper to determine if a position is near the path (within buffer distance)
fn is_near_path(path: &[UVec2], pos: UVec2, buffer: u32) -> bool {
    for &path_pos in path {
        let dx = if path_pos.x > pos.x {
            path_pos.x - pos.x
        } else {
            pos.x - path_pos.x
        };
        let dy = if path_pos.y > pos.y {
            path_pos.y - pos.y
        } else {
            pos.y - path_pos.y
        };

        if dx <= buffer && dy <= buffer {
            return true;
        }
    }
    false
}

fn create_waves() -> Vec<Wave> {
    vec![
        // Wave 1: Basic enemies
        Wave {
            enemy_types: vec![(EnemyType::Basic, 10)],
            spawn_interval: 1.0,
            wave_delay: 3.0,
        },
        // Wave 2: Basic and Fast enemies
        Wave {
            enemy_types: vec![(EnemyType::Basic, 8), (EnemyType::Fast, 5)],
            spawn_interval: 0.8,
            wave_delay: 5.0,
        },
        // Wave 3: Basic, Fast, and Tank enemies
        Wave {
            enemy_types: vec![
                (EnemyType::Basic, 10),
                (EnemyType::Fast, 8),
                (EnemyType::Tank, 3),
            ],
            spawn_interval: 0.7,
            wave_delay: 7.0,
        },
        // Wave 4: All enemy types including Boss
        Wave {
            enemy_types: vec![
                (EnemyType::Basic, 15),
                (EnemyType::Fast, 10),
                (EnemyType::Tank, 5),
                (EnemyType::Boss, 1),
            ],
            spawn_interval: 0.5,
            wave_delay: 10.0,
        },
    ]
}

// New function to spawn map visuals with textures
fn spawn_map_visuals_with_textures(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    map: &Map,
    level_data: Option<LevelData>,
) {
    // Spawn background for the entire map
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

    // Create a map of positions to textures
    let texture_map: std::collections::HashMap<UVec2, String> = if let Some(data) = level_data {
        data.path_textures
            .iter()
            .filter(|pt| pt.position.len() >= 2)
            .map(|pt| {
                (
                    UVec2::new(pt.position[0], pt.position[1]),
                    pt.texture.clone(),
                )
            })
            .collect()
    } else {
        std::collections::HashMap::new()
    };

    // Spawn path tiles with appropriate textures
    for &pos in &map.path_tiles {
        let world_pos = map.grid_to_world(pos);

        // Get texture for this position or use default
        let texture_path = texture_map
            .get(&pos)
            .cloned()
            .unwrap_or_else(|| "textures/path01.png".to_string());

        commands.spawn((
            Sprite {
                image: asset_server.load(&texture_path),
                custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                ..default()
            },
            Transform::from_translation(Vec3::new(world_pos.x, world_pos.y, 0.1)),
        ));
    }

    // Add visual indicators for start and end points
    let start_pos = map.grid_to_world(map.start);
    let end_pos = map.grid_to_world(map.end);

    // Start portal
    commands.spawn((
        Sprite {
            image: asset_server.load("textures/start_portal.png"),
            custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
            ..default()
        },
        Transform::from_translation(Vec3::new(start_pos.x, start_pos.y, 0.2)),
    ));

    // End portal
    commands.spawn((
        Sprite {
            image: asset_server.load("textures/end_portal.png"),
            custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
            ..default()
        },
        Transform::from_translation(Vec3::new(end_pos.x, end_pos.y, 0.2)),
    ));
}

// Update the setup_level function in level.rs
fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level_data_result = std::fs::read_to_string("assets/levels/level-test.json")
        .map_err(|e| format!("Error reading level file: {}", e))
        .and_then(|json_str| {
            serde_json::from_str::<LevelData>(&json_str)
                .map_err(|e| format!("Error parsing JSON: {}", e))
        });

    let map = if let Ok(level_data) = &level_data_result {
        Map {
            grid_size: Vec2::new(CELL_SIZE, CELL_SIZE),
            dimensions: UVec2::new(level_data.dimensions[0], level_data.dimensions[1]),
            path_tiles: level_data
                .path
                .iter()
                .map(|coords| UVec2::new(coords[0], coords[1]))
                .collect(),
            buildable_tiles: level_data
                .buildable_areas
                .iter()
                .map(|coords| UVec2::new(coords[0], coords[1]))
                .collect(),
            start: UVec2::new(level_data.start[0], level_data.start[1]),
            end: UVec2::new(level_data.end[0], level_data.end[1]),
        }
    } else {
        info!("Failed to load level data, using fallback map");
        create_map()
    };

    // info!("buildable tiles: {:?}", map.buildable_tiles);
    // info!("path tiles: {:?}", map.path_tiles);

    let map_width = map.dimensions.x as usize;
    let map_height = map.dimensions.y as usize;
    
    let goal_x = map.end.x.min(map_width as u32 - 1);
    let goal_y = map.end.y.min(map_height as u32 - 1);
    let goal_pos = UVec2::new(goal_x, goal_y);
    
    let mut flow_field = FlowField::new(map_width, map_height);
    flow_field.compute(&map, goal_pos);
    
    info!("Flow field initialized with goal at: {:?}", goal_pos);
    info!("Start position is: {:?}", map.start);
    info!("Map has {} path tiles", map.path_tiles.len());

    commands.insert_resource(flow_field);

    spawn_map_visuals_with_textures(&mut commands, &asset_server, &map, level_data_result.ok());

    commands.insert_resource(map);
}

fn spawn_wave_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Res<Map>,
    flow_field: Option<Res<FlowField>>,
    time: Res<Time>,
    enemies: Query<&crate::entities::enemy::Enemy>,
) {
    if enemies.is_empty() && flow_field.is_some() && flow_field.as_ref().unwrap().is_initialized {
        let start_pos = map.grid_to_world(map.start);
        
        commands.spawn((
            Sprite {
                image: asset_server.load("textures/enemies/enemy.png"),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(start_pos.x, start_pos.y, 10.0)),
            crate::entities::enemy::Enemy {
                health: 100.0,
                speed: 200.0,
                reward: 10,
                enemy_type: EnemyType::Basic,
                path_index: 0,
                path_progress: 0.0,
            },
        ));
        
        info!("Spawned a basic enemy at: {:?}", start_pos);
    }
}


fn check_wave_progress(
    mut level: ResMut<Level>,
    enemies: Query<&crate::entities::enemy::Enemy>,
    mut wave_complete_events: EventWriter<WaveCompleteEvent>,
    game_state: Option<State<crate::core::game_state::GameState>>,
) {
    // Skip if game isn't in Playing state
    if let Some(state) = game_state {
        if *state != crate::core::game_state::GameState::Playing {
            return;
        }
    }

    // If wave is in progress and all enemies are spawned
    if level.wave_in_progress && level.enemies_spawned >= level.enemies_to_spawn.len() {
        // Count remaining enemies
        let remaining = enemies.iter().count();
        level.enemies_remaining = remaining;

        // If no enemies remain, wave is complete
        if remaining == 0 {
            info!("Wave {} completed!", level.current_wave_index + 1);

            // Send wave complete event
            wave_complete_events.send(WaveCompleteEvent {
                wave_index: level.current_wave_index,
            });

            // Set up for next wave
            level.current_wave_index += 1;
            level.wave_in_progress = false;

            // If there are more waves, prepare the next one
            if level.current_wave_index < level.waves.len() {
                let next_wave = &level.waves[level.current_wave_index];

                // Prepare enemies for next wave
                let mut enemies_to_spawn = Vec::new();
                for (enemy_type, count) in &next_wave.enemy_types {
                    for _ in 0..*count {
                        enemies_to_spawn.push(*enemy_type);
                    }
                }

                // Set up timer for wave delay
                // level.spawn_timer = Timer::from_seconds(next_wave.wave_delay, TimerMode::Once);
                // level.enemies_to_spawn = enemies_to_spawn;
                // level.enemies_spawned = 0;
                // level.enemies_remaining = enemies_to_spawn.len();

                info!("Next wave starting in {} seconds", next_wave.wave_delay);
            } else {
                // All waves completed
                info!("All waves completed! Level finished!");
                // You could trigger a level complete event here
            }
        }
    }
}
