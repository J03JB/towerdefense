// src/utils.rs
use bevy::prelude::*;

pub fn distance(a: Vec2, b: Vec2) -> f32 {
    a.distance(b)
}

pub fn is_point_in_radius(center: Vec2, point: Vec2, radius: f32) -> bool {
    distance(center, point) <= radius
}

pub fn find_path(start: Vec2, end: Vec2, obstacles: &[Vec2]) -> Vec<Vec2> {
    // Simple pathfinding implementation
    // In a real game, you'd use A* or another algorithm
    vec![start, end]
}

