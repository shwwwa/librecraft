use bevy::prelude::*;
use bevy::{window::PresentMode, diagnostic::FrameTimeDiagnosticsPlugin, math::vec3};

mod debug;
mod ui;

use crate::debug::*;
use crate::ui::hud::*;
use crate::hotbar::setup_hotbar;

pub const PROTOCOL_VERSION : u32 = 758;

pub fn main() {
    App::new()
	.add_plugins(FrameTimeDiagnosticsPlugin)
	.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "libcraft".into(),
		present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
	.insert_resource(Time::<Fixed>::from_hz(50.0))
	.add_systems(
	    Startup, (setup_hud, setup_hotbar, spawn_camera))
	.add_systems(
	    Update,
	    (toggle_hud))
	.add_systems(
	    FixedUpdate,
	    (update_fps_text)
	)
	.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera3d::default(),
		    Transform::from_translation(vec3(-1.25, 2.25, 4.5)).looking_at(Vec3::ZERO, Vec3::Y),
		    Camera {
			hdr: true,
			..default()
		    },
		    Msaa::Off,
    ));
	
}
