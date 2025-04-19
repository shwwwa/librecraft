#![allow(clippy::default_constructed_unit_structs)]
// Tells windows not to show console window on release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Unsafe code violates one of design goals and used only in crates.
#![forbid(unsafe_code)]

use bevy::prelude::*;
use bevy::window::{Monitor, PrimaryMonitor};
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

#[cfg(not(debug_assertions))]
use dirs::config_dir;

#[cfg(debug_assertions)]
use consts::DEBUG_SETTINGS_PATH;
#[cfg(debug_assertions)]
use std::str::FromStr;
use std::time::Duration;

use consts::{FIXED_TIME_CLOCK, MIN_HEIGHT, MIN_WIDTH, TITLE, VERSION};

use game::GamePlugin;
use settings::SettingsPath;
use splash::SplashPlugin;

use std::path::PathBuf;

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
                            position: WindowPosition::Centered(MonitorSelection::Primary),
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

    /* Panic if no handle to cwd in debug mode. */
    #[cfg(debug_assertions)]
    let mut settings_path: PathBuf = PathBuf::from_str(DEBUG_SETTINGS_PATH).unwrap();
    /* Panic if no handle to config dir in release mode.*/
    #[cfg(not(debug_assertions))]
    let mut settings_path: PathBuf = config_dir().unwrap();
    #[cfg(not(debug_assertions))]
    settings_path.push(TITLE);

    settings_path.push("settings");
    settings_path.set_extension("toml");

    app.add_plugins(NecessaryPlugins)
	.add_systems(
	    PreStartup,
	    |assets: Res<AssetServer>, mut window: ResMut<WindowUtils>| {
                window.window_icon = Some(assets.load("icon/icon512.png"));
	    },
        )
	.add_systems(Update, limit_fps)
        .insert_resource(SettingsPath {
            path: settings_path,
            save_settings: false,
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

/** Limits fps ["refresh rate", "off", "30 fps"] */
fn limit_fps(
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
    query_monitor: Query<(Entity, &Monitor, Has<PrimaryMonitor>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        use bevy_framepace::Limiter;
	let monitor = query_monitor.single().1;
	let hz : f64 = (monitor.refresh_rate_millihertz.unwrap_or(0).div_ceil(10000) * 10) as f64;
	
        settings.limiter = match settings.limiter {
	    Limiter::Auto => Limiter::Off,
            Limiter::Off => Limiter::from_framerate(30.0),
            Limiter::Manual(fps) => {
		// Attempt to fix 180 fps bug. Doesnt work.
		if fps != Duration::from_secs_f64(1.0 / hz){
		    Limiter::from_framerate(hz)
		}
		else {
		    Limiter::Off
		}
	    }
        };

	info!("{}",settings.limiter);
    }
}
