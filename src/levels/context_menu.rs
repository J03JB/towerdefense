use bevy::color::palettes::basic;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn context_menu(
    mut commands: Commands,
    query_window: Query<&Window, With<PrimaryWindow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    let window = query_window.single();

    if let Some(cursor_translate) = window.cursor_position() {
        let relative_position = (
            (cursor_translate.y / window.height()).abs(),
            (cursor_translate.x / window.width()).abs(),
        );

        if mouse_button.just_pressed(MouseButton::Right) {
            let mut style = Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(relative_position.0.abs() * 100.),
                left: Val::Percent(relative_position.1.abs() * 100.),

                width: Val::Percent(15.0),
                height: Val::Percent(15.0),
                ..default()
            };

            if relative_position.1.abs() > 0.9 {
                style.left = Val::Auto;
                style.right = Val::Percent(100. - relative_position.1.abs() * 100.);
            }
            if relative_position.0.abs() > 0.9 {
                style.top = Val::Auto;
                style.bottom = Val::Percent(100. - relative_position.0.abs() * 100.);
            }

            commands.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(relative_position.0.abs() * 100.),
                    left: Val::Percent(relative_position.1.abs() * 100.),
                    width: Val::Percent(15.0),
                    height: Val::Percent(15.0),
                    ..default()
                },
                BackgroundColor(basic::BLUE.into()),
            ));
        }
    }
}
