use crate::core::game_state::GameState;
use crate::core::map::Map;
use crate::core::utils::distance;
use crate::entities::pathfinding::{FlowField, FlowDirection};
use bevy::prelude::*;
use bevy::math::Vec3Swizzles;

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
    mut enemies: Query<(&mut Transform, &Enemy)>, // Keep Enemy non-mut if only reading speed
) {
    let delta = time.delta_secs();

    let Some(flow_field) = flow_field else { return; };
    if !flow_field.is_initialized { return; }

    for (mut transform, enemy) in enemies.iter_mut() {
        let current_pos_world = transform.translation.xy();

        // 1. Find current grid cell
        let current_grid_pos = map.world_to_grid(current_pos_world);

        // --- Safety Check: Ensure grid_pos is within bounds ---
        if current_grid_pos.x >= flow_field.width as u32 || current_grid_pos.y >= flow_field.height as u32 {
             warn!(
                "Enemy at {:?} (world {:?}) is outside flow field bounds ({}, {}). Stopping.",
                current_grid_pos, current_pos_world, flow_field.width, flow_field.height
            );
             continue;
        }
        // --- End Safety Check ---

        // 2. Get flow direction *from the current cell*
        // We use get_direction here, not get_flow_vector, to determine the *next cell*
        let flow_direction_enum = flow_field.get_direction(current_grid_pos.x as usize, current_grid_pos.y as usize);

        // If no direction (e.g., at goal or stuck), don't move
        let Some(direction) = flow_direction_enum else { continue; };
        if direction == FlowDirection::None { continue; };

        // 3. Determine the *next* grid cell based on the direction
        let next_grid_pos = match direction {
            FlowDirection::North => UVec2::new(current_grid_pos.x, current_grid_pos.y.saturating_sub(1)), // Grid Y decreases upwards
            FlowDirection::South => UVec2::new(current_grid_pos.x, (current_grid_pos.y + 1).min(flow_field.height as u32 - 1)), // Grid Y increases downwards
            FlowDirection::East => UVec2::new((current_grid_pos.x + 1).min(flow_field.width as u32 - 1), current_grid_pos.y),
            FlowDirection::West => UVec2::new(current_grid_pos.x.saturating_sub(1), current_grid_pos.y),
            FlowDirection::None => current_grid_pos, // Should not happen due to check above
        };

        // 4. Calculate the world position of the *center* of the next grid cell
        let target_pos_world = map.grid_to_world(next_grid_pos);

        // 5. Calculate the vector pointing from current position to the target center
        let direction_to_target = target_pos_world - current_pos_world;

        // 6. Calculate the distance we *can* move this frame
        let max_distance_this_frame = enemy.speed * delta;

        // 7. Calculate the actual movement vector
        let movement;
        if direction_to_target.length_squared() < max_distance_this_frame * max_distance_this_frame {
            // If we can reach the target center this frame, move exactly there
            movement = direction_to_target;
        } else {
            // Otherwise, move towards the target by the max distance
            movement = direction_to_target.normalize_or_zero() * max_distance_this_frame;
        }

        // 8. Apply the movement
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;

        // 9. Rotation (Optional - Face the actual movement direction)
        if movement != Vec2::ZERO {
            let angle = movement.y.atan2(movement.x);
            transform.rotation = Quat::from_rotation_z(angle);
        }

        // --- Debug Visualization (Optional) ---
        // use bevy::gizmos::gizmos::Gizmos;
        // fn debug_move(mut gizmos: Gizmos, query: Query<&Transform, With<Enemy>>) {
        //     for transform in query.iter() {
        //         gizmos.circle_2d(transform.translation.xy(), 5.0, Color::RED);
        //         // You could also draw the target_pos_world here if you pass it out or store it
        //     }
        // }
        // ------------------------------------
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
    }
}
// Make sure this function is properly implemented
fn handle_enemies_at_end(
    mut commands: Commands,
    map: Res<Map>,
    enemies: Query<(Entity, &Transform, &Enemy)>, // Enemy component not strictly needed here
    mut game_resources: Option<ResMut<crate::core::game_state::PlayerResource>>,
    time: Res<Time> // Add Time resource if you want to log frequency
) {
    // Calculate the end position ONCE outside the loop
    let end_pos_world = map.grid_to_world(map.end);

    // Define a threshold distance. This should likely be related to the grid size
    // or the enemy's speed/size. Maybe half a grid cell?
    let end_threshold = map.grid_size.x.min(map.grid_size.y) * 0.5; // Example threshold

    for (entity, transform, _enemy) in enemies.iter() {
        let enemy_pos_xy = transform.translation.xy();

        // Use distance_squared for efficiency if possible
        let distance_sq_to_end = enemy_pos_xy.distance_squared(end_pos_world);

        // If enemy is close enough to the end point
        if distance_sq_to_end < end_threshold * end_threshold {
            // Log only when it actually happens
            info!(tick = time.elapsed_secs_f64(), "Enemy {:?} reached the end (pos: {:?}, end: {:?}). Despawning.", entity, enemy_pos_xy, end_pos_world);

            // Despawn the enemy
            commands.entity(entity).despawn();

            // Reduce player lives if resources exist
            if let Some(mut resources) = game_resources.as_mut() {
                resources.health = resources.health.saturating_sub(10);
                info!(tick = time.elapsed_secs_f64(), "Player health remaining: {}", resources.health);
            }
        }
    }
}

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
