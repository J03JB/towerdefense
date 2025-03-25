use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use crate::overlay::MainCamera;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, update_sprites);
    }
}

fn setup_camera(mut commands: Commands) {
    // commands.spawn(Camera2d);
    let projection = Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::FixedVertical {
            viewport_height: 1600.0,
        },
        // scale: 0.01,
        ..OrthographicProjection::default_2d()
    });

    commands.spawn((Camera2d, MainCamera));
}

fn update_sprites() {
    // Update sprite positions and animations
}
