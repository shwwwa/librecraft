#![allow(clippy::default_constructed_unit_structs)]
// Tells windows not to show console window on release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Unsafe code belongs violates one of design goals and used only in crates.
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
    /** Title of the main program. */
    pub const TITLE: &str = env!("CARGO_PKG_NAME");
    /** Version of the main program. */
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    /** Librecraft's minimum resolution width. */
    pub const MIN_WIDTH: f32 = 320.;
    /** Librecraft's minimum resolution height. */
    pub const MIN_HEIGHT: f32 = 240.;
}

/** Adds splash screen to app (independent). */
pub mod splash;
/** All that belongs to GUI from debug info to hud. (except splash)*/
pub mod gui;
/** Responsible for music. (then audio->music, audio->sound) */
pub mod music;
/** Accesses player information. (getting transferred to game mod) */
pub mod player;
/** Reads and stores user owned settings. */
pub mod settings;
/** All resources that belong to game's logic. */
pub mod game;

use consts::{FIXED_TIME_CLOCK, TITLE, VERSION, MIN_WIDTH, MIN_HEIGHT};
use splash::SplashPlugin;
use gui::{GUIScaleChanged, GUIMode, GUIModeChanged}; 
use player::Player;
use settings::Settings;
use game::GamePlugin;

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
    app.add_plugins(NecessaryPlugins)
	.add_systems(
	    PreStartup,
	    |assets: Res<AssetServer>, mut window: ResMut<WindowUtils>| {
                window.window_icon = Some(assets.load("icon/icon512.png"));
	    },)
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
	.insert_resource(Time::<Fixed>::from_hz(FIXED_TIME_CLOCK))
        // Init default versions of resources.
	// Later on startup they should be overwritten.
        // In future should be in their own plugin space.
	.init_resource::<GUIMode>()
        .init_resource::<Settings>()
        .init_resource::<Player>()
	.add_event::<GUIScaleChanged>()
        .add_event::<GUIModeChanged>()
        .add_plugins((SplashPlugin, GamePlugin))
        .run();
}
