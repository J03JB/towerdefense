use crate::map::Map;
use crate::game_state::GameState;
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
                    render_editor_path,
                    export_level_data,
                    handle_save_dialog,
                    handle_text_input,
                    toggle_editor_tool,
                )
                    .run_if(in_state(EditorState::Active)),
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum EditorTool {
    #[default]
    PathPlacer,
    StartPoint,
    EndPoint,
    BuildableArea,
}

#[derive(Resource, Default)]
pub struct EditorData {
    pub path: Vec<UVec2>,
    pub start: Option<UVec2>,
    pub end: Option<UVec2>,
    pub buildable_areas: Vec<UVec2>,
    pub current_tool: EditorTool,
    pub grid_overlay: bool,
}

#[derive(Component)]
pub struct EditorPathMarker;

#[derive(Component)]
pub struct EditorToolDisplay;

#[derive(Serialize, Deserialize)]
pub struct LevelData {
    pub path: Vec<Vec<u32>>,            // Stored as [[x, y], [x, y], ...]
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
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15).into()),
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
                        if !editor_data.path.contains(&grid_pos) {
                            editor_data.path.push(grid_pos);

                            let world_pos = if let Some(map) = map.as_ref() {
                                map.grid_to_world(grid_pos)
                            } else {
                                let grid_size = Vec2::new(48.0, 48.0);
                                Vec2::new(
                                    grid_pos.x as f32 * grid_size.x + grid_size.x * 0.5,
                                    grid_pos.y as f32 * grid_size.y + grid_size.y * 0.5,
                                )
                            };

                            commands.spawn((
                                Sprite {
                                    color: Color::srgba(0.8, 0.3, 0.3, 0.7),
                                    custom_size: Some(Vec2::new(48.0, 48.0)),
                                    ..default()
                                },
                                Transform::from_translation(world_pos.extend(1.0)),
                                EditorPathMarker,
                            ));
                        }
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

fn render_editor_path(editor_data: Res<EditorData>, mut gizmos: Gizmos, map: Option<Res<Map>>) {
    let grid_size = if let Some(map) = map.as_ref() {
        map.grid_size
    } else {
        Vec2::new(48.0, 48.0)
    };

    let grid_start_x = -crate::config::WINDOW_WIDTH / 2.0 + grid_size.x / 2.0;
    let grid_start_y = crate::config::WINDOW_HEIGHT / 2.0 - grid_size.y / 2.0;

    // Draw path lines
    if editor_data.path.len() >= 2 {
        for i in 0..editor_data.path.len() - 1 {
            let start = editor_data.path[i];
            let end = editor_data.path[i + 1];

            let start_world = Vec2::new(
                grid_start_x + start.x as f32 * grid_size.x,
                grid_start_y - start.y as f32 * grid_size.y,
            );

            let end_world = Vec2::new(
                grid_start_x + end.x as f32 * grid_size.x,
                grid_start_y - end.y as f32 * grid_size.y,
            );

            gizmos.line_2d(start_world, end_world, Color::srgb(0.9, 0.3, 0.7));
        }
    }

    if editor_data.grid_overlay {
        let dimensions = if let Some(map) = map.as_ref() {
            map.dimensions
        } else {
            UVec2::new(27, 15)
        };

        let grid_size = if let Some(map) = map.as_ref() {
            map.grid_size
        } else {
            Vec2::new(48.0, 48.0)
        };

        let grid_start_pos = if let Some(map) = map.as_ref() {
            map.grid_to_world(UVec2::new(0, 0))
        } else {
            let grid_width = dimensions.x as f32 * grid_size.x;
            let grid_height = dimensions.y as f32 * grid_size.y;
            Vec2::new(-grid_width / 2.0, grid_height / 2.0)
        };

        // Draw vertical grid lines
        for x in 0..=dimensions.x {
            let x_pos = grid_start_pos.x + (x as f32 * grid_size.x);
            let start = Vec2::new(640.0, 360.0);
            let end = Vec2::new(
                x_pos,
                grid_start_pos.y - (dimensions.y as f32 * grid_size.y),
            );
            gizmos.line_2d(start, end, Color::srgba(0.5, 0.5, 0.5, 0.3));
        }

        // Draw horizontal grid lines
        for y in 0..=dimensions.y {
            let y_pos = grid_start_pos.y - (y as f32 * grid_size.y);
            let start = Vec2::new(grid_start_pos.x, y_pos);
            let end = Vec2::new(
                grid_start_pos.x + (dimensions.x as f32 * grid_size.x),
                y_pos,
            );
            gizmos.line_2d(start, end, Color::srgba(0.5, 0.5, 1.5, 0.3));
        }
    }
}

fn toggle_editor_tool(
    interaction_query: Query<(&Interaction, &EditorButton), (Changed<Interaction>, With<Button>)>,
    mut editor_data: ResMut<EditorData>,
    mut tool_text_query: Query<&mut Text, With<EditorToolDisplay>>,
) {
    for (interaction, button) in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            editor_data.current_tool = button.0.clone();

            if let Ok(mut text) = tool_text_query.get_single_mut() {
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
    let level_data = LevelData {
        path: editor_data
            .path
            .iter()
            .map(|pos| vec![pos.x, pos.y])
            .collect(),
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
            level_name: "level_1".to_string(),
            dialog_open: false,
        }
    }
}

fn handle_text_input(
    mut editor_text_input: ResMut<EditorTextInput>,
    mut text_query: Query<&mut Text, With<LevelNameInput>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if !editor_text_input.dialog_open {
        return;
    }

    for key in keyboard_input.get_just_pressed() {
        let mut name_changed = false;
        match key {
            KeyCode::Backspace => {
                if !editor_text_input.level_name.is_empty() {
                    editor_text_input.level_name.pop();
                    name_changed = true;
                    // level_name.pop();
                }
            }
            KeyCode::Space => {
                editor_text_input.level_name.push('_'); // Convert spaces to underscores
                name_changed = true;
            }

            KeyCode::KeyA => {
                editor_text_input.level_name.push('a');
                name_changed = true;
            }
            KeyCode::KeyB => {
                editor_text_input.level_name.push('b');
                name_changed = true;
            }
            KeyCode::KeyC => {
                editor_text_input.level_name.push('c');
                name_changed = true;
            }
            KeyCode::KeyD => {
                editor_text_input.level_name.push('d');
                name_changed = true;
            }
            KeyCode::KeyE => {
                editor_text_input.level_name.push('e');
                name_changed = true;
            }
            KeyCode::KeyF => {
                editor_text_input.level_name.push('f');
                name_changed = true;
            }
            KeyCode::KeyG => {
                editor_text_input.level_name.push('g');
                name_changed = true;
            }
            KeyCode::KeyH => {
                editor_text_input.level_name.push('h');
                name_changed = true;
            }
            KeyCode::KeyI => {
                editor_text_input.level_name.push('i');
                name_changed = true;
            }
            KeyCode::KeyJ => {
                editor_text_input.level_name.push('j');
                name_changed = true;
            }
            KeyCode::KeyK => {
                editor_text_input.level_name.push('k');
                name_changed = true;
            }
            KeyCode::KeyL => {
                editor_text_input.level_name.push('l');
                name_changed = true;
            }
            KeyCode::KeyM => {
                editor_text_input.level_name.push('m');
                name_changed = true;
            }
            KeyCode::KeyN => {
                editor_text_input.level_name.push('n');
                name_changed = true;
            }
            KeyCode::KeyO => {
                editor_text_input.level_name.push('o');
                name_changed = true;
            }
            KeyCode::KeyP => {
                editor_text_input.level_name.push('p');
                name_changed = true;
            }
            KeyCode::KeyQ => {
                editor_text_input.level_name.push('q');
                name_changed = true;
            }
            KeyCode::KeyR => {
                editor_text_input.level_name.push('r');
                name_changed = true;
            }
            KeyCode::KeyS => {
                editor_text_input.level_name.push('s');
                name_changed = true;
            }
            KeyCode::KeyT => {
                editor_text_input.level_name.push('t');
                name_changed = true;
            }
            KeyCode::KeyU => {
                editor_text_input.level_name.push('u');
                name_changed = true;
            }
            KeyCode::KeyV => {
                editor_text_input.level_name.push('v');
                name_changed = true;
            }
            KeyCode::KeyW => {
                editor_text_input.level_name.push('w');
                name_changed = true;
            }
            KeyCode::KeyX => {
                editor_text_input.level_name.push('x');
                name_changed = true;
            }
            KeyCode::KeyY => {
                editor_text_input.level_name.push('y');
                name_changed = true;
            }
            KeyCode::KeyZ => {
                editor_text_input.level_name.push('z');
                name_changed = true;
            }
            KeyCode::Digit0 => {
                editor_text_input.level_name.push('0');
                name_changed = true;
            }
            KeyCode::Digit1 => {
                editor_text_input.level_name.push('1');
                name_changed = true;
            }
            KeyCode::Digit2 => {
                editor_text_input.level_name.push('2');
                name_changed = true;
            }
            KeyCode::Digit3 => {
                editor_text_input.level_name.push('3');
                name_changed = true;
            }
            KeyCode::Digit4 => {
                editor_text_input.level_name.push('4');
                name_changed = true;
            }
            KeyCode::Digit5 => {
                editor_text_input.level_name.push('5');
                name_changed = true;
            }
            KeyCode::Digit6 => {
                editor_text_input.level_name.push('6');
                name_changed = true;
            }
            KeyCode::Digit7 => {
                editor_text_input.level_name.push('7');
                name_changed = true;
            }
            KeyCode::Digit8 => {
                editor_text_input.level_name.push('8');
                name_changed = true;
            }
            KeyCode::Digit9 => {
                editor_text_input.level_name.push('9');
                name_changed = true;
            }
            KeyCode::Minus => {
                editor_text_input.level_name.push('-');
                name_changed = true;
            }
            _ => {}
        }
        if name_changed {
            for mut text in text_query.iter_mut() {
                *text = Text::new(editor_text_input.level_name.clone());
            }
        }
    }

    // if level_name != editor_text_input.level_name {
    //     editor_text_input.level_name = level_name;

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
