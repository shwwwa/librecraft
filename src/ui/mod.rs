use crate::settings::*;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow, WindowFocused, WindowResized};

pub mod hud;

/// Scales elements on display according to scale.
///
/// Automatic scale scales depending on window's size and contains its automatic value in itself.
/// Provide 0 to reload auto scale.
/// Allows setting custom scale with more precise controls.
#[derive(PartialEq, Clone, Copy, Resource, Debug)]
pub enum GUIScale {
    Auto(u8),
    Scale(u8),
    Custom(f32),
}

/// Determines if GUI is opened, closed, or user is typing something.
/// On opened GUI, allows user to handle his mouse.
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

//todo: scale and custom merge?
pub fn gui_scale_to_float(gui_scale: GUIScale) -> f32 {
    match gui_scale {
        GUIScale::Auto(x) => x as f32,
        GUIScale::Scale(x) => x as f32,
	GUIScale::Custom(x) => x,
    }
}

/** Controls user's mouse.

Responsible for two following actions:
- Grabs user cursor if in-game.
- Releases user cursor on lost focus/menu. */
pub fn handle_mouse(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut gui_mode: ResMut<GUIMode>,
    settings: Res<Settings>,
    mut gui_mode_events: ResMut<Events<GUIModeChanged>>,
    mut focus_reader: EventReader<WindowFocused>,
) {
    let mut gui_mode_cursor = gui_mode_events.get_cursor();

    for ev in focus_reader.read() {
        let is_playing = *gui_mode == GUIMode::Closed;

        if is_playing {
            if settings.pause_on_lost_focus {
                *gui_mode = GUIMode::Opened;
                gui_mode_events.send(GUIModeChanged {
                    gui_mode: *gui_mode,
                });

                return;
            }

            let mut window = windows.single_mut();

            if ev.focused {
                window.cursor_options.grab_mode = CursorGrabMode::Locked;
            } else {
                window.cursor_options.grab_mode = CursorGrabMode::None;
            }

            window.cursor_options.visible = !ev.focused;
        }
    }

    for ev in gui_mode_cursor.read(&gui_mode_events) {
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

/** Might cause flickering for a frame. */
pub fn update_gui_scale(
    mut gui_scale_reader: EventReader<GUIScaleChanged>,
    mut resize_reader: EventReader<WindowResized>,
    mut gui_scale: ResMut<GUIScale>,
    query_window: Query<&Window>,
) {
    // Return if non-auto
    match *gui_scale {
	GUIScale::Auto(_) => {},
	_ => { return },
    }
    
    // Remains 0 if doesn't need any changes
    let mut scale_change : bool = false;
    
    for evr in gui_scale_reader.read() {
	match evr.gui_scale {
	    GUIScale::Auto(scale) => {
		if scale == 0 {
		    // needs update
		    scale_change = true;
		}
	    },
	    _ => {}
	}
    }
    
    for evr in resize_reader.read() {
	
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
            *gui_scale = GUIScale::Auto(0);
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
