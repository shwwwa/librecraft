use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::{GUIScale, GUIScaleChanged};
use crate::ui::gui_scale_to_float;

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
    let scale : f32 = gui_scale_to_float(*gui_scale);

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
	    let scale : f32 = gui_scale_to_float(event.gui_scale);
	    sprite.custom_size = Some(Vec2::new(182. * scale, 22. * scale));
	}
    }
}
