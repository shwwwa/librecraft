#![allow(clippy::default_constructed_unit_structs)]
use bevy::prelude::*;
use bevy::{
    app::PluginGroupBuilder,
    diagnostic::{FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin},
    window::PresentMode,
};
use music::mute_music_on_focus;

/** Necessary plugins, responsible for generic app functions. */
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
mod player;
mod settings;
mod ui;

use crate::crosshair::{setup_crosshair, update_crosshair};
use crate::debug::*;
use crate::hotbar::{
    HotbarSelectionChanged, setup_hotbar, update_hotbar, update_hotbar_selection,
    update_hotbar_selector,
};
use crate::music::{change_track, fade_in, fade_out, setup_soundtrack};
use crate::player::*;
use crate::settings::*;
use crate::ui::{
    GUIMode, GUIModeChanged, GUIScale, GUIScaleChanged, change_gui_mode, change_gui_scale,
    handle_mouse, hud::*,
};

/** Minecraft protocol version that we are trying to support. */
pub const PROTOCOL_VERSION: u32 = 758;

/** Main entry of the program. */
pub fn main() {
    let mut app = App::new();
    app.insert_resource(Time::<Fixed>::from_hz(50.0))
        .insert_resource(GUIScale::Auto)
        .insert_resource(GUIMode::Opened)
        .init_resource::<Settings>()
        .init_resource::<Player>()
        .add_plugins(NecessaryPlugins)
        .add_event::<GUIScaleChanged>()
        .add_event::<GUIModeChanged>()
        .add_event::<HotbarSelectionChanged>()
        .add_systems(
            Startup,
            (
                setup_debug_hud,
                setup_settings,
                setup_player_data,
                setup_camera,
                setup_hotbar,
                setup_crosshair,
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
        .add_systems(Update, (fade_in, fade_out, change_track));
    app.add_systems(Update, mute_music_on_focus.run_if(is_mute_on_lost_focus))
        .run();
}

/** Setups camera for game to use. */
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
