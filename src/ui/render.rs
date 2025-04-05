use crate::core::config::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::core::map::Map;
use crate::ui::overlay::MainCamera;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::{PrimaryWindow, WindowResized};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera).add_systems(
            Update,
            (highlight_tile_under_cursor, resize_window, update_sprites),
        );
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}

fn resize_window(
    mut camera_query: Query<&mut OrthographicProjection, With<Camera>>,
    mut resize_event: EventReader<WindowResized>,
) {
    if let Some(w) = resize_event.read().next() {
        if let Ok(mut projection) = camera_query.get_single_mut() {
            projection.scaling_mode = ScalingMode::AutoMin {
                min_width: WINDOW_WIDTH,
                min_height: WINDOW_HEIGHT,
            };
        }
    }
}

fn update_sprites() {}

#[derive(Component)]
pub struct TileHighlight;

pub fn highlight_tile_under_cursor(
    mut commands: Commands,
    map: Res<Map>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<crate::ui::overlay::MainCamera>>,
    highlight: Query<Entity, With<TileHighlight>>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = windows.single();

    if let Some(cursor_position) = window.cursor_position() {
        if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
            let grid_x = ((world_position.x + crate::core::config::WINDOW_WIDTH / 2.0)
                / map.grid_size.x)
                .floor() as u32;
            let grid_y = ((crate::core::config::WINDOW_HEIGHT / 2.0 - world_position.y)
                / map.grid_size.y)
                .floor() as u32;

            let tile_center_x = (grid_x as f32 * map.grid_size.x) + (map.grid_size.x / 2.0)
                - crate::core::config::WINDOW_WIDTH / 2.0;
            let tile_center_y = crate::core::config::WINDOW_HEIGHT / 2.0
                - (grid_y as f32 * map.grid_size.y)
                - (map.grid_size.y / 2.0);

            // info!(
            //     "Cursor at world: ({:.1}, {:.1}), Grid: ({}, {}), Tile center: ({:.1}, {:.1})",
            //     world_position.x, world_position.y,
            //     grid_x, grid_y,
            //     tile_center_x, tile_center_y
            // );

            if let Ok(highlight_entity) = highlight.get_single() {
                commands.entity(highlight_entity).despawn();
            }

            let pos = UVec2::new(grid_x, grid_y);
            let is_buildable = map.is_buildable(pos);
            let what_color = if is_buildable {
                Color::srgba(0.0, 1.0, 0.0, 0.3)
            } else {
                Color::srgba(1.0, 0.0, 0.0, 0.3)
            };

            commands.spawn((
                Sprite {
                    color: what_color,
                    custom_size: Some(Vec2::new(map.grid_size.x, map.grid_size.y)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(tile_center_x, tile_center_y, 10.0)),
                TileHighlight,
            ));
        }
    }
}

pub fn render_background() {}
