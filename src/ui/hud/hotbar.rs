use bevy::prelude::*;
use bevy::sprite::Anchor;

// Hotbar system.
// Requires a running camera.
pub fn setup_hotbar(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Sprite {
        image: asset_server.load("hotbar.png"),
        anchor: Anchor::BottomCenter,
        custom_size: Some(Vec2::new(364.0, 44.0)), // 182, 22
        ..Default::default()
    },
    Transform::from_xyz(100., 0., 0.)));
}
