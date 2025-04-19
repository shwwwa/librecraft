use bevy::window::PrimaryWindow;
use bevy::{prelude::*, window::WindowMode};

use serde::Deserialize;
use toml::from_str;

use crate::gui::GUIScale;
use std::error::Error;

#[allow(dead_code)]
#[derive(Deserialize, Clone, Copy, Resource, Debug)]
pub struct Settings {
    pub seed: u64,
    pub gui_scale: f32,
    pub fullscreen: bool,
    pub pause_on_lost_focus: bool,
    pub mute_on_lost_focus: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            seed: 0,
            gui_scale: 0.1,
            fullscreen: false,
            pause_on_lost_focus: true,
            mute_on_lost_focus: true,
        }
    }
}

pub fn is_mute_on_lost_focus(settings: Res<Settings>) -> bool {
    settings.mute_on_lost_focus
}

pub fn change_fullscreen(
    keys: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<Settings>,
    mut query_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keys.just_pressed(KeyCode::F11) {
        settings.fullscreen = !settings.fullscreen;

        if settings.fullscreen {
            query_window.single_mut().mode = WindowMode::Fullscreen(MonitorSelection::Current);
        } else {
            query_window.single_mut().mode = WindowMode::Windowed;
        }
    }
}

pub fn setup_settings(
    mut commands: Commands,
    mut settings: ResMut<Settings>,
    mut query_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    match read_settings("./assets/settings.toml", &mut settings) {
        Ok(()) => info!("{:?}", *settings),
        Err(e) => warn!("Couldn't retrieve settings: {}", e),
    }

    let gui_scale = f32::floor(settings.gui_scale);

    // Protection from negative value
    if settings.gui_scale < 0.2 {
        commands.insert_resource(GUIScale::Auto(0));
    } else if settings.gui_scale == gui_scale {
        commands.insert_resource(GUIScale::Scale(gui_scale as u8));
    } else {
        commands.insert_resource(GUIScale::Custom(settings.gui_scale));
    }

    if settings.fullscreen {
        query_window.single_mut().mode = WindowMode::Fullscreen(MonitorSelection::Current);
    }
}

pub fn read_settings(file: &str, settings: &mut Settings) -> Result<(), Box<dyn Error>> {
    debug!("Path to settings: {:?}", std::fs::canonicalize(file));

    let settings_str = std::fs::read_to_string(file)?;

    *settings = from_str(&settings_str)?;

    Ok(())
}
