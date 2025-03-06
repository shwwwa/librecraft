#![allow(clippy::default_constructed_unit_structs)]

use bevy::prelude::*;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, window::PresentMode};

mod debug;
mod ui;

use crate::debug::*;
use crate::hotbar::setup_hotbar;
use crate::crosshair::setup_crosshair;
use crate::ui::{hud::*, GUIScale};

pub const PROTOCOL_VERSION: u32 = 758;

pub fn main() {
    App::new()
	.insert_resource(GUIScale::Scale(2))
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
        .add_systems(Startup, (setup_debug_hud, setup_hotbar, setup_crosshair, setup_camera))
        .add_systems(Update, toggle_debug_hud)
        .add_systems(FixedUpdate, update_fps_text)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Msaa::Off,
    ));
}
