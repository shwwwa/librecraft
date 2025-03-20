#![allow(clippy::default_constructed_unit_structs)]

use bevy::prelude::*;
use bevy::{
    app::PluginGroupBuilder,
    diagnostic::{FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin},
    window::PresentMode,
};

struct NecessaryPlugins;

impl PluginGroup for NecessaryPlugins {
    fn build(self) -> PluginGroupBuilder {
	PluginGroupBuilder::start::<Self>()
	    .add_group(DefaultPlugins
                       .set(WindowPlugin {
			   primary_window: Some(Window {
                               resize_constraints: WindowResizeConstraints {
				   min_width: 480.,
				   min_height: 360.,
				   ..default()
                               },
                               title: "librecraft".into(),
                               present_mode: PresentMode::AutoNoVsync,
                               ..default()
			   }),
			   ..default()
                       })
                 .set(ImagePlugin::default_nearest()))
	    .add(FrameTimeDiagnosticsPlugin)
	    .add(SystemInformationDiagnosticsPlugin)
	    .add(bevy_framepace::FramepacePlugin)
    }
}

mod debug;
mod music;
mod ui;

use crate::crosshair::{setup_crosshair, update_crosshair};
use crate::debug::*;
use crate::hotbar::{
    HotbarSelectionChanged, setup_hotbar, update_hotbar, update_hotbar_selection,
    update_hotbar_selector,
};
use crate::ui::{GUIScale, GUIScaleChanged, change_gui_scale, hud::*};

pub const PROTOCOL_VERSION: u32 = 758;

pub fn main() {
    App::new()
        .insert_resource(GUIScale::Auto)
        .insert_resource(Time::<Fixed>::from_hz(50.0))
        .add_plugins(NecessaryPlugins)
        .add_event::<GUIScaleChanged>()
        .add_event::<HotbarSelectionChanged>()
        .add_systems(
            Startup,
            (
                setup_debug_hud,
                setup_hotbar,
                setup_crosshair,
                setup_camera,
                music::setup_soundtrack,
            ),
        )
        .add_systems(Update, (toggle_debug_hud, change_gui_scale, limit_fps))
        .add_systems(
            FixedUpdate,
            (update_fps_text, update_display_text, update_focus_text),
        )
        .add_systems(
            Update,
            (
                update_hotbar,
                update_hotbar_selection,
                update_hotbar_selector,
                update_crosshair,
            ),
        )
        .add_systems(
            Update,
            (music::fade_in, music::fade_out, music::change_track),
        )
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
