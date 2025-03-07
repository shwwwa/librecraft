use crate::GUIScale;
use bevy::prelude::*;

pub fn setup_crosshair(mut commands: Commands, gui_scale: Res<GUIScale>) {
    let scale: f32 = match *gui_scale {
        GUIScale::Auto => 2 as f32,
        GUIScale::Scale(x) => x as f32,
    };

    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            margin: UiRect {
                left: Val::Auto,
                right: Val::Auto,
                top: Val::Auto,
                bottom: Val::Auto,
            },
            ..default()
        },))
        .with_children(|parent| {
            // Horizontal line (horizontal bar of the cross)
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(20.0 * scale),
                    height: Val::Px(2.0 * scale),
                    left: Val::Px(-10.0 * scale),
                    top: Val::Px(-1.0 * scale),
                    ..Default::default()
                },
                BackgroundColor(Color::WHITE),
            ));

            // Vertical line (vertical bar of the cross)
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(2.0 * scale),
                    height: Val::Px(20.0 * scale),
                    left: Val::Px(-1.0 * scale),
                    top: Val::Px(-10.0 * scale),
                    ..Default::default()
                },
                BackgroundColor(Color::WHITE),
            ));
        });
}
