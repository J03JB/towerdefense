use crate::core::game_state::GameState;
use crate::core::map::Map;
use crate::core::utils::distance;
use crate::entities::pathfinding::{FlowField, FlowDirection};
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemies)
            .add_systems(Update, (
                move_enemies_along_path, 
                check_enemy_health, 
                handle_enemies_at_end)
                .run_if(in_state(GameState::Playing))
            );
    
    }
    
}

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
    pub reward: u32,
    pub enemy_type: EnemyType,
    pub path_index: usize,  // Current position on the path
    pub path_progress: f32, // Progress between current and next path point (0.0 to 1.0)
}

#[derive(Debug, Clone, Copy)]
pub enum EnemyType {
    Basic,
    Fast,
    Tank,
    Boss,
}

fn setup_enemies() {}

/// Spawns an enemy at the start of the path
pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    map: &Res<Map>,
    enemy_type: EnemyType,
) {
    // Get start position from the map
    let start_pos = map.grid_to_world(map.start);

    // Determine enemy properties based on type
    let (health, speed, reward) = match enemy_type {
        EnemyType::Basic => (100.0, 0.01, 10),
        EnemyType::Fast => (50.0, 100.0, 15),
        EnemyType::Tank => (200.0, 30.0, 20),
        EnemyType::Boss => (500.0, 40.0, 50),
    };

    // Load appropriate sprite based on enemy type
    let texture_path = match enemy_type {
        EnemyType::Basic => "enemies/basic_enemy.png",
        EnemyType::Fast => "enemies/fast_enemy.png",
        EnemyType::Tank => "enemies/tank_enemy.png",
        EnemyType::Boss => "enemies/boss_enemy.png",
    };

    // Spawn the enemy entity
    commands.spawn((
        Sprite {
            image: asset_server.load(texture_path),
            ..default()
        },
        Transform::from_translation(Vec3::new(start_pos.x, start_pos.y, 1.0)),
        Enemy {
            health,
            speed,
            reward,
            enemy_type,
            path_index: 0, // Start at the beginning of the path
            path_progress: 0.0,
        },
    ));
}

fn move_enemies_along_path(
    time: Res<Time>,
    map: Res<Map>,
    flow_field: Option<Res<FlowField>>,
    mut enemies: Query<(&mut Transform, &mut Enemy)>,
) {
    let delta = time.delta_secs();

    // If we don't have a flow field yet, return
    let Some(flow_field) = flow_field.as_ref() else {
        return;
    };

    if !flow_field.is_initialized {
        return;
    }

    for (mut transform, mut enemy) in enemies.iter_mut() {
        // Get the current world position
        let current_pos = Vec2::new(transform.translation.x, transform.translation.y);

        // Convert to grid coordinates
        let grid_pos = map.world_to_grid(current_pos);
        
        // Check grid bounds
        if grid_pos.x as usize >= flow_field.width || grid_pos.y as usize >= flow_field.height {
            continue;
        }

        // Get flow direction at current grid position
        let flow_direction = flow_field.get_direction(grid_pos.x as usize, grid_pos.y as usize);

        // Get the next grid position and its world center
        let next_grid_pos = match flow_direction {
            Some(FlowDirection::North) => UVec2::new(grid_pos.x, grid_pos.y - 1),
            Some(FlowDirection::South) => UVec2::new(grid_pos.x, grid_pos.y + 1),
            Some(FlowDirection::East) => UVec2::new(grid_pos.x + 1, grid_pos.y),
            Some(FlowDirection::West) => UVec2::new(grid_pos.x - 1, grid_pos.y),
            Some(FlowDirection::None) | None => grid_pos,
        };

        let next_world_pos = map.grid_to_world(next_grid_pos);
        
        // Calculate movement to fully cover the grid cell
        let precise_movement = match flow_direction {
            Some(FlowDirection::North) => {
                // Move fully up with guaranteed coverage
                Vec2::new(0.0, map.grid_size.y + enemy.speed * 0.01)
            },
            Some(FlowDirection::South) => {
                // Move fully down
                Vec2::new(0.0, -map.grid_size.y - enemy.speed * 0.01)
            },
            Some(FlowDirection::East) => {
                // Move fully right
                Vec2::new(map.grid_size.x + enemy.speed * 0.01, 0.0)
            },
            Some(FlowDirection::West) => {
                // Move fully left
                Vec2::new(-map.grid_size.x - enemy.speed * 0.01, 0.0)
            },
            Some(FlowDirection::None) | None => Vec2::ZERO,
        };

        // Update position with precise movement
        transform.translation.x += precise_movement.x;
        transform.translation.y += precise_movement.y;

        // Ensure exact positioning
        transform.translation.x = next_world_pos.x;
        transform.translation.y = next_world_pos.y;

        // Logging for debugging
        // info!(
        //     "Current Grid: {:?}, Flow Direction: {:?}, Next Grid: {:?}, World Movement: {:?}",
        //     grid_pos, flow_direction, next_grid_pos, precise_movement
        // );

        // Rotate enemy to face movement direction
        if precise_movement != Vec2::ZERO {
            let angle = precise_movement.y.atan2(precise_movement.x);
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

// fn move_enemies_along_path(
//     time: Res<Time>,
//     map: Res<Map>,
//     flow_field: Option<Res<FlowField>>,
//     mut enemies: Query<(&mut Transform, &mut Enemy)>,
// ) {
//     let delta = time.delta_secs();
//
//     // If we don't have a flow field yet, return
//     let Some(flow_field) = flow_field.as_ref() else {
//         info!("No flow field available");
//         return;
//     };
//
//     if !flow_field.is_initialized {
//         info!("Flow field not initialized yet");
//         return;
//     }
//
//     for (mut transform, mut enemy) in enemies.iter_mut() {
//         // Get the current world position
//         let current_pos = Vec2::new(transform.translation.x, transform.translation.y);
//
//         // Convert to grid coordinates
//         let grid_pos = map.world_to_grid(current_pos);
//
//         // Debug info
//         info!("Enemy at grid position: {:?}", grid_pos);
//
//         // Check if the position is on a valid path tile
//         if !map.path_tiles.contains(&grid_pos) {
//             info!("Enemy not on a path tile!");
//         }
//
//         // Get flow direction at current grid position
//         if grid_pos.x as usize >= flow_field.width || grid_pos.y as usize >= flow_field.height {
//             info!("Grid position out of flow field bounds!");
//             continue;
//         }
//
//         let direction = flow_field.get_direction(grid_pos.x as usize, grid_pos.y as usize);
//         // match direction {
//         //     Some(dir) => info!("Flow direction at {:?}: {:?}", grid_pos, dir),
//         //     None => info!("No flow direction at grid position: {:?}", grid_pos),
//         // }
//
//         let flow_vector = flow_field.get_flow_vector(grid_pos.x as usize, grid_pos.y as usize);
//
//         // If we have a valid flow direction, move along it
//         if flow_vector != Vec2::ZERO {
//             // Calculate movement
//             let movement = flow_vector * enemy.speed * delta;
//             // info!("Moving enemy by: {:?}", movement);
//
//             // Update position
//             transform.translation.x += movement.x;
//             transform.translation.y += movement.y;
//
//             // Optionally rotate enemy to face movement direction
//             if flow_vector != Vec2::ZERO {
//                 let angle = flow_vector.y.atan2(flow_vector.x);
//                 transform.rotation = Quat::from_rotation_z(angle);
//             }
//         } else {
//             info!("No movement - zero flow vector at {:?}", grid_pos);
//         }
//     }
// }


fn check_enemy_health(
    mut commands: Commands,
    mut enemies: Query<(Entity, &Enemy)>,
    mut game_resources: Option<ResMut<crate::core::game_state::PlayerResource>>,
) {
    for (entity, enemy) in enemies.iter() {
        // Check if enemy is dead
        if enemy.health <= 0.0 {
            // Despawn the enemy
            commands.entity(entity).despawn();

            // Award money/score to player if resources exist
            if let Some(mut resources) = game_resources.as_mut() {
                resources.money += enemy.reward;
                resources.score += enemy.reward;
            }
        }

        // Note: Enemy reaching the end and damaging player lives
        // is handled in a separate system
    }
}
// Make sure this function is properly implemented
fn handle_enemies_at_end(
    mut commands: Commands,
    map: Res<Map>,
    enemies: Query<(Entity, &Transform, &Enemy)>,
    mut game_resources: Option<ResMut<crate::core::game_state::PlayerResource>>,
) {
    let end_pos = map.grid_to_world(map.end);

    for (entity, transform, _enemy) in enemies.iter() {
        let enemy_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let distance_to_end = enemy_pos.distance(end_pos);

        // If enemy is close enough to end point (adjust this value if needed)
        if distance_to_end < 30.0 {
            info!("Enemy reached the end at: {:?}", end_pos);
            
            // Despawn the enemy
            commands.entity(entity).despawn();

            // Reduce player lives if resources exist
            if let Some(mut resources) = game_resources.as_mut() {
                resources.lives = resources.lives.saturating_sub(1);
                info!("Player lives remaining: {}", resources.lives);
            }
        }
    }
}

// pub fn handle_enemies_at_end(
//     mut commands: Commands,
//     map: Res<Map>,
//     enemies: Query<(Entity, &Transform, &Enemy)>,
//     mut game_resources: Option<ResMut<crate::core::game_state::PlayerResource>>,
// ) {
//     let end_pos = map.grid_to_world(map.end);
//
//     for (entity, transform, _enemy) in enemies.iter() {
//         let enemy_pos = Vec2::new(transform.translation.x, transform.translation.y);
//         let distance_to_end = distance(enemy_pos, end_pos);
//
//         // If enemy is close enough to end point
//         if distance_to_end < 10.0 {
//             // Despawn the enemy
//             commands.entity(entity).despawn();
//
//             // Reduce player lives if resources exist
//             if let Some(mut resources) = game_resources.as_mut() {
//                 resources.lives = resources.lives.saturating_sub(1);
//             }
//         }
//     }
// }

// System for enemy wave spawning
pub fn spawn_enemy_wave(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Res<Map>,
    wave_data: Vec<(EnemyType, u32)>, // (enemy_type, count)
    spawn_interval: f32,
    time: Res<Time>,
) -> impl FnMut() {
    // Create a flat vec of all enemies to spawn
    let mut enemies_to_spawn = Vec::new();
    for (enemy_type, count) in wave_data {
        for _ in 0..count {
            enemies_to_spawn.push(enemy_type);
        }
    }

    let mut spawn_timer = Timer::from_seconds(spawn_interval, TimerMode::Repeating);
    let mut current_index = 0;


        move || {
            if current_index >= enemies_to_spawn.len() {
                // Wave complete
                info!("wave completed");
                // return true;
            }

        spawn_timer.tick(time.delta());

        if spawn_timer.just_finished() {
            let enemy_type = enemies_to_spawn[current_index];
            spawn_enemy(&mut commands, &asset_server, &map, enemy_type);
            current_index += 1;
        }

        info!("wave not complete");
        // false // Wave not complete yet
    }
}
