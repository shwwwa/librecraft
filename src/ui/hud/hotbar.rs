use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::ui::gui_scale_to_float;
use crate::{GUIScale, GUIScaleChanged};

pub const MAX_HOTBAR_SLOTS : u8 = 9;
pub const HOTBAR_WIDTH : f32 = 182.;
pub const HOTBAR_HEIGHT : f32 = 22.;
pub const HOTBAR_SELECTION_WIDTH : f32 = 24.;
pub const HOTBAR_SELECTION_HEIGHT : f32 = 23.;

#[derive(Component)]
pub struct Hotbar {
    pub selected: u32,
}

#[derive(Component)]
pub struct HotbarSelection;

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
            custom_size: Some(Vec2::new(HOTBAR_WIDTH * scale, HOTBAR_HEIGHT * scale)), // original_size * gui_scale; hard-coded to punish changing size of assets
            ..Default::default()
        },
        Transform::from_xyz(0., 0., 0.),
    )).with_child((
        HotbarSelection,
        Sprite {
            image: asset_server.load("hotbar_selection.png"),
            anchor: Anchor::BottomCenter,
	    // original_size * gui_scale; hard-coded to punish changes in the size of assets
            custom_size: Some(Vec2::new(HOTBAR_SELECTION_WIDTH * scale, HOTBAR_SELECTION_HEIGHT * scale)),
            ..Default::default()
        },
        Transform::from_xyz(0., 0., 0.),
    ));
}

pub fn update_hotbar(
    mut gui_scale_events: EventReader<GUIScaleChanged>,
    mut query: Query<&mut Sprite, With<Hotbar>>,
) {
    for event in gui_scale_events.read() {
        for mut sprite in query.iter_mut() {
            let scale: f32 = gui_scale_to_float(event.gui_scale);
            sprite.custom_size = Some(Vec2::new(HOTBAR_WIDTH * scale, HOTBAR_HEIGHT * scale));
        }
    }
}

pub fn update_hotbar_selection(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Hotbar, (With<Hotbar>, Without<HotbarSelection>)>,
    mut gui_scale_events: EventReader<GUIScaleChanged>,
    mut query_selection: Query<&mut Sprite, (With<HotbarSelection>, Without<Hotbar>)>,
) {
    // todo: enum to int conversion?
    if keys.any_pressed([
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
        KeyCode::Digit9,
    ]) {
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

    for event in gui_scale_events.read() {
        for mut sprite in query_selection.iter_mut() {
            let scale: f32 = gui_scale_to_float(event.gui_scale);
            sprite.custom_size = Some(Vec2::new(HOTBAR_SELECTION_WIDTH * scale, HOTBAR_SELECTION_HEIGHT * scale));
        }
    }
}
