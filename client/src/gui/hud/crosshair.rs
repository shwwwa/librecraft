use bevy::prelude::*;

use crate::gui::{GUIScale, GUIScaleChanged, gui_scale_to_float};

#[derive(Component)]
pub struct HorizontalLine;
#[derive(Component)]
pub struct VerticalLine;

pub fn setup_crosshair(mut commands: Commands, gui_scale: Res<GUIScale>) {
    let scale: f32 = gui_scale_to_float(*gui_scale);

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
            parent.spawn((
                HorizontalLine,
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(10.0 * scale),
                    height: Val::Px(1.0 * scale),
                    left: Val::Px(-5.0 * scale),
                    top: Val::Px(-0.5 * scale),
                    ..default()
                },
                BackgroundColor(Color::WHITE),
            ));

            parent.spawn((
                VerticalLine,
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(1.0 * scale),
                    height: Val::Px(10.0 * scale),
                    left: Val::Px(-0.5 * scale),
                    top: Val::Px(-5.0 * scale),
                    ..default()
                },
                BackgroundColor(Color::WHITE),
            ));
        });
}

pub fn update_crosshair(
    mut gui_scale_reader: EventReader<GUIScaleChanged>,
    mut query_vertical: Query<&mut Node, (With<VerticalLine>, Without<HorizontalLine>)>,
    mut query_horizontal: Query<&mut Node, (With<HorizontalLine>, Without<VerticalLine>)>,
) {
    for event in gui_scale_reader.read() {
        let scale: f32 = gui_scale_to_float(event.gui_scale);
        for mut node_v in query_vertical.iter_mut() {
            node_v.width = Val::Px(10.0 * scale);
            node_v.height = Val::Px(1.0 * scale);
            node_v.left = Val::Px(-5.0 * scale);
            node_v.top = Val::Px(-0.5 * scale);
        }

        for mut node_h in query_horizontal.iter_mut() {
            node_h.width = Val::Px(1.0 * scale);
            node_h.height = Val::Px(10.0 * scale);
            node_h.left = Val::Px(-0.5 * scale);
            node_h.top = Val::Px(-5.0 * scale);
        }
    }
}
