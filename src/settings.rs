use bevy::prelude::*;
use serde::Deserialize;
use toml::from_str;

use std::error::Error;

#[allow(dead_code)]
#[derive(Deserialize, Clone, Copy, Resource, Debug)]
pub struct Settings {
    pub seed: u64,
    pub pause_on_lost_focus: bool,
    pub mute_on_lost_focus: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            seed: 0,
            pause_on_lost_focus: true,
            mute_on_lost_focus: true,
        }
    }
}

pub fn is_mute_on_lost_focus(settings: Res<Settings>) -> bool {
    settings.mute_on_lost_focus
}

pub fn setup_settings(mut settings: ResMut<Settings>) {
    match read_settings("./assets/settings.toml", &mut settings) {
        Ok(()) => info!("{:?}", *settings),
        Err(e) => warn!("Couldn't retrieve settings: {}", e),
    }
}

pub fn read_settings(file: &str, settings: &mut Settings) -> Result<(), Box<dyn Error>> {
    debug!("Path to settings: {:?}", std::fs::canonicalize(file));

    let settings_str = std::fs::read_to_string(file)?;

    *settings = from_str(&settings_str)?;

    Ok(())
}
