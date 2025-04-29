use bevy::prelude::*;

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
