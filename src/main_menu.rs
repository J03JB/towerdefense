use bevy::prelude::*;
use crate::game_state::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
           .add_systems(OnEnter(GameState::GameOver), setup_game_over)
           .add_systems(OnEnter(GameState::Paused), setup_pause_menu)
           .add_systems(Update, button_interactions.run_if(in_state(GameState::MainMenu)))
           .add_systems(Update, pause_menu_interactions.run_if(in_state(GameState::Paused)))
           .add_systems(Update, game_over_interactions.run_if(in_state(GameState::GameOver)))
           .add_systems(OnExit(GameState::MainMenu), cleanup_menu_ui)
           .add_systems(OnExit(GameState::Paused), cleanup_menu_ui)
           .add_systems(OnExit(GameState::GameOver), cleanup_menu_ui);
    }
}

#[derive(Component)]
enum MenuButton {
    Play,
    Quit,
    MainMenu,
    Resume,
}

#[derive(Component)]
struct MenuUI;

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.2)),
            MenuUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Tower Defense"),
                TextFont { 
                    font_size: 60.0,
                    ..default()
                },
                TextColor::WHITE,
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
            ));

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(20.0)),
                    row_gap: Val::Px(20.0),
                    ..default()
                })
                .with_children(|parent| {
                    spawn_button(parent, "Play Game", MenuButton::Play);
                    spawn_button(parent, "Quit", MenuButton::Quit);
                });
        });
}

fn setup_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            MenuUI,
        ))
        .with_children(|parent| {
            // Pause text
            parent.spawn((
                Text::new("Paused"),
                TextFont { 
                    font_size: 60.0,
                    ..default()
                },
                TextColor::WHITE,
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Buttons
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(20.0)),
                    row_gap: Val::Px(20.0),
                    ..default()
                })
                .with_children(|parent| {
                    spawn_button(parent, "Resume", MenuButton::Resume);
                    spawn_button(parent, "Main Menu", MenuButton::MainMenu);
                    spawn_button(parent, "Quit", MenuButton::Quit);
                });
        });
}

fn setup_game_over(
    mut commands: Commands,
    player_resource: Res<crate::game_state::PlayerResource>,
) {
    // Display game over screen with score
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.05, 0.1)),
            MenuUI,
        ))
        .with_children(|parent| {
            // Game Over Text
            parent.spawn((
                Text::new("Game Over"),
                TextFont { 
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0,0.0, 0.0)),
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Score Text
            parent.spawn((
                Text::new(format!("Score: {}", player_resource.score)),
                TextFont { 
                    font_size: 40.0,
                    ..default()
                },
                TextColor::WHITE,
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Button to return to main menu
            parent
                .spawn(Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                })
                .with_children(|parent| {
                    spawn_button(parent, "Main Menu", MenuButton::MainMenu);
                });
        });
}

fn spawn_button(parent: &mut ChildBuilder, text: &str, button_type: MenuButton) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            button_type,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(text),
                TextFont { 
                    font_size: 20.0,
                    ..default()
                },
                TextColor::WHITE,
            ));
        });
}

fn button_interactions(
    mut buttons: Query<(&Interaction, &MenuButton, &mut BackgroundColor), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    for (interaction, button_type, mut color) in buttons.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match button_type {
                    MenuButton::Play => {
                        next_state.set(GameState::Playing);
                    }
                    MenuButton::Quit => {
                        exit.send(bevy::app::AppExit::Success);
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.25, 0.25, 0.25));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
            }
        }
    }
}

fn pause_menu_interactions(
    mut buttons: Query<(&Interaction, &MenuButton, &mut BackgroundColor), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    for (interaction, button_type, mut color) in buttons.iter_mut() {
        if let Interaction::Pressed = *interaction {
            match button_type {
                MenuButton::Resume => {
                    next_state.set(GameState::Playing);
                }
                MenuButton::MainMenu => {
                    next_state.set(GameState::MainMenu);
                }
                MenuButton::Quit => {
                    exit.send(bevy::app::AppExit::Success);
                }
                _ => {}
            }
        }
    }
}

fn game_over_interactions(
    mut buttons: Query<(&Interaction, &MenuButton, &mut BackgroundColor), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button_type, mut color) in buttons.iter_mut() {
        if let Interaction::Pressed = *interaction {
            if let MenuButton::MainMenu = button_type {
                next_state.set(GameState::MainMenu);
            }
        }
    }
}

fn cleanup_menu_ui(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
