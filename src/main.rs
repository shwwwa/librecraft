#![allow(clippy::default_constructed_unit_structs)]
// Tells windows not to show console window on release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Unsafe code belongs violates one of design goals and used only in crates.
#![forbid(unsafe_code)]

use bevy::prelude::*;
use bevy::render::view::screenshot::{Capturing, Screenshot, save_to_disk};
use bevy::window::SystemCursorIcon;
use bevy::winit::cursor::CursorIcon;
use bevy::{
    app::PluginGroupBuilder,
    diagnostic::{FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin},
    window::PresentMode,
};

#[cfg(feature = "embed-assets")]
use bevy_embedded_assets::{self, EmbeddedAssetPlugin};
use bevy_framepace::FramepacePlugin;
use bevy_window_utils::{WindowUtils, WindowUtilsPlugin};

/** Minecraft protocol version that we are trying to support. */
pub const PROTOCOL_VERSION: u32 = 758;
/** Title of the main program. */
pub const TITLE: &str = env!("CARGO_PKG_NAME");
/** Version of the main program. */
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/** Minecraft's minimum resolution width. */
pub const MIN_WIDTH: f32 = 320.;
/** Minecraft's minimum resolution height. */
pub const MIN_HEIGHT: f32 = 240.;

/** Necessary plugins, responsible for generic app functions. */
struct NecessaryPlugins;

impl PluginGroup for NecessaryPlugins {
    fn build(self) -> PluginGroupBuilder {
        #[allow(unused_mut)]
        let mut builder = PluginGroupBuilder::start::<Self>();

        // Must be loaded before DefaultPlugins
        #[cfg(feature = "embed-assets")]
        {
            builder = builder.add(EmbeddedAssetPlugin {
                mode: bevy_embedded_assets::PluginMode::ReplaceDefault,
            });
        }
        builder
            .add_group(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            resize_constraints: WindowResizeConstraints {
                                min_width: MIN_WIDTH,
                                min_height: MIN_HEIGHT,
                                ..default()
                            },
                            title: TITLE.to_string() + " v." + VERSION,
                            // Instead of vsync we limit fps by framepace.
                            present_mode: PresentMode::AutoNoVsync,
                            ..default()
                        }),
                        ..default()
                    })
                    .set(ImagePlugin::default_nearest()),
            )
            .add(WindowUtilsPlugin)
            .add(FrameTimeDiagnosticsPlugin)
            .add(SystemInformationDiagnosticsPlugin)
            .add(FramepacePlugin)
    }
}

mod debug;
mod gui;
mod music;
mod player;
mod settings;

use crate::crosshair::{setup_crosshair, update_crosshair};
use crate::debug::*;
use crate::gui::{
    GUIMode, GUIModeChanged, GUIScale, GUIScaleChanged, change_gui_mode, change_gui_scale,
    handle_mouse, hud::*, menu::*, update_gui_scale,
};
use crate::hotbar::{
    HotbarSelectionChanged, setup_hotbar, update_hotbar, update_hotbar_selection,
    update_hotbar_selector,
};
use crate::music::{change_track, fade_in, fade_out, mute_music_on_focus, setup_soundtrack};
use crate::player::*;
use crate::settings::*;

/** Main entry of the program. */
pub fn main() {
    let mut app = App::new();
    app.insert_resource(Time::<Fixed>::from_hz(50.0))
        .insert_resource(GUIMode::Opened)
        // Init default versions of resources. Later on startup they should be overwritten.
        .init_resource::<Settings>()
        .init_resource::<Player>()
        .add_plugins(NecessaryPlugins)
        .add_event::<GUIScaleChanged>()
        .add_event::<GUIModeChanged>()
        .add_event::<HotbarSelectionChanged>()
        .add_systems(
            PreStartup,
            (setup_debug_hud, setup_settings, setup_player_data),
        )
        .add_systems(
            PreStartup,
            |assets: Res<AssetServer>, mut window: ResMut<WindowUtils>| {
                window.window_icon = Some(assets.load("icon/icon512.png"));
            },
        )
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_pause_menu,
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
                change_fullscreen,
                screenshot,
                save_screenshot,
                update_gui_scale,
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
        .add_systems(Update, (fade_in, fade_out, change_track))
        .add_systems(Update, mute_music_on_focus.run_if(is_mute_on_lost_focus))
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

fn screenshot(mut commands: Commands, input: Res<ButtonInput<KeyCode>>, mut counter: Local<u32>) {
    if input.just_pressed(KeyCode::F2) {
        let path = format!("./screenshot-{}.png", *counter);
        *counter += 1;
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }
}

fn save_screenshot(
    mut commands: Commands,
    screenshot_saving: Query<Entity, With<Capturing>>,
    query_window: Query<Entity, With<Window>>,
) {
    let Ok(window) = query_window.get_single() else {
        warn!("Couldn't save screenshot.");
        return;
    };

    match screenshot_saving.iter().count() {
        0 => {
            commands.entity(window).remove::<CursorIcon>();
        }
        x if x > 0 => {
            commands
                .entity(window)
                .insert(CursorIcon::from(SystemCursorIcon::Progress));
        }
        _ => {}
    }
}
