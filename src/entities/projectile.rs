use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_projectiles, handle_projectile_collisions));
    }
}

#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
    pub speed: f32,
    pub target: Entity,
    pub projectile_type: ProjectileType,
}

#[derive(Debug, Clone, Copy)]
pub enum ProjectileType {
    Bullet,
    Missile,
    Laser,
    AreaEffect,
}

fn move_projectiles() {
    // Move projectiles toward their targets
}

fn handle_projectile_collisions() {
    // Detect and handle collisions between projectiles and enemies
}

