use crate::core::config::{CELL_SIZE, GRID_HEIGHT, GRID_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH}; // Assuming these are f32 constants
use crate::core::game_state::GameState;
use crate::core::map::Map;
use crate::levels::level_textures::{
    AvailableTextures, PathTexture, TextureSelectorPanel, get_selected_texture,
};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<EditorState>()
            .init_resource::<EditorTextInput>()
            .add_systems(Startup, setup_editor)
            .add_systems(
                Update,
                (
                    editor_input_handler,
                    spawn_context_menu,
                    handle_context_menu_interaction,
                    render_editor_path,
                    export_level_data,
                    handle_save_dialog,
                    handle_text_input,
                    toggle_editor_tool,
                )
                    .run_if(in_state(EditorState::Active)),
            )
            .add_systems(
                OnEnter(EditorState::Active),
                crate::levels::level_textures::setup_texture_selector,
            );
    }
}
#[derive(Component)]
pub struct LevelNameInput;

#[derive(Component)]
pub struct SaveDialog;

#[derive(Resource)]
pub struct EditorTextInput {
    pub level_name: String,
    pub dialog_open: bool,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default, Reflect)]
pub enum EditorState {
    #[default]
    Inactive,
    Active,
}

#[derive(Resource, Default)]
pub struct EditorData {
    pub path: Vec<(UVec2, String)>,
    pub start: Option<UVec2>,
    pub end: Option<UVec2>,
    pub buildable_areas: Vec<UVec2>,
    pub current_tool: EditorTool,
    pub grid_overlay: bool,
    pub selected_texture: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum EditorTool {
    #[default]
    PathPlacer,
    StartPoint,
    EndPoint,
    BuildableArea,
    TextureSelector,
}

#[derive(Component)]
pub struct EditorPathMarker;

#[derive(Component)]
pub struct EditorToolDisplay;

#[derive(Serialize, Deserialize)]
pub struct LevelData {
    pub path: Vec<Vec<u32>>, // Stored as [[x, y], [x, y], ...]
    pub path_textures: Vec<PathTexture>,
    pub start: Vec<u32>,                // [x, y]
    pub end: Vec<u32>,                  // [x, y]
    pub buildable_areas: Vec<Vec<u32>>, // [[x, y], [x, y], ...]
    pub dimensions: Vec<u32>,           // [width, height]
}

fn setup_editor(
    mut commands: Commands,
    editor_state: ResMut<State<EditorState>>,
    mut next_state: ResMut<NextState<EditorState>>,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--editor") {
        // Create editor UI
        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                EditorToolDisplay,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Editor Mode: Path Placement"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor::WHITE,
                ));

                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(120.0),
                            height: Val::Px(40.0),
                            margin: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Path Tool"),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor::WHITE,
                        ));
                    })
                    .insert(EditorButton(EditorTool::PathPlacer));
                let button_style = Node {
                    width: Val::Px(120.0),
                    height: Val::Px(40.0),
                    margin: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                };

                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(120.0),
                            height: Val::Px(40.0),
                            margin: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Start Point"),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor::WHITE,
                        ));
                    })
                    .insert(EditorButton(EditorTool::StartPoint));

                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(120.0),
                            height: Val::Px(40.0),
                            margin: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("End Point"),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor::WHITE,
                        ));
                    })
                    .insert(EditorButton(EditorTool::EndPoint));

                parent
                    .spawn((
                        Button,
                        button_style.clone(),
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Buildable Area"),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor::WHITE,
                        ));
                    })
                    .insert(EditorButton(EditorTool::BuildableArea));

                parent
                    .spawn((
                        Button,
                        button_style.clone(),
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Export JSON"),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor::WHITE,
                        ));
                    })
                    .insert(ExportButton);
            });

        commands.insert_resource(EditorData::default());

        next_state.set(EditorState::Active);

        game_state.set(GameState::Playing);
    }
}

#[derive(Component)]
struct EditorButton(EditorTool);

#[derive(Component)]
struct ExportButton;
#[allow(clippy::too_many_arguments)]
fn editor_input_handler(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    key_press: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut editor_data: ResMut<EditorData>,
    editor_text_input: ResMut<EditorTextInput>,
    map: Option<Res<Map>>,
    asset_server: Res<AssetServer>,
    textures: Res<AvailableTextures>,
    markers_query: Query<(Entity, &Transform), With<EditorPathMarker>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera_q.single();
        let window = windows.single();

        if let Some(cursor_position) = window.cursor_position() {
            if let Ok(world_position) =
                camera.viewport_to_world_2d(camera_transform, cursor_position)
            {
                let grid_pos = if let Some(map) = map.as_ref() {
                    map.world_to_grid(world_position)
                } else {
                    let grid_size = Vec2::new(48.0, 48.0);
                    UVec2::new(
                        (world_position.x / grid_size.x).floor() as u32,
                        (world_position.y / grid_size.y).floor() as u32,
                    )
                };

                let grid_size = Vec2::new(48.0, 48.0);
                match editor_data.current_tool {
                    EditorTool::PathPlacer => {
                        // Check if grid position already exists
                        let path_index = editor_data
                            .path
                            .iter()
                            .position(|(pos, _)| *pos == grid_pos);

                        // Get the current texture from AvailableTextures
                        let texture_path = get_selected_texture(&textures);

                        if let Some(idx) = path_index {
                            // Update texture for existing path position
                            editor_data.path[idx].1 = texture_path.clone();

                            // Find and despawn the old marker
                            for (entity, transform) in markers_query.iter() {
                                let pos =
                                    Vec2::new(transform.translation.x, transform.translation.y);
                                let world_pos = if let Some(map) = map.as_ref() {
                                    map.grid_to_world(grid_pos)
                                } else {
                                    let grid_start_x = -crate::core::config::WINDOW_WIDTH / 2.0
                                        + grid_size.x / 2.0;
                                    let grid_start_y = crate::core::config::WINDOW_HEIGHT / 2.0
                                        - grid_size.y / 2.0;

                                    Vec2::new(
                                        grid_start_x + grid_pos.x as f32 * grid_size.x,
                                        grid_start_y - grid_pos.y as f32 * grid_size.y,
                                    )
                                };

                                if pos.distance(world_pos) < 5.0 {
                                    commands.entity(entity).despawn();
                                }
                            }
                        } else {
                            // Add new path position with texture
                            editor_data.path.push((grid_pos, texture_path.clone()));
                        }

                        // Spawn or update the visual marker
                        let world_pos = if let Some(map) = map.as_ref() {
                            map.grid_to_world(grid_pos)
                        } else {
                            let grid_start_x =
                                -crate::core::config::WINDOW_WIDTH / 2.0 + grid_size.x / 2.0;
                            let grid_start_y =
                                crate::core::config::WINDOW_HEIGHT / 2.0 - grid_size.y / 2.0;

                            Vec2::new(
                                grid_start_x + grid_pos.x as f32 * grid_size.x,
                                grid_start_y - grid_pos.y as f32 * grid_size.y,
                            )
                        };

                        commands.spawn((
                            Sprite {
                                image: asset_server.load(&texture_path),
                                custom_size: Some(Vec2::new(48.0, 48.0)),
                                ..default()
                            },
                            Transform::from_translation(world_pos.extend(1.0)),
                            EditorPathMarker,
                        ));
                    }
                    EditorTool::StartPoint => {
                        editor_data.start = Some(grid_pos);

                        // Clear previous start marker and create a new one
                        commands.spawn((
                            Sprite {
                                color: Color::srgba(0.2, 0.9, 0.2, 0.7),
                                custom_size: Some(Vec2::new(48.0, 48.0)),
                                ..default()
                            },
                            Transform::from_translation(
                                Vec2::new(
                                    grid_pos.x as f32 * grid_size.x + grid_size.x * 0.5,
                                    grid_pos.y as f32 * grid_size.y + grid_size.y * 0.5,
                                )
                                .extend(1.0),
                            ),
                            EditorPathMarker,
                        ));
                    }
                    EditorTool::EndPoint => {
                        editor_data.end = Some(grid_pos);

                        // Clear previous end marker and create a new one
                        commands.spawn((
                            Sprite {
                                color: Color::srgba(0.9, 0.1, 0.1, 0.7),
                                custom_size: Some(Vec2::new(grid_size.x, grid_size.y)),
                                ..default()
                            },
                            Transform::from_translation(
                                Vec2::new(
                                    grid_pos.x as f32 * grid_size.x + grid_size.x * 0.5,
                                    grid_pos.y as f32 * grid_size.y + grid_size.y * 0.5,
                                )
                                .extend(1.0),
                            ),
                            EditorPathMarker,
                        ));
                    }
                    EditorTool::BuildableArea => {
                        if !editor_data.buildable_areas.contains(&grid_pos) {
                            editor_data.buildable_areas.push(grid_pos);

                            commands.spawn((
                                Sprite {
                                    color: Color::srgba(0.2, 0.5, 0.8, 0.4),
                                    custom_size: Some(Vec2::new(grid_size.x, grid_size.y)),
                                    ..default()
                                },
                                Transform::from_translation(
                                    Vec2::new(
                                        grid_pos.x as f32 * grid_size.x + grid_size.x * 0.5,
                                        grid_pos.y as f32 * grid_size.y + grid_size.y * 0.5,
                                    )
                                    .extend(0.5),
                                ),
                                EditorPathMarker,
                            ));
                        }
                    }
                    EditorTool::TextureSelector => {
                        // When in texture selector mode, clicking doesn't place anything
                        // The texture panel should be visible and the user just selects a texture
                        // No additional action needed here as texture selection is handled elsewhere
                    }
                }
            }
        }
    }

    if mouse_input.just_pressed(MouseButton::Right)
        && editor_data.current_tool == EditorTool::PathPlacer
    {
        editor_data.path.pop();
    }

    if key_press.just_pressed(KeyCode::Enter) {
        editor_data.current_tool = EditorTool::PathPlacer;
    } else if key_press.just_pressed(KeyCode::KeyS) {
        editor_data.current_tool = EditorTool::StartPoint;
    } else if key_press.just_pressed(KeyCode::KeyE) {
        editor_data.current_tool = EditorTool::EndPoint;
    } else if key_press.just_pressed(KeyCode::KeyB) {
        editor_data.current_tool = EditorTool::BuildableArea;
    } else if key_press.just_pressed(KeyCode::KeyS) && key_press.pressed(KeyCode::ControlLeft) {
        // Ctrl+S to export
        export_level(&editor_data, &editor_text_input.level_name);
    } else if key_press.just_pressed(KeyCode::KeyG) {
        editor_data.grid_overlay = !editor_data.grid_overlay;
        info!("overlay")
    }
}

// Function to render path and grid overlay in the editor
fn render_editor_path(editor_data: Res<EditorData>, mut gizmos: Gizmos, map: Option<Res<Map>>) {
    let (grid_size, dimensions) = if let Some(map_res) = map.as_ref() {
        (map_res.grid_size, map_res.dimensions)
    } else {
        (
            Vec2::new(CELL_SIZE, CELL_SIZE),
            UVec2::new(GRID_WIDTH as u32, GRID_HEIGHT as u32),
        )
    };

    let grid_to_world_logic = |grid_pos: UVec2| -> Vec2 {
        let grid_origin_x = -WINDOW_WIDTH / 2.0;
        let grid_origin_y = WINDOW_HEIGHT / 2.0;
        let cell_corner_x = grid_origin_x + grid_pos.x as f32 * grid_size.x;
        let cell_corner_y = grid_origin_y - grid_pos.y as f32 * grid_size.y;
        Vec2::new(
            cell_corner_x + grid_size.x / 2.0,
            cell_corner_y - grid_size.y / 2.0,
        )
    };

    if editor_data.path.len() >= 2 {
        for i in 0..editor_data.path.len() - 1 {
            let start_grid = editor_data.path[i].0;
            let end_grid = editor_data.path[i + 1].0;

            let start_world = grid_to_world_logic(start_grid);
            let end_world = grid_to_world_logic(end_grid);

            gizmos.line_2d(start_world, end_world, Color::srgb(0.9, 0.3, 0.7));
        }
    }

    if editor_data.grid_overlay {
        let grid_world_left = -WINDOW_WIDTH / 2.0;
        let grid_world_top = WINDOW_HEIGHT / 2.0;
        let grid_world_width = dimensions.x as f32 * grid_size.x;
        let grid_world_height = dimensions.y as f32 * grid_size.y;
        let grid_world_right = grid_world_left + grid_world_width;
        let grid_world_bottom = grid_world_top - grid_world_height; // Subtract because Y decreases

        for x_index in 0..=dimensions.x {
            let x_world = grid_world_left + x_index as f32 * grid_size.x;
            let line_start = Vec2::new(x_world, grid_world_top);
            let line_end = Vec2::new(x_world, grid_world_bottom);
            gizmos.line_2d(line_start, line_end, Color::srgba(0.5, 0.5, 0.5, 0.8)); // Gray color
        }

        for y_index in 0..=dimensions.y {
            let y_world = grid_world_top - y_index as f32 * grid_size.y; // Subtract because Y decreases
            let line_start = Vec2::new(grid_world_left, y_world);
            let line_end = Vec2::new(grid_world_right, y_world);
            gizmos.line_2d(line_start, line_end, Color::srgba(0.5, 0.5, 0.5, 0.8)); // Gray color
        }
    }
}

fn toggle_editor_tool(
    buttons: Query<(&Interaction, &EditorButton), Changed<Interaction>>,
    mut editor_data: ResMut<EditorData>,
    mut text_query: Query<&mut Text, With<EditorToolDisplay>>,
) {
    for (interaction, button) in &buttons {
        if let Interaction::Pressed = interaction {
            editor_data.current_tool = button.0.clone();

            if let Ok(mut text) = text_query.get_single_mut() {
                *text = Text::new(format!("Editor Mode: {:?}", editor_data.current_tool));
            }
        }
    }
}

fn export_level_data(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExportButton>)>,
    editor_data: Res<EditorData>,
    mut editor_text_input: ResMut<EditorTextInput>,
    mut commands: Commands,
) {
    for interaction in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            editor_text_input.dialog_open = true;
            spawn_save_dialog(&mut commands, &mut editor_text_input);
        }
    }
}

fn export_level(editor_data: &EditorData, level_name: &str) {
    let mut path = Vec::new();
    let mut path_textures = Vec::new();

    for (pos, texture) in &editor_data.path {
        path.push(vec![pos.x, pos.y]);

        path_textures.push(PathTexture {
            position: vec![pos.x, pos.y],
            texture: texture.clone(),
        });
    }

    let level_data = LevelData {
        path,
        path_textures, // Include texture information
        start: if let Some(start) = editor_data.start {
            vec![start.x, start.y]
        } else {
            vec![0, 0]
        },
        end: if let Some(end) = editor_data.end {
            vec![end.x, end.y]
        } else {
            vec![0, 0]
        },
        buildable_areas: editor_data
            .buildable_areas
            .iter()
            .map(|pos| vec![pos.x, pos.y])
            .collect(),
        dimensions: vec![27, 15],
    };

    //     let level_data = LevelData {
    //         path: editor_data
    //             .path
    //             .iter()
    //             .map(|pos| vec![pos.x, pos.y])
    //             .collect(),
    //         start: if let Some(start) = editor_data.start {
    //             vec![start.x, start.y]
    //         } else {
    //             vec![0, 0]
    //         },
    //         end: if let Some(end) = editor_data.end {
    //             vec![end.x, end.y]
    //         } else {
    //             vec![0, 0]
    //         },
    //         buildable_areas: editor_data
    //             .buildable_areas
    //             .iter()
    //             .map(|pos| vec![pos.x, pos.y])
    //             .collect(),
    //         dimensions: vec![27, 15],
    //     };

    if let Ok(json_string) = serde_json::to_string_pretty(&level_data) {
        std::fs::create_dir_all("assets/levels").unwrap_or_else(|_| {
            eprintln!("Failed to create levels directory");
        });

        let file_path = format!("assets/levels/{}.json", level_name);
        if let Ok(mut file) = File::create(&file_path) {
            if let Err(e) = file.write_all(json_string.as_bytes()) {
                eprintln!("Failed to write level data: {}", e);
            } else {
                println!("Successfully exported level data to '{}'", file_path);
            }
        } else {
            eprintln!("Failed to create level file");
        }
    } else {
        eprintln!("Failed to serialize level data");
    }
}
#[derive(Component)]
struct ConfirmSaveButton;

#[derive(Component)]
struct CancelSaveButton;

impl Default for EditorTextInput {
    fn default() -> Self {
        Self {
            level_name: "".to_string(),
            dialog_open: false,
        }
    }
}

fn handle_text_input(
    mut editor_text_input: ResMut<EditorTextInput>,
    editor_data: ResMut<EditorData>,
    mut text_query: Query<&mut Text, With<LevelNameInput>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    dialog_query: Query<Entity, With<SaveDialog>>,
) {
    if !editor_text_input.dialog_open {
        return;
    }

    for key in keyboard_input.get_just_pressed() {
        let mut name_changed = false;

        if *key == KeyCode::Backspace {
            if !editor_text_input.level_name.is_empty() {
                editor_text_input.level_name.pop();
                name_changed = true;
            }
        } else if *key == KeyCode::Space {
            editor_text_input.level_name.push('_');
            name_changed = true;
        } else if *key == KeyCode::Minus {
            editor_text_input.level_name.push('-');
            name_changed = true;
        } else if *key == KeyCode::Enter {
            export_level(&editor_data, &editor_text_input.level_name);
            editor_text_input.dialog_open = false;
            for entity in dialog_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
        } else {
            let char = crate::core::utils::key_to_char(*key);
            editor_text_input
                .level_name
                .push(char.expect("key not found"));
            name_changed = true;
        }

        if name_changed {
            for mut text in text_query.iter_mut() {
                *text = Text::new(editor_text_input.level_name.clone());
            }
        }
    }

    // Update displayed text
    if let Ok(mut text) = text_query.get_single_mut() {
        *text = Text::new(editor_text_input.level_name.clone());
    }
}

fn handle_save_dialog(
    mut commands: Commands,
    mut editor_text_input: ResMut<EditorTextInput>,
    cancel_interaction: Query<&Interaction, (Changed<Interaction>, With<CancelSaveButton>)>,
    save_interaction: Query<&Interaction, (Changed<Interaction>, With<ConfirmSaveButton>)>,
    dialog_query: Query<Entity, With<SaveDialog>>,
    editor_data: Res<EditorData>,
) {
    // Handle cancel button
    for interaction in cancel_interaction.iter() {
        if matches!(interaction, Interaction::Pressed) {
            editor_text_input.dialog_open = false;
            for entity in dialog_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    // Handle save button
    for interaction in save_interaction.iter() {
        if matches!(interaction, Interaction::Pressed) {
            export_level(&editor_data, &editor_text_input.level_name);
            editor_text_input.dialog_open = false;
            for entity in dialog_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
    // for interaction in &cancel_query {
    //     if matches!(interaction, Interaction::Pressed) {
    //         editor_text_input.dialog_open = false;
    //         for entity in &dialog_query {
    //             commands.entity(entity).despawn_recursive();
    //         }
    //     }
    // }
    //
    // for interaction in &save_query {
    //     if matches!(interaction, Interaction::Pressed) {
    //         export_level(&editor_data, &editor_text_input.level_name);
    //         editor_text_input.dialog_open = false;
    //         for entity in &dialog_query {
    //             commands.entity(entity).despawn_recursive();
    //         }
    //     }
    // }
}

fn spawn_save_dialog(commands: &mut Commands, editor_text_input: &mut ResMut<EditorTextInput>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(300.0),
                top: Val::Px(200.0),
                width: Val::Px(400.0),
                height: Val::Auto,
                padding: UiRect::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            BorderColor(Color::srgb(0.7, 0.7, 0.7)),
            SaveDialog,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Save Level"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor::WHITE,
            ));

            // Text input field
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        border: UiRect::all(Val::Px(1.0)),
                        padding: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                    BorderColor(Color::srgb(0.5, 0.5, 0.5)),
                    LevelNameInput,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(editor_text_input.level_name.clone()),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor::WHITE,
                    ));
                });

            // Button row
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::End,
                    column_gap: Val::Px(10.0),
                    ..default()
                })
                .with_children(|parent| {
                    // Cancel button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(100.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.5, 0.1, 0.1)),
                            CancelSaveButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Cancel"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor::WHITE,
                            ));
                        });

                    // Save button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(100.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.1, 0.5, 0.1)),
                            ConfirmSaveButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Save"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor::WHITE,
                            ));
                        });
                });
        });
}

// Add these components to src/level_editor.rs

#[derive(Component)]
struct ContextMenu;

#[derive(Component)]
enum ContextMenuOption {
    PathTool,
    StartPoint,
    EndPoint,
    BuildableArea,
    Delete,
    Save,
}

fn spawn_context_menu(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    context_menu_query: Query<Entity, With<ContextMenu>>,
    markers_query: Query<(Entity, &Transform), With<EditorPathMarker>>,
) {
    // If right mouse button was just pressed
    if mouse_input.just_pressed(MouseButton::Right) {
        // First, despawn any existing context menu
        for entity in context_menu_query.iter() {
            commands.entity(entity).despawn_recursive();
        }

        // Get cursor position for menu placement
        if let Some(cursor_position) = windows.single().cursor_position() {
            // Spawn context menu at cursor position
            commands
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(cursor_position.x),
                        top: Val::Px(cursor_position.y),
                        width: Val::Px(150.0),
                        // This will be determined by child content
                        height: Val::Auto,
                        padding: UiRect::all(Val::Px(5.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    BorderColor(Color::srgb(0.7, 0.7, 0.7)),
                    ContextMenu,
                ))
                .with_children(|parent| {
                    // Add menu options
                    spawn_menu_option(parent, "Path Tool", ContextMenuOption::PathTool);
                    spawn_menu_option(parent, "Start Point", ContextMenuOption::StartPoint);
                    spawn_menu_option(parent, "End Point", ContextMenuOption::EndPoint);
                    spawn_menu_option(parent, "Buildable Area", ContextMenuOption::BuildableArea);

                    // Add a separator
                    parent
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(1.0),
                            margin: UiRect::vertical(Val::Px(5.0)),
                            ..default()
                        })
                        .insert(BackgroundColor(Color::srgb(0.5, 0.5, 0.5)));

                    spawn_menu_option(parent, "Delete Tile", ContextMenuOption::Delete);
                    spawn_menu_option(parent, "Save Level", ContextMenuOption::Save);
                });
        }
    }

    // Close menu when clicking elsewhere
    if mouse_input.just_pressed(MouseButton::Left) {
        for entity in context_menu_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_menu_option(parent: &mut ChildBuilder, text: &str, option: ContextMenuOption) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(30.0),
                margin: UiRect::vertical(Val::Px(2.0)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            option,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(text),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor::WHITE,
            ));
        });
}

#[allow(clippy::too_many_arguments)]
fn handle_context_menu_interaction(
    mut commands: Commands,
    interaction_query: Query<
        (&Interaction, &ContextMenuOption),
        (Changed<Interaction>, With<Button>),
    >,
    context_menu_query: Query<Entity, With<ContextMenu>>,
    mut editor_data: ResMut<EditorData>,
    mut editor_text_input: ResMut<EditorTextInput>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    map: Option<Res<Map>>,
    markers_query: Query<(Entity, &Transform), With<EditorPathMarker>>,
    textures: Res<AvailableTextures>,
) {
    for (interaction, option) in interaction_query.iter() {
        if matches!(interaction, Interaction::Pressed) {
            match option {
                ContextMenuOption::PathTool => {
                    editor_data.current_tool = EditorTool::PathPlacer;
                }
                ContextMenuOption::StartPoint => {
                    editor_data.current_tool = EditorTool::StartPoint;
                }
                ContextMenuOption::EndPoint => {
                    editor_data.current_tool = EditorTool::EndPoint;
                }
                ContextMenuOption::BuildableArea => {
                    editor_data.current_tool = EditorTool::BuildableArea;
                }
                ContextMenuOption::Delete => {
                    // Get current cursor position
                    if let Some(cursor_position) = windows.single().cursor_position() {
                        let (camera, camera_transform) = camera_q.single();

                        if let Ok(world_position) =
                            camera.viewport_to_world_2d(camera_transform, cursor_position)
                        {
                            let grid_pos = if let Some(map) = map.as_ref() {
                                map.world_to_grid(world_position)
                            } else {
                                let grid_size = Vec2::new(48.0, 48.0);
                                UVec2::new(
                                    (world_position.x / grid_size.x).floor() as u32,
                                    (world_position.y / grid_size.y).floor() as u32,
                                )
                            };

                            // Remove from appropriate lists
                            editor_data.path.retain(|(pos, _)| *pos != grid_pos);
                            editor_data.buildable_areas.retain(|&pos| pos != grid_pos);

                            // Clear start/end if they match
                            if editor_data.start == Some(grid_pos) {
                                editor_data.start = None;
                            }

                            if editor_data.end == Some(grid_pos) {
                                editor_data.end = None;
                            }

                            // Remove visual markers (handled in a separate function)
                            let grid_size = 48.0;
                            let grid_pos_world = Vec2::new(
                                grid_pos.x as f32 * 48.0 + 24.0,
                                grid_pos.y as f32 * 48.0 + 24.0,
                            );
                            for (entity, transform) in markers_query.iter() {
                                let pos =
                                    Vec2::new(transform.translation.x, transform.translation.y);
                                if pos.distance(grid_pos_world) < 5.0 {
                                    commands.entity(entity).despawn();
                                }
                            }
                        }
                    }
                }
                ContextMenuOption::Save => {
                    // Show save dialog
                    editor_text_input.dialog_open = true;
                    spawn_save_dialog(&mut commands, &mut editor_text_input);
                }
            }

            // Close the context menu after selection
            for entity in context_menu_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
