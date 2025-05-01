#![allow(clippy::default_constructed_unit_structs)]
// Warns of some basic pitfalls, but basically too pedantic.
// #![warn(clippy::pedantic)]
#![feature(stmt_expr_attributes)]
// Tells windows not to show console window on release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Unsafe code violates one of design goals and used only in crates.
#![forbid(unsafe_code)]

#[cfg(all(feature = "embed-assets", feature = "fast-compile"))]
compile_error!("features `crate/embed-assets` and `crate/fast-compile` are mutually exclusive");

use bevy::prelude::*;
use bevy::window::{Monitor, PrimaryMonitor};
use bevy::{
    app::PluginGroupBuilder,
    diagnostic::{FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin},
    log::LogPlugin,
    window::PresentMode,
};

#[cfg(feature = "embed-assets")]
use bevy_embedded_assets::{self, EmbeddedAssetPlugin};
use bevy_framepace::FramepacePlugin;
use bevy_renet::RenetClientPlugin;
use bevy_window_utils::{WindowUtils, WindowUtilsPlugin};

/** Librecraft's module of assets path. */
pub mod assets;
/** Librecraft's main hard-coded constants. */
pub mod consts;
/** Librecraft's game logic. */
pub mod game;
/** Librecraft's GUI. (cross-state)*/
pub mod gui;
/** Librecraft's music. (then audio->music, audio->sound) */
pub mod music;
/** Reads and stores user owned settings. */
pub mod settings;
/** Adds splash screen to app (independent). */
pub mod splash;

#[cfg(not(debug_assertions))]
use dirs::config_dir;

use consts::*;
use game::world::SkyboxCamera;
use game::GamePlugin;
use settings::SettingsPath;
use splash::SplashPlugin;

use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

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
                    .set(AssetPlugin {
                        file_path: ASSET_FOLDER.to_owned(),
                        ..default()
                    })
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
                    .set(
                        #[cfg(debug_assertions)]
                        LogPlugin {
                            filter: LOG_FILTER.to_owned(),
                            level: bevy::log::Level::DEBUG,
                            ..default()
                        },
                        #[cfg(not(debug_assertions))]
                        LogPlugin {
                            filter: LOG_FILTER.to_owned(),
                            level: bevy::log::Level::INFO,
                            ..default()
                        },
                    )
                    .set(ImagePlugin::default_nearest()),
            )
            .add(WindowUtilsPlugin::default())
            .add(FrameTimeDiagnosticsPlugin::default())
            .add(SystemInformationDiagnosticsPlugin)
            .add(RenetClientPlugin)
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
    if DEBUG_MODE {
        settings_path = PathBuf::from_str(DEBUG_SETTINGS_PATH).unwrap();
    } else {
        #[cfg(debug_assertions)]
        {
            warn!(
                "Debug build is used, while debug mode is false. This can cause unexpected issues."
            );
            settings_path = PathBuf::from_str(DEBUG_SETTINGS_PATH).unwrap();
        }
        #[cfg(not(debug_assertions))]
        {
            settings_path = config_dir().unwrap();
            settings_path.push(consts::title!());
        }
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
            ..default()
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
    // Sets up directional light.
    commands.spawn((
        DirectionalLight {
            illuminance: 32000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 2.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 4.)),
    ));

    commands.spawn((
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            fov: 120_f32.to_radians(),
            ..default()
        }),
        SkyboxCamera,
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
    monitor_q: Query<&Monitor, With<PrimaryMonitor>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        use bevy_framepace::Limiter;

        let hz: f64 = match monitor_q.single() {
            Ok(monitor) => {
                (monitor.refresh_rate_millihertz.unwrap_or(0).div_ceil(10000) * 10) as f64
            }
            Err(_) => {
                warn_once!("No monitor was detected. Can't limit fps.");
                return;
            }
        };

        settings.limiter = match settings.limiter {
            Limiter::Auto => Limiter::Off,
            Limiter::Off => Limiter::from_framerate(30.0),
            Limiter::Manual(fps) => {
                if fps != Duration::from_secs_f64(1.0 / hz) {
                    Limiter::from_framerate(hz)
                } else {
                    Limiter::Off
                }
            }
        };
    }
}
