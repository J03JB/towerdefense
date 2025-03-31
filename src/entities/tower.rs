use crate::core::map::Map;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_towers).add_systems(
            Update,
            (
                tower_targeting,
                tower_shooting,
                handle_tower_upgrades,
                handle_tower_placement,
            ),
        );
    }
}

#[derive(Component)]
pub struct Tower {
    pub tower_type: TowerType,
    pub range: f32,
    pub damage: f32,
    pub fire_rate: f32,
    pub last_shot: f32,
    pub level: u32,
    pub target: Option<Entity>,
}

#[derive(Debug, Clone, Copy)]
pub enum TowerType {
    Archer,
    Cannon,
    LongBow,
    Splash,
    Slow,
}

fn setup_towers() {
    // Initialize tower-related resources
}

fn tower_targeting() {
    // Logic for towers to find and select targets
}

fn tower_shooting() {
    // Logic for towers to shoot at targets
}

fn handle_tower_upgrades() {
    // Handle tower upgrades and improvements
}

fn handle_tower_placement(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    map: Res<Map>,
    asset_server: Res<AssetServer>,
    player_resources: Option<ResMut<crate::core::game_state::PlayerResource>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera_q.single();
        let window = windows.single();

        if let Some(cursor_position) = window.cursor_position() {
            if let Ok(world_position) =
                camera.viewport_to_world_2d(camera_transform, cursor_position)
            {
                let grid_pos = map.world_to_grid(world_position);

                if map.is_buildable(grid_pos) {
                    let tower_cost = 0;

                    if let Some(mut resources) = player_resources {
                        if resources.money < tower_cost {   
                            info!("Not enough money to build tower");
                            return;
                        }
                        resources.money -= tower_cost;
                    };

                    let world_pos = map.grid_to_world(grid_pos);

                    commands.spawn((
                        Sprite {
                            image: asset_server.load("textures/archer01.png"),
                            ..default()
                        },
                        Transform::from_translation(Vec3::new(world_pos.x, world_pos.y, 10.0)),
                        Tower {
                            tower_type: TowerType::Archer,
                            range: 150.0,
                            damage: 10.0,
                            fire_rate: 1.0,
                            last_shot: 0.0,
                            level: 1,
                            target: None,
                        },
                    ));

                    info!("Tower placed at grid position: {:?}", grid_pos);
                }
            }
        }
    }
}
