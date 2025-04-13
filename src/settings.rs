use bevy::prelude::Resource;

use serde::Deserialize;
use toml::from_str;

use std::error::Error;

#[derive(Deserialize, Clone, Copy, Resource)]
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

pub fn read_settings(file: &str) -> Result<Settings, Box<dyn Error>> {
    let settings_str = std::fs::read_to_string(file)?;
    let settings = from_str(&settings_str)?;
    Ok(settings)
}
