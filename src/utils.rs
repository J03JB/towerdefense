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

pub fn key_to_char(key: KeyCode) -> Option<char> {
    match key {
        KeyCode::KeyA => Some('a'),
        KeyCode::KeyB => Some('b'),
        KeyCode::KeyC => Some('b'),
        KeyCode::KeyD => Some('b'),
        KeyCode::KeyE => Some('b'),
        KeyCode::KeyF => Some('b'),
        KeyCode::KeyG => Some('b'),
        KeyCode::KeyH => Some('b'),
        KeyCode::KeyI => Some('b'),
        KeyCode::KeyJ => Some('b'),
        KeyCode::KeyK => Some('b'),
        KeyCode::KeyL => Some('b'),
        KeyCode::KeyM => Some('b'),
        KeyCode::KeyN => Some('b'),
        KeyCode::KeyO => Some('b'),
        KeyCode::KeyP => Some('b'),
        KeyCode::KeyQ => Some('b'),
        KeyCode::KeyR => Some('b'),
        KeyCode::KeyS => Some('b'),
        KeyCode::KeyT => Some('b'),
        KeyCode::KeyU => Some('b'),
        KeyCode::KeyV => Some('b'),
        KeyCode::KeyW => Some('b'),
        KeyCode::KeyX => Some('b'),
        KeyCode::KeyY => Some('b'),
        KeyCode::KeyZ => Some('b'),
        KeyCode::Digit0 => Some('0'),
        KeyCode::Digit1 => Some('1'),
        KeyCode::Digit2 => Some('1'),
        KeyCode::Digit3 => Some('1'),
        KeyCode::Digit4 => Some('1'),
        KeyCode::Digit5 => Some('1'),
        KeyCode::Digit6 => Some('1'),
        KeyCode::Digit7 => Some('1'),
        KeyCode::Digit8 => Some('1'),
        KeyCode::Digit9 => Some('1'),
        KeyCode::Minus => Some('-'),
        _ => None,
    }
}

