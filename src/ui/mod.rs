use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow, WindowFocused};

pub mod hud;

/** Scales elements on display according to scale. Automatic scale scales depending on window's size. */
#[derive(PartialEq, Eq, Clone, Copy, Resource, Debug)]
pub enum GUIScale {
    Auto,
    Scale(u8),
}

/** Determines if GUI is opened, closed, or user is typing something. On opened GUI, allows user to handle his mouse. */
#[derive(PartialEq, Eq, Clone, Copy, Resource, Debug)]
pub enum GUIMode {
    Closed,
    Opened,
    #[allow(dead_code)]
    Typing,
}

#[derive(Event, Debug)]
pub struct GUIScaleChanged {
    pub gui_scale: GUIScale,
}

#[derive(Event, Debug)]
pub struct GUIModeChanged {
    pub gui_mode: GUIMode,
}

pub fn gui_scale_to_float(gui_scale: GUIScale) -> f32 {
    match gui_scale {
        GUIScale::Auto => 2_f32,
        GUIScale::Scale(x) => x as f32,
    }
}

pub fn handle_mouse(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    gui_mode: ResMut<GUIMode>,
    mut gui_mode_events: EventReader<GUIModeChanged>,
    mut focus_reader: EventReader<WindowFocused>,
) {
    for ev in focus_reader.read() {
        let is_playing = *gui_mode == GUIMode::Closed;

        if is_playing {
            let mut window = windows.single_mut();

            if ev.focused {
                window.cursor_options.grab_mode = CursorGrabMode::Locked;
            } else {
                window.cursor_options.grab_mode = CursorGrabMode::None;
            }

            window.cursor_options.visible = !ev.focused;
        }
    }

    for ev in gui_mode_events.read() {
        let mut window = windows.single_mut();

        let is_playing = ev.gui_mode == GUIMode::Closed;

        window.cursor_options.grab_mode = if is_playing {
            CursorGrabMode::Locked
        } else {
            CursorGrabMode::None
        };

        window.cursor_options.visible = !is_playing;
    }
}

pub fn change_gui_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut gui_mode: ResMut<GUIMode>,
    mut gui_mode_events: EventWriter<GUIModeChanged>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if *gui_mode == GUIMode::Closed {
            *gui_mode = GUIMode::Opened;
        } else {
            *gui_mode = GUIMode::Closed;
        }

        info!("GUI mode was changed: {:?}", *gui_mode);
        gui_mode_events.send(GUIModeChanged {
            gui_mode: *gui_mode,
        });
    }
}

pub fn change_gui_scale(
    keys: Res<ButtonInput<KeyCode>>,
    mut gui_scale: ResMut<GUIScale>,
    mut gui_scale_events: EventWriter<GUIScaleChanged>,
) {
    let mut scale_changed: bool = false;
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

    if scale_changed {
        info!("GUI scale was changed: {:?}", *gui_scale);
        gui_scale_events.send(GUIScaleChanged {
            gui_scale: *gui_scale,
        });
    }
}
