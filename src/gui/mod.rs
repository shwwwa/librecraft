use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow, WindowFocused, WindowResized};

use crate::consts::{MIN_HEIGHT, MIN_WIDTH};
use crate::settings::{Settings, SettingsUpdated};

/** All that includes debug GUI */
pub mod debug;
/** HUD systems */
pub mod hud;
/** Menu systems: main menu, pause menu, game menus (inventory) */
pub mod menu;

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

#[derive(Event, Debug)]
pub struct GUIScaleChanged {
    pub gui_scale: GUIScale,
}

/// In-game only.
/// Determines if GUI is opened, closed, or user is typing something.
/// On opened GUI, allows user to handle his mouse, disables in-game controls.
#[derive(States, Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
pub enum GUIState {
    #[default]
    Opened,
    Closed,
    #[allow(dead_code)]
    Typing,
}

/** An utility function that converts [`GUIScale`] to [`f32`]. */
pub fn gui_scale_to_float(gui_scale: GUIScale) -> f32 {
    match gui_scale {
        GUIScale::Auto(x) | GUIScale::Scale(x) => x as f32,
        GUIScale::Custom(x) => x,
    }
}

/** An utility function that fires [`GUIScaleChanged`] event */
pub fn gui_scale_changed(
    gui_scale: &ResMut<GUIScale>,
    gui_scale_events: &mut ResMut<Events<GUIScaleChanged>>,
) {
    info!("GUI scale was changed: {:?}", *gui_scale);
    gui_scale_events.send(GUIScaleChanged {
        gui_scale: **gui_scale,
    });
}

/// Controls user's mouse.
///
/// Responsible for two following actions:
/// - Grabs user cursor if in-game.
/// - Releases user cursor on lost focus/menu.
pub fn handle_mouse(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut focus_reader: EventReader<WindowFocused>,
    settings: Res<Settings>,
    gui_state: Res<State<GUIState>>,
    mut gui_state_reader: EventReader<StateTransitionEvent<GUIState>>,
    mut next_gui_state: ResMut<NextState<GUIState>>,
) {
    for ev in focus_reader.read() {
        let is_playing = *gui_state.get() == GUIState::Closed;

        if is_playing {
            if settings.pause_on_lost_focus {
                next_gui_state.set(GUIState::Opened);
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

    for ev in gui_state_reader.read() {
        let mut window = windows.single_mut();

        // todo(gui): may cause panic - tests needed.
        let is_playing = ev.entered.unwrap() == GUIState::Closed;

        window.cursor_options.grab_mode = if is_playing {
            CursorGrabMode::Locked
        } else {
            CursorGrabMode::None
        };
        if !is_playing {
            let center = Vec2::new(window.width() / 2., window.height() / 2.);
            window.set_cursor_position(Some(center));
        }

        window.cursor_options.visible = !is_playing;
    }
}

/** If GUIScale::Auto(c) && c == 0, trigger update. */
pub fn update_auto_gui_scale(
    gui_scale: ResMut<GUIScale>,
    mut gui_scale_writer: EventWriter<GUIScaleChanged>,
) {
    if let GUIScale::Auto(scale) = *gui_scale {
        if scale == 0 {
            gui_scale_writer.send(GUIScaleChanged {
                gui_scale: *gui_scale,
            });
        }
    }
}

/// System that handles automatic gui scale change.
pub fn update_gui_scale(
    mut gui_scale_events: ResMut<Events<GUIScaleChanged>>,
    mut resize_reader: EventReader<WindowResized>,
    mut gui_scale: ResMut<GUIScale>,
    query_window: Query<&Window>,
) {
    let mut gui_scale_cursor = gui_scale_events.get_cursor();

    // Return if non-auto.
    match *gui_scale {
        GUIScale::Auto(_) => {}
        _ => return,
    }

    // Remains 0 if doesn't need any changes.
    let mut scale_change: bool = false;

    // WindowResized
    for evr in resize_reader.read() {
        let scale = gui_scale_to_float(*gui_scale);
        // Check if scale on both sides needs change.
        scale_change = (f32::ceil((evr.width + 1.) / MIN_WIDTH) - 1.) != scale
            && (f32::ceil((evr.height + 1.) / MIN_HEIGHT) - 1.) != scale;
    }

    // GUIScaleChanged
    for evr in gui_scale_cursor.read(&gui_scale_events) {
        if let GUIScale::Auto(scale) = evr.gui_scale {
            if scale == 0 {
                // Needs update.
                scale_change = true;
            }
        }
    }

    if scale_change {
        let resolution = query_window.single();
        // Basically gets best scale for window. We are appending +1 to make sure result of f32::ceil() is always >= 2.
        *gui_scale = GUIScale::Auto(
            (f32::ceil(f32::min(
                (resolution.width() + 1.) / MIN_WIDTH,
                (resolution.height() + 1.) / MIN_HEIGHT,
            )) as u8)
                - 1,
        );

        gui_scale_changed(&gui_scale, &mut gui_scale_events);
    }
}

/// System that handles gui scale change by request.
/// Because it's manual also saves it in settings.toml.
pub fn change_gui_scale(
    keys: Res<ButtonInput<KeyCode>>,
    mut gui_scale: ResMut<GUIScale>,
    mut gui_scale_events: ResMut<Events<GUIScaleChanged>>,
    mut settings: ResMut<Settings>,
    mut settings_writer: EventWriter<SettingsUpdated>,
) {
    let mut scale_changed: bool = false;
    if let GUIScale::Scale(scale) = *gui_scale {
        if keys.just_pressed(KeyCode::BracketLeft) && scale > 1 {
            *gui_scale = GUIScale::Scale(scale - 1);
            scale_changed = true;
        }

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
        gui_scale_changed(&gui_scale, &mut gui_scale_events);

        settings.gui_scale = gui_scale_to_float(*gui_scale);
        settings_writer.send(SettingsUpdated {
            settings: settings.clone(),
        });
    }
}
