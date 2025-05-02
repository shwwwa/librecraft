use std::error::Error;
use std::path::PathBuf;

use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode, WindowResized, WindowResolution};
use bevy_window_utils::WindowUtils;
use serde::{Deserialize, Serialize};
use toml::from_str;

use crate::assets::{FONT_PATH, MINECRAFT_FONT_PATH, RuntimeAsset};
use crate::consts::DEBUG_MODE;
use crate::gui::GUIScale;

/// -1 for default values on startup (pos - centered).
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Clone, Resource, Debug)]
pub struct Settings {
    pub player_name: String,
    pub fullscreen: bool,
    pub position_x: i32,
    pub position_y: i32,
    pub size_x: f32,
    pub size_y: f32,
    pub maximized: bool,
    pub seed: u64,
    pub gui_scale: f32,
    pub pause_on_lost_focus: bool,
    pub mute_on_lost_focus: bool,
    /// Controversial change: replace fonts to be minecraft-like.
    pub replace_fonts: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            player_name: "player1".to_string(),
            fullscreen: false,
            pause_on_lost_focus: true,
            mute_on_lost_focus: true,
            replace_fonts: true,
            position_x: -1,
            position_y: -1,
            size_x: -1.,
            size_y: -1.,
            maximized: false,
            seed: 0,
            gui_scale: 0.,
        }
    }
}

/// Contains only 2 fields: path to settings file and ability to save changes.
#[derive(Resource)]
pub struct SettingsPath {
    pub path: PathBuf,
    pub save_settings: bool,
}

impl Default for SettingsPath {
    fn default() -> Self {
        Self { path: PathBuf::new(), save_settings: false }
    }
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
    mut window_q: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keys.just_pressed(KeyCode::F11) {
        settings.fullscreen = !settings.fullscreen;
        match window_q.single_mut() {
            Ok(mut window) => {
                if settings.fullscreen {
                    window.mode = WindowMode::Fullscreen(
                        MonitorSelection::Current,
                        VideoModeSelection::Current,
                    );
                } else {
                    window.mode = WindowMode::Windowed;
                }
            },
            Err(_) => {
                warn_once!("No primary window detected. Fullscreen cannot be changed.");
            },
        }

        debug!("Fullscreen mode: {}", settings.fullscreen);

        settings_writer.write(SettingsUpdated { settings: settings.clone() });
    }
}

pub fn save_window_position(
    window_utils: Res<WindowUtils>,
    mut settings: ResMut<Settings>,
    mut settings_writer: EventWriter<SettingsUpdated>,
    mut position_reader: EventReader<WindowMoved>,
) {
    for ev in position_reader.read() {
        match window_utils.is_maximized {
            Some(is_maximized) => {
                if settings.maximized != is_maximized {
                    settings.maximized = is_maximized;
                    info!("Window was maximized/minimized.");
                }
            },
            None => info!("Couldn't intercept maximized method."),
        }

        settings.position_x = ev.position.x;
        settings.position_y = ev.position.y;

        // Produces a lot of events when moving, but so far works.
        debug!("Window changed position: {}x{}px", settings.position_x, settings.position_y);

        settings_writer.write(SettingsUpdated { settings: settings.clone() });
    }
}

pub fn save_window_size(
    mut settings: ResMut<Settings>,
    mut settings_writer: EventWriter<SettingsUpdated>,
    mut size_reader: EventReader<WindowResized>,
) {
    for ev in size_reader.read() {
        settings.size_x = ev.width;
        settings.size_y = ev.height;

        debug!("Window resized: {}x{}px", settings.size_x, settings.size_y);

        settings_writer.write(SettingsUpdated { settings: settings.clone() });
    }
}

pub fn setup_settings(
    mut commands: Commands,
    mut settings: ResMut<Settings>,
    mut settings_path: ResMut<SettingsPath>,
    mut window_q: Query<&mut Window, With<PrimaryWindow>>,
) {
    match read_settings((*settings_path.path).to_path_buf(), &mut settings) {
        Ok(()) => {
            info!("{:?}", *settings);
            settings_path.save_settings = true;
        },
        Err(e) => {
            let path: &str = settings_path.path.to_str().unwrap();

            warn!("Couldn't retrieve settings from {}: {}.", path, e);
            if DEBUG_MODE {
                warn!("Debug mode enabled, settings file is not created, fallback values in use.");
            } else {
                warn!("Creating default settings file...");
                match write_settings((*settings_path.path).to_path_buf(), &settings) {
                    Ok(()) => {
                        info!("Default settings file was created.");
                        settings_path.save_settings = true;
                    },
                    Err(e) => {
                        warn!("Default settings file can't be created: {}", e);
                    },
                }
            }
        },
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

    match window_q.single_mut() {
        Ok(mut window) => {
            if settings.position_x > 0 && settings.position_y > 0 {
                window.position =
                    WindowPosition::At(IVec2::new(settings.position_x, settings.position_y));
            }

            if settings.size_x > 0. && settings.size_y > 0. {
                window.resolution = WindowResolution::new(settings.size_x, settings.size_y);
            }

            if settings.maximized {
                window.set_maximized(true);
            }

            if settings.fullscreen {
                window.mode =
                    WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current);
            }
        },
        Err(_) => {
            warn_once!("No primary window detected. Window settings are ignored.");
        },
    }

    if settings.replace_fonts {
        commands.insert_resource(RuntimeAsset { font_path: MINECRAFT_FONT_PATH.to_string() });
    } else {
        commands.insert_resource(RuntimeAsset { font_path: FONT_PATH.to_string() });
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
    let settings_toml = toml::to_string(settings)?;

    std::fs::write(file_path, settings_toml)?;

    Ok(())
}

pub fn read_settings(file_path: PathBuf, settings: &mut Settings) -> Result<(), Box<dyn Error>> {
    debug!("Path to settings: {:?}", std::fs::canonicalize(&file_path));

    let settings_str = std::fs::read_to_string(file_path)?;

    *settings = from_str(&settings_str)?;

    Ok(())
}
