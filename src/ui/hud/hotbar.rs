use bevy::prelude::*;
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
	Node {
	    display: Display::Flex,
	    position_type: PositionType::Absolute,
	    bottom: Val::Px(40.),
	    width: Val::Px(HOTBAR_WIDTH * scale),
	    height: Val::Px(HOTBAR_HEIGHT * scale),
	    padding: UiRect::ZERO,
	    border: UiRect::ZERO,
	    margin: UiRect::all(Val::Auto),
	    ..default()
	},
        ImageNode {
            image: asset_server.load("hotbar.png"), 
            ..Default::default()
        },
	GlobalZIndex(1),
    )).with_child((
        HotbarSelection,
        ImageNode {
            image: asset_server.load("hotbar_selection.png"),
            ..Default::default()
        },
	Node {
	    display: Display::Flex,
	    position_type: PositionType::Relative,
	    padding: UiRect::ZERO,
	    border: UiRect::ZERO,
	    margin: UiRect::all(Val::Auto),
	    ..default()
	},
	GlobalZIndex(1),
    ));
}

pub fn update_hotbar(
    mut gui_scale_events: EventReader<GUIScaleChanged>,
    mut query: Query<&mut Node, With<Hotbar>>,
) {
    for event in gui_scale_events.read() {
        for mut node in query.iter_mut() {
            let scale: f32 = gui_scale_to_float(event.gui_scale);
            node.width = Val::Px(HOTBAR_WIDTH * scale);
	    node.height = Val::Px(HOTBAR_HEIGHT * scale);
        }
    }
}

pub fn update_hotbar_selection(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Hotbar, (With<Hotbar>, Without<HotbarSelection>)>,
    mut gui_scale_events: EventReader<GUIScaleChanged>,
    mut query_selection: Query<&mut Node, (With<HotbarSelection>, Without<Hotbar>)>,
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
	    
	    for mut node in query_selection.iter_mut() {
		// todo: proper placing (not formula one)
		node.margin.left = Val::Percent(((hotbar.selected - 1) as f32) * (100./((MAX_HOTBAR_SLOTS) as f32)))
	    }
    
        }
    }
    for event in gui_scale_events.read() {
        for mut node in query_selection.iter_mut() {
            let scale: f32 = gui_scale_to_float(event.gui_scale);
            node.width = Val::Px(HOTBAR_SELECTION_WIDTH * scale);
	    node.height = Val::Px(HOTBAR_SELECTION_HEIGHT * scale);
        }
    }
}
