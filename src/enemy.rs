// src/enemy.rs - Enhanced with path following
use bevy::prelude::*;
use crate::map::Map;
use crate::utils::distance;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemies)
           .add_systems(Update, (move_enemies_along_path, check_enemy_health));
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

fn setup_enemies() {
}

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
        EnemyType::Basic => (100.0, 50.0, 10),
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
            path_index: 0,  // Start at the beginning of the path
            path_progress: 0.0,
        },
    ));
}

fn move_enemies_along_path(
    time: Res<Time>,
    map: Res<Map>,
    mut enemies: Query<(&mut Transform, &mut Enemy)>,
) {
    let delta = time.delta_secs();
    let path = map.get_path_positions();
    
    // Skip if path is empty or has only one point
    if path.len() < 2 {
        return;
    }
    
    for (mut transform, mut enemy) in enemies.iter_mut() {
        // If we've reached the end of the path
        if enemy.path_index >= path.len() - 1 {
            // Enemy reached the end - handle this in another system
            continue;
        }
        
        // Get current and next path positions
        let current_pos = path[enemy.path_index];
        let next_pos = path[enemy.path_index + 1];
        
        // Calculate direction vector between path points
        let direction = (next_pos - current_pos).normalize();
        
        // Move enemy along the path segment based on speed
        let distance_to_travel = enemy.speed * delta;
        let segment_length = distance(current_pos, next_pos);
        
        // Calculate how much progress we make along the current segment
        let progress_increase = distance_to_travel / segment_length;
        enemy.path_progress += progress_increase;
        
        // If we've completed this segment
        if enemy.path_progress >= 1.0 {
            // Move to next path segment
            enemy.path_index += 1;
            enemy.path_progress = 0.0;
            
            // If we just reached the end of the path
            if enemy.path_index >= path.len() - 1 {
                // Handle enemy reaching the end point
                continue;
            }
        }
        
        // Interpolate position between current and next path points
        let new_position = current_pos.lerp(next_pos, enemy.path_progress);
        
        // Update transform position
        transform.translation.x = new_position.x;
        transform.translation.y = new_position.y;
        
        // Optional: Rotate enemy to face movement direction
        if direction != Vec2::ZERO {
            let angle = direction.y.atan2(direction.x);
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

fn check_enemy_health(
    mut commands: Commands,
    mut enemies: Query<(Entity, &Enemy)>,
    mut game_resources: Option<ResMut<crate::game_state::PlayerResource>>,
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

/// Handle enemies that reach the end of the path
pub fn handle_enemies_at_end(
    mut commands: Commands,
    map: Res<Map>,
    enemies: Query<(Entity, &Transform, &Enemy)>,
    mut game_resources: Option<ResMut<crate::game_state::PlayerResource>>,
) {
    let end_pos = map.grid_to_world(map.end);
    
    for (entity, transform, _enemy) in enemies.iter() {
        let enemy_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let distance_to_end = distance(enemy_pos, end_pos);
        
        // If enemy is close enough to end point
        if distance_to_end < 10.0 {
            // Despawn the enemy
            commands.entity(entity).despawn();
            
            // Reduce player lives if resources exist
            if let Some(mut resources) = game_resources.as_mut() {
                resources.lives = resources.lives.saturating_sub(1);
            }
        }
    }
}

// System for enemy wave spawning
pub fn spawn_enemy_wave(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Res<Map>,
    wave_data: Vec<(EnemyType, u32)>,  // (enemy_type, count)
    spawn_interval: f32,
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
        
        // spawn_timer.tick(Time::delta())
        
        if spawn_timer.just_finished() {
            let enemy_type = enemies_to_spawn[current_index];
            spawn_enemy(&mut commands, &asset_server, &map, enemy_type);
            current_index += 1;
        }
        
        info!("wave not complete");
        // false // Wave not complete yet
    }
}
