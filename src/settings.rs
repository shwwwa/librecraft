use bevy::window::PrimaryWindow;
use bevy::{prelude::*, window::WindowMode};

use serde::{Deserialize, Serialize};
use toml::from_str;

use crate::gui::GUIScale;
use std::error::Error;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Clone, Copy, Resource, Debug)]
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
            gui_scale: 0.,
            fullscreen: false,
            pause_on_lost_focus: true,
            mute_on_lost_focus: true,
        }
    }
}

/** Contains only 2 fields: path to settings file and ability to save changes. */
#[derive(Resource)]
pub struct SettingsPath {
    pub path: PathBuf,
    pub save_settings: bool,
}

#[derive(Event, Debug)]
pub struct SettingsUpdated {
    pub settings: Settings,
}

pub fn is_mute_on_lost_focus(settings: Res<Settings>) -> bool {
    settings.mute_on_lost_focus
}

pub fn change_fullscreen(
    keys: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<Settings>,
    mut settings_writer: EventWriter<SettingsUpdated>,
    mut query_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keys.just_pressed(KeyCode::F11) {
        settings.fullscreen = !settings.fullscreen;

        if settings.fullscreen {
            query_window.single_mut().mode = WindowMode::Fullscreen(MonitorSelection::Current);
        } else {
            query_window.single_mut().mode = WindowMode::Windowed;
        }

        settings_writer.send(SettingsUpdated {
            settings: *settings,
        });
    }
}

pub fn setup_settings(
    mut commands: Commands,
    mut settings: ResMut<Settings>,
    mut settings_path: ResMut<SettingsPath>,
    mut query_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    match read_settings((*settings_path.path).to_path_buf(), &mut settings) {
        Ok(()) => {
            info!("{:?}", *settings);
            settings_path.save_settings = true;
        }
        Err(e) => {
            let path: &str = settings_path.path.to_str().unwrap();
            #[cfg(debug_assertions)]
            let debug: bool = true;
            #[cfg(not(debug_assertions))]
            let debug: bool = false;

            warn!("Couldn't retrieve settings from {}: {}.", path, e);
            if debug {
                warn!("Debug mode enabled, settings file is not created, fallback values in use.");
            } else {
                warn!("Creating default settings file...");
                match write_settings((*settings_path.path).to_path_buf(), &settings) {
                    Ok(()) => {
                        info!("Default settings file was created.");
                        settings_path.save_settings = true;
                    }
                    Err(e) => {
                        warn!("Default settings file can't be created: {}", e);
                    }
                }
            }
        }
    }

    let gui_scale = f32::floor(settings.gui_scale);

    // Too small/negative values.
    if settings.gui_scale < 0.5 {
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

pub fn update_settings(
    settings_path: ResMut<SettingsPath>,
    mut settings_reader: EventReader<SettingsUpdated>,
) {
    if !settings_path.save_settings {
        return;
    }
    for ev in settings_reader.read() {
        match write_settings((*settings_path.path).to_path_buf(), &ev.settings) {
            Ok(()) => debug!("Settings file was modified."),
            Err(e) => warn!("Couldn't update settings file: {}", e),
        }
    }
}

pub fn write_settings(file_path: PathBuf, settings: &Settings) -> Result<(), Box<dyn Error>> {
    let settings_toml = toml::to_string(*&settings)?;

    std::fs::write(file_path, settings_toml)?;

    Ok(())
}

pub fn read_settings(file_path: PathBuf, settings: &mut Settings) -> Result<(), Box<dyn Error>> {
    debug!("Path to settings: {:?}", std::fs::canonicalize(&file_path));

    let settings_str = std::fs::read_to_string(file_path)?;

    *settings = from_str(&settings_str)?;

    Ok(())
}
