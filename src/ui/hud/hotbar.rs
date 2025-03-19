use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::ui::gui_scale_to_float;
use crate::{GUIScale, GUIScaleChanged};

pub const MAX_HOTBAR_SLOTS: u8 = 9;

#[derive(Component)]
pub struct Hotbar {
    pub selected: u32,
}

// Hotbar system.
// Requires a running camera.
pub fn setup_hotbar(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gui_scale: Res<GUIScale>,
) {
    let scale: f32 = gui_scale_to_float(*gui_scale);

    commands.spawn((
        Hotbar { selected: 1 },
        Sprite {
            image: asset_server.load("hotbar.png"),
            anchor: Anchor::BottomCenter,
            custom_size: Some(Vec2::new(182. * scale, 22. * scale)), // original_size * gui_scale; hard-coded to punish changing size of assets
            ..Default::default()
        },
        Transform::from_xyz(100., 0., 0.),
    ));
}

pub fn update_hotbar(
    mut gui_scale_events: EventReader<GUIScaleChanged>,
    mut query: Query<&mut Sprite, With<Hotbar>>,
) {
    for event in gui_scale_events.read() {
        for mut sprite in query.iter_mut() {
            let scale: f32 = gui_scale_to_float(event.gui_scale);
            sprite.custom_size = Some(Vec2::new(182. * scale, 22. * scale));
        }
    }
}

pub fn update_hotbar_selection(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Hotbar, With<Hotbar>>,
) {
    // todo: enum to int conversion
    for mut hotbar in query.iter_mut() {
        if keys.pressed(KeyCode::Digit1) {
            hotbar.selected = 1;
        }
        if keys.pressed(KeyCode::Digit2) {
            hotbar.selected = 2;
        }
        if keys.pressed(KeyCode::Digit3) {
            hotbar.selected = 3;
        }
        if keys.pressed(KeyCode::Digit4) {
            hotbar.selected = 4;
        }
        if keys.pressed(KeyCode::Digit5) {
            hotbar.selected = 5;
        }
        if keys.pressed(KeyCode::Digit6) {
            hotbar.selected = 6;
        }
        if keys.pressed(KeyCode::Digit7) {
            hotbar.selected = 7;
        }
        if keys.pressed(KeyCode::Digit8) {
            hotbar.selected = 8;
        }
        if keys.pressed(KeyCode::Digit9) {
            hotbar.selected = 9;
        }
    }
}
