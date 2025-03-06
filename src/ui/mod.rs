use bevy::prelude::Resource;

pub mod hud;

#[derive(PartialEq, Eq, Clone, Copy, Resource)]
pub enum GUIScale {
    Auto,
    Scale(u8),
}
