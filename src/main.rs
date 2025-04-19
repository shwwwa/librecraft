#![allow(clippy::default_constructed_unit_structs)]
// Tells windows not to show console window on release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Unsafe code violates one of design goals and used only in crates.
#![forbid(unsafe_code)]

use bevy::prelude::*;
use bevy::{
    app::PluginGroupBuilder,
    diagnostic::{FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin},
    window::PresentMode,
};

#[cfg(feature = "embed-assets")]
use bevy_embedded_assets::{self, EmbeddedAssetPlugin};
use bevy_framepace::FramepacePlugin;
use bevy_window_utils::{WindowUtils, WindowUtilsPlugin};

/** Librecraft's main hard-coded constants. */
pub mod consts {
    /** Fixed time clock - leave it at 50 hz. */
    pub const FIXED_TIME_CLOCK: f64 = 50.;
    /** Minecraft protocol version that we are trying to support. */
    pub const PROTOCOL_VERSION: u32 = 758;
    /** Path to the settings file when on debug. */
    pub const DEBUG_SETTINGS_PATH: &str = "./";
    /** Title of the main program. */
    pub const TITLE: &str = env!("CARGO_PKG_NAME");
    /** Version of the main program. */
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    /** Librecraft's minimum resolution width. */
    pub const MIN_WIDTH: f32 = 320.;
    /** Librecraft's minimum resolution height. */
    pub const MIN_HEIGHT: f32 = 240.;
}

/** Game's logic resources. */
pub mod game;
/** All that belongs to GUI from debug info to hud. (cross-state)*/
pub mod gui;
/** Responsible for audio. (then audio->music, audio->sound) */
pub mod music;
/** Reads and stores user owned settings. */
pub mod settings;
/** Adds splash screen to app (independent). */
pub mod splash;

#[cfg(debug_assertions)]
use consts::DEBUG_SETTINGS_PATH;
use consts::{FIXED_TIME_CLOCK, MIN_HEIGHT, MIN_WIDTH, TITLE, VERSION};

use game::GamePlugin;
use settings::SettingsPath;
use splash::SplashPlugin;

use std::path::PathBuf;
use std::str::FromStr;

/** Necessary plugins, responsible for generic app functions like windowing or asset packaging (prestartup). */
struct NecessaryPlugins;

impl PluginGroup for NecessaryPlugins {
    fn build(self) -> PluginGroupBuilder {
        #[allow(unused_mut)]
        let mut builder = PluginGroupBuilder::start::<Self>();

        // Warning: Must be loaded before DefaultPlugins.
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
                            // [`FramepacePlugin`] handles it.
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

/** Main game state (alike scenes) */
#[derive(States, Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Splash,
    InGame,
}

/** Main entry of the program. */
pub fn main() {
    let mut app = App::new();

    /* Panic if we don't have a cwd handle */
    #[cfg(debug_assertions)]
    let mut settings_path: PathBuf = PathBuf::from_str(DEBUG_SETTINGS_PATH).unwrap();
    /* Panic if no handle to config dir in release mode.*/
    #[cfg(not(debug_assertions))]
    let mut settings_path: PathBuf = config_dir().unwrap().push(TITLE);

    settings_path.push("settings");
    settings_path.set_extension("toml");

    app.add_plugins(NecessaryPlugins)
        // todo(bevy 0.16.0): replace it with nec plug defs
        .add_systems(
            PreStartup,
            |assets: Res<AssetServer>, mut window: ResMut<WindowUtils>| {
                window.window_icon = Some(assets.load("icon/icon512.png"));
            },
        )
        .insert_resource(SettingsPath {
            path: settings_path,
        })
        .insert_resource(Time::<Fixed>::from_hz(FIXED_TIME_CLOCK))
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_plugins((
            SplashPlugin {
                state: GameState::Splash,
            },
            GamePlugin {
                state: GameState::InGame,
            },
        ))
        .run();
}

/** Setups camera for [`App`] to use. */
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
