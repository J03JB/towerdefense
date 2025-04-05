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

#[derive(Resource, Default)]
pub struct MyWorldCoords(Vec2);

#[derive(Component)]
pub struct MainCamera;

pub fn my_cursor_system(
    mut mycoords: ResMut<MyWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();

    let window = q_window.single();

    if let Some(cursor_position) = window.cursor_position() {
        let viewport_position = cursor_position;

        if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, viewport_position)
        {
            mycoords.0 = world_position;
            eprintln!("World coords: {}/{}", world_position.x, world_position.y);
        }
    }
}
