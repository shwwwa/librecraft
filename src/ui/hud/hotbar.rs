use bevy::prelude::*;

// Hotbar system.
// Requires a running camera.
pub fn setup_hotbar(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
	Sprite {
	    image: asset_server.load("hotbar.png"),
	    ..Default::default()
	},
	Transform::from_xyz(100.,0.,0.),
    ));
}
