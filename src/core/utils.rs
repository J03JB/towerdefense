use bevy::prelude::*;
use crate::core::map::Map;

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

pub fn get_grid_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    map: &Map,
) -> Option<UVec2> {
    window.cursor_position()
        .and_then(|cursor_position| {
            camera.viewport_to_world_2d(camera_transform, cursor_position).ok()
        })
        .map(|world_position| {
            let grid_x = ((world_position.x + crate::core::config::WINDOW_WIDTH / 2.0) / map.grid_size.x).floor() as u32;
            let grid_y = ((crate::core::config::WINDOW_HEIGHT / 2.0 - world_position.y) / map.grid_size.y).floor() as u32;
            UVec2::new(grid_x, grid_y)
        })
}

pub fn key_to_char(key: KeyCode) -> Option<char> {
    match key {
        KeyCode::KeyA => Some('a'),
        KeyCode::KeyB => Some('b'),
        KeyCode::KeyC => Some('c'),
        KeyCode::KeyD => Some('d'),
        KeyCode::KeyE => Some('e'),
        KeyCode::KeyF => Some('f'),
        KeyCode::KeyG => Some('g'),
        KeyCode::KeyH => Some('h'),
        KeyCode::KeyI => Some('i'),
        KeyCode::KeyJ => Some('j'),
        KeyCode::KeyK => Some('k'),
        KeyCode::KeyL => Some('l'),
        KeyCode::KeyM => Some('m'),
        KeyCode::KeyN => Some('n'),
        KeyCode::KeyO => Some('o'),
        KeyCode::KeyP => Some('p'),
        KeyCode::KeyQ => Some('q'),
        KeyCode::KeyR => Some('r'),
        KeyCode::KeyS => Some('s'),
        KeyCode::KeyT => Some('t'),
        KeyCode::KeyU => Some('u'),
        KeyCode::KeyV => Some('v'),
        KeyCode::KeyW => Some('w'),
        KeyCode::KeyX => Some('x'),
        KeyCode::KeyY => Some('y'),
        KeyCode::KeyZ => Some('z'),
        KeyCode::Digit0 => Some('0'),
        KeyCode::Digit1 => Some('1'),
        KeyCode::Digit2 => Some('2'),
        KeyCode::Digit3 => Some('3'),
        KeyCode::Digit4 => Some('4'),
        KeyCode::Digit5 => Some('5'),
        KeyCode::Digit6 => Some('6'),
        KeyCode::Digit7 => Some('7'),
        KeyCode::Digit8 => Some('8'),
        KeyCode::Digit9 => Some('9'),
        KeyCode::Minus => Some('-'),
        _ => None,
    }
}

pub fn format_me(a: &str, b: &str, c: &str) -> String {
    [a, b, c].join(" ")
}
