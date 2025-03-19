use bevy::prelude::{ButtonInput, KeyCode, Res, ResMut, Resource};

pub mod hud;

#[derive(PartialEq, Eq, Clone, Copy, Resource, Debug)]
pub enum GUIScale {
    Auto,
    Scale(u8),
}

pub fn change_gui_scale(keys: Res<ButtonInput<KeyCode>>, mut gui_scale: ResMut<GUIScale>) {
    //debug!("{:#?}", gui_scale);
    if let GUIScale::Scale(scale) = *gui_scale {
        if keys.just_pressed(KeyCode::BracketLeft) && scale > 1 {
            *gui_scale = GUIScale::Scale(scale - 1);
        }
	// todo: recognize max size with the help of display
        if keys.just_pressed(KeyCode::BracketRight) && scale < 5 {
            *gui_scale = GUIScale::Scale(scale + 1);   
        }

        if keys.just_pressed(KeyCode::Backslash) {
            *gui_scale = GUIScale::Auto;
        }
    } else if keys.just_pressed(KeyCode::BracketLeft)
        || keys.just_pressed(KeyCode::BracketRight)
        || keys.just_pressed(KeyCode::Backslash)
    {
        // todo: from auto to scale (not needed to replicate minecraft behaviour)
        *gui_scale = GUIScale::Scale(1);
    }

}
