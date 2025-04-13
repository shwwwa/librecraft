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
            .add_group(
                DefaultPlugins
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
                    .set(ImagePlugin::default_nearest()),
            )
            .add(FrameTimeDiagnosticsPlugin)
            .add(SystemInformationDiagnosticsPlugin)
            .add(bevy_framepace::FramepacePlugin)
    }
}

mod debug;
mod music;
mod settings;
mod ui;

use crate::debug::*;
use crate::music::{setup_soundtrack, fade_in, fade_out, change_track};
use crate::crosshair::{setup_crosshair, update_crosshair};
use crate::hotbar::{
    HotbarSelectionChanged, setup_hotbar, update_hotbar, update_hotbar_selection,
    update_hotbar_selector,
};
use crate::settings::Settings;
use crate::ui::{
    GUIMode, GUIModeChanged, GUIScale, GUIScaleChanged, change_gui_mode, change_gui_scale,
    handle_mouse, hud::*,
};

pub const PROTOCOL_VERSION: u32 = 758;

pub fn main() {
    let settings = settings::read_settings("assets/settings.toml").unwrap_or_default();

    App::new()
	.insert_resource(Time::<Fixed>::from_hz(50.0))
        .insert_resource(GUIScale::Auto)
        .insert_resource(GUIMode::Opened)
        .insert_resource(settings)
        .add_plugins(NecessaryPlugins)
        .add_event::<GUIScaleChanged>()
        .add_event::<GUIModeChanged>()
        .add_event::<HotbarSelectionChanged>()
        .add_systems(
            Startup,
            (
                setup_debug_hud,
                setup_hotbar,
                setup_crosshair,
                setup_camera,
                setup_soundtrack,
            ),
        )
	.add_systems(
            FixedUpdate,
            (update_fps_text, update_display_text, update_focus_text),
        )
        .add_systems(
            Update,
            (
                toggle_debug_hud,
                change_gui_scale,
                change_gui_mode,
                limit_fps,
                handle_mouse,
            ),
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
            (fade_in, fade_out, change_track),
        )
        .run();
}

fn setup_camera(mut commands: Commands, settings: Res<Settings>) {
    commands.spawn((
        Camera2d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Msaa::Off,
    ));

    info!("seed: {}", settings.seed);
}
