use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::core::game_state::PlayerResource;

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyWorldCoords>()
            // app.add_systems(Startup, setup)
            .add_systems(Update, health_bar);
            // .add_systems(Update, my_cursor_system);
    }
}

#[derive(Component)]
struct HealthBar;

fn health_bar(
    mut commands: Commands,
    player_resources: Option<Res<PlayerResource>>,
    asset_server: Res<AssetServer>,
    mut texture_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut query: Query<&mut Sprite, With<HealthBar>>,
) {
    let health_bar_texture = asset_server.load("originals/health_bar.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(162, 24), 3, 4, None, None);
    let texture_layout = texture_layout.add(layout);

    let player_health = player_resources.map(|pr| pr.health).unwrap_or(0);
    // info!("player health: {}", player_health);
    let player_max_health = 100;

    let health_index = if player_max_health > 0 {
        ((1.0 - (player_health as f32 / player_max_health as f32)) * 10.0).round() as usize
    } else {
        10
    };
    // info!("health index: {}", health_index);

    let health_index = health_index.min(10);

    if let Ok(mut sprite) = query.get_single_mut() {
        sprite.texture_atlas = Some(TextureAtlas {
            layout: texture_layout.clone(),
            index: health_index,
        });
    } else {
        commands.spawn((
            Sprite {
                image: health_bar_texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_layout.clone(),
                    index: health_index,
                }),
                ..default()
            },
    // TODO: change translation to overlay::something
            Transform::from_translation(Vec3::new(-465.0, 310.0, 10.0)),
            HealthBar,
        ));
    }
}

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
pub struct MyWorldCoords(Vec2);

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

// pub fn setup(mut commands: Commands) {
// Make sure to add the marker component when you set up your camera
// commands.spawn(Camera2d {
//     MainCameram
//     ..default()
//
// });
// commands.spawn((Camera2d, OverLayCamera));
// }

pub fn my_cursor_system(
    mut mycoords: ResMut<MyWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    if let Some(cursor_position) = window.cursor_position() {
        // Convert cursor position to viewport position
        let viewport_position = cursor_position;

        // Use the new viewport_to_world_2d which returns a Result instead of Option
        match camera.viewport_to_world_2d(camera_transform, viewport_position) {
            Ok(world_position) => {
                mycoords.0 = world_position;
                eprintln!("World coords: {}/{}", world_position.x, world_position.y);
            }
            Err(_) => {
                // Handle error - cursor outside viewport or other conversion error
            }
        }
    }
    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    //     if let Some(cursor_position) = window.cursor_position() {
    //         if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, viewport_position) {
    //
    //         mycoords.0 = world_position;
    //         eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    //     }
    // }
}
