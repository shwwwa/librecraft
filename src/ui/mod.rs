use bevy::prelude::{ButtonInput, KeyCode, Res, ResMut, Resource, info, Event, EventWriter};

pub mod hud;

#[derive(PartialEq, Eq, Clone, Copy, Resource, Debug)]
pub enum GUIScale {
    Auto,
    Scale(u8),
}

#[derive(Event, Debug)]
pub struct GUIScaleChanged {
    pub gui_scale: GUIScale,
}

pub fn gui_scale_to_float(gui_scale: GUIScale) -> f32 {
    match gui_scale {
        GUIScale::Auto => 2_f32,
        GUIScale::Scale(x) => x as f32,
    }
}

pub fn change_gui_scale(
    keys: Res<ButtonInput<KeyCode>>,
    mut gui_scale: ResMut<GUIScale>,
    mut gui_scale_events: EventWriter<GUIScaleChanged>,
) {
    let mut scale_changed : bool = false;
    if let GUIScale::Scale(scale) = *gui_scale {
        if keys.just_pressed(KeyCode::BracketLeft) && scale > 1 {
            *gui_scale = GUIScale::Scale(scale - 1);
            scale_changed = true;
        }
        // todo: recognize max size with the help of display
        if keys.just_pressed(KeyCode::BracketRight) && scale < 5 {
            *gui_scale = GUIScale::Scale(scale + 1);
            scale_changed = true;
        }

        if keys.just_pressed(KeyCode::Backslash) {
            *gui_scale = GUIScale::Auto;
            scale_changed = true;
        }
    } else if keys.just_pressed(KeyCode::BracketLeft)
        || keys.just_pressed(KeyCode::BracketRight)
        || keys.just_pressed(KeyCode::Backslash)
    {
        *gui_scale = GUIScale::Scale(1);
	scale_changed = true;
    }

    if scale_changed
    {
	info!("Gui scale was changed: {:?}", *gui_scale);
	gui_scale_events.send(GUIScaleChanged { gui_scale: *gui_scale });
    }
}
