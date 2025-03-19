use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::GUIScale;

// Hotbar system.
// Requires a running camera.
pub fn setup_hotbar(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gui_scale: Res<GUIScale>,
) {
    let scale: f32 = match *gui_scale {
        GUIScale::Auto => 2_f32,
        GUIScale::Scale(x) => x as f32,
    };

    commands.spawn((
        Sprite {
            image: asset_server.load("hotbar.png"),
            anchor: Anchor::BottomCenter,
            custom_size: Some(Vec2::new(182. * scale, 22. * scale)), // original_size * gui_scale; hard-coded to punish changing size of assets
            ..Default::default()
        },
        Transform::from_xyz(100., 0., 0.),
    ));
}
