#![allow(clippy::default_constructed_unit_structs)]
// Warns of some basic pitfalls, but basically too pedantic.
// #![warn(clippy::pedantic)]
#![feature(stmt_expr_attributes)]
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
    macro_rules! title {
        () => {
            env!("CARGO_PKG_NAME")
        };
    }
    /** Version of the main program. */
    macro_rules! version {
        () => {
            env!("CARGO_PKG_VERSION")
        };
    }
    /** Version string. */
    macro_rules! version_string {
        () => {
            concat!(crate::consts::title!(), " v.", crate::consts::version!())
        };
    }
    /** About (developer and version string) */
    #[allow(unused_macros)]
    macro_rules! about {
        () => {
            concat!(crate::consts::version_string!(), " | by caffidev")
        };
    }
    pub(crate) use about;
    pub(crate) use title;
    pub(crate) use version;
    pub(crate) use version_string;

    /** Librecraft's minimum resolution width. */
    pub const MIN_WIDTH: f32 = 320.;
    /** Librecraft's minimum resolution height. */
    pub const MIN_HEIGHT: f32 = 240.;
}

/** Path to all assets of the librecraft. */
pub mod assets {
    use bevy::prelude::*;
    
    /** todo: have an ability to use minecraft's font as custom font to have some identity */
    #[derive(Debug, Clone, Resource)]
    pub struct RuntimeAsset {
	pub font_path: String,
    }

    pub const FONT_PATH: &str = "fonts/FiraMonoRegular.ttf";
    pub const MINECRAFT_FONT_PATH: &str = "fonts/MinecraftRegular.otf";
    
    pub const ICON_PATH: &str = "icon/icon512.png";
    pub const SPLASH_PATH: &str = "icon/logo-highres.png";

    pub const CALM1_PATH: &str = "music/calm1.ogg";
    pub const CALM2_PATH: &str = "music/calm2.ogg";
    pub const CALM3_PATH: &str = "music/calm3.ogg";

    pub const HAL1_PATH: &str = "music/hal1.ogg";
    pub const HAL2_PATH: &str = "music/hal2.ogg";
    pub const HAL3_PATH: &str = "music/hal3.ogg";
    pub const HAL4_PATH: &str = "music/hal4.ogg";

    pub const NUANCE1_PATH: &str = "music/nuance1.ogg";
    pub const NUANCE2_PATH: &str = "music/nuance2.ogg";

    pub const PIANO1_PATH: &str = "music/piano1.ogg";
    pub const PIANO2_PATH: &str = "music/piano2.ogg";
    pub const PIANO3_PATH: &str = "music/piano3.ogg";

    pub const HOTBAR_PATH: &str = "hotbar.png";
    pub const HOTBAR_SELECTION_PATH: &str = "hotbar_selection.png";
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

use game::GamePlugin;
use settings::SettingsPath;
use splash::SplashPlugin;
#[cfg(debug_assertions)]
use consts::DEBUG_SETTINGS_PATH;
use consts::{FIXED_TIME_CLOCK, MIN_HEIGHT, MIN_WIDTH};

use std::path::PathBuf;
use std::time::Duration;
#[cfg(debug_assertions)]
use std::str::FromStr;

/** Necessary plugins, responsible for generic app functions like windowing or asset packaging (prestartup). */
struct NecessaryPlugins;

impl PluginGroup for NecessaryPlugins {
    fn build(self) -> PluginGroupBuilder {
        #[allow(unused_mut)]
        let mut builder = PluginGroupBuilder::start::<Self>();

        // Warning: must be loaded before DefaultPlugins.
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
                            title: consts::version_string!().to_string(),
                            // [`FramepacePlugin`] handles it.
                            present_mode: PresentMode::AutoNoVsync,
                            ..default()
                        }),
                        ..default()
                    })
                    .set(LogPlugin {
			filter: "info,wgpu_core=warn,wgpu_hal=warn,librecraft=debug".into(),
			level: bevy::log::Level::DEBUG,
		    })
                    .set(ImagePlugin::default_nearest()),
            )
            .add(WindowUtilsPlugin::default())
            .add(FrameTimeDiagnosticsPlugin::default())
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
    let mut settings_path: PathBuf;
    
    /* Panic if no handle to cwd in debug mode. */
    #[cfg(debug_assertions)]
    {
	settings_path = PathBuf::from_str(DEBUG_SETTINGS_PATH).unwrap();
    }
    
    /* Panic if no handle to config dir in release mode.*/
    #[cfg(not(debug_assertions))]
    {
	settings_path = config_dir().unwrap().push(consts::title!());
    }
    
    settings_path.push("settings");
    settings_path.set_extension("toml");
    
    app.add_plugins(NecessaryPlugins)
        .add_systems(
            PreStartup,
            |assets: Res<AssetServer>, mut window: ResMut<WindowUtils>| {
                window.window_icon = Some(assets.load(assets::ICON_PATH));
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
    monitor_q: Query<(&Monitor, Has<PrimaryMonitor>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        use bevy_framepace::Limiter;
	let hz: f64;
	match monitor_q.single() {
	    Ok((monitor, _)) => {
		hz = (monitor.refresh_rate_millihertz.unwrap_or(0).div_ceil(10000) * 10) as f64;
	    },
	    Err(_) => {
		warn!("No monitor was detected. Can't limit fps.");
		return;
	    }
	}
	
        settings.limiter = match settings.limiter {
            Limiter::Auto => Limiter::Off,
            Limiter::Off => Limiter::from_framerate(30.0),
            Limiter::Manual(fps) => {
                // Attempt to fix refresh rate issue. Has no effect.
                if fps != Duration::from_secs_f64(1.0 / hz) {
                    Limiter::from_framerate(hz)
                } else {
                    Limiter::Off
                }
            }
        };
    }
}
