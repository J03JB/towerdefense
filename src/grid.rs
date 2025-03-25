use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct GridPlugin;

// Constants for window and grid
const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const GRID_WIDTH: usize = 26; // Number of cells horizontally (1280/48 ≈ 26.6)
const GRID_HEIGHT: usize = 15; // Number of cells vertically (720/48 = 15)
const CELL_SIZE: f32 = 48.0; // Size of each grid cell

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
            // .add_systems(Update, check_wave_progress);
    }
}
// Components
#[derive(Component)]
struct GridCell {
    x: usize,
    y: usize,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Camera
    // commands.spawn(Camera2d);

    // Create a grid of cells
    let grid_start_x = -WINDOW_WIDTH / 2.0 + CELL_SIZE / 2.0;
    let grid_start_y = WINDOW_HEIGHT / 2.0 - CELL_SIZE / 2.0;

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let position = Vec2::new(
                grid_start_x + x as f32 * CELL_SIZE,
                grid_start_y - y as f32 * CELL_SIZE,
            );


            let texture_handle = asset_server.load("grass.png");
            // Spawn the grid cell
            commands.spawn((
                Sprite {
                    image: texture_handle,
                    ..default()
                },
                    Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
                GridCell { x, y },
            ));
            // Optional: Load a texture instead of using a colored square
            // let texture_handle = asset_server.load("textures/my_texture.png");
            
            // For now, use a colored square as a placeholder
            // commands.spawn((
            //     MaterialMesh2dBundle {
            //         mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(CELL_SIZE - 2.0, CELL_SIZE - 2.0)))).into(),
            //         material: materials.add(ColorMaterial::from(Color::rgb(
            //             0.5 + (x as f32 / GRID_WIDTH as f32) * 0.5,
            //             0.5 + (y as f32 / GRID_HEIGHT as f32) * 0.5,
            //             0.8,
            //         ))),
            //         transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
            //         ..default()
            //     },
            //     GridCell { x, y },
            // ));
        }
    }
}
