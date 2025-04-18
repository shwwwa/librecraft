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

pub mod game;
mod gui;

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
        // Init default versions of resources. Later on startup they should be overwritten.
        // In future should be in their own plugin space.
	.init_resource::<GUIMode>()
        .init_resource::<Settings>()
        .init_resource::<Player>()
	.add_event::<GUIScaleChanged>()
        .add_event::<GUIModeChanged>()
        .add_plugins(SplashPlugin, GamePlugin)
        .run();
}
