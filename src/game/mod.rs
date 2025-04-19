use bevy::prelude::*;
use bevy::render::view::screenshot::{Capturing, Screenshot, save_to_disk};
use bevy::window::SystemCursorIcon;
use bevy::winit::cursor::CursorIcon;

use crate::gui;
use crate::gui::debug;
use crate::gui::hud;
use crate::gui::menu;
use crate::music;
use crate::settings;

/** Accesses player information. */
pub mod player;

/** Plugin responsible for game logic. */
pub struct GamePlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for GamePlugin<S> {
    fn build(&self, app: &mut App) {
        app.init_state::<gui::GUIState>()
            .init_resource::<settings::Settings>()
            .init_resource::<player::Player>()
            .add_event::<gui::GUIScaleChanged>()
            .add_event::<hud::HotbarSelectionChanged>()
            .add_systems(
                OnEnter(self.state.clone()),
                (
                    settings::setup_settings,
                ),
            )
            .add_systems(
                OnEnter(self.state.clone()),
                (
		    /* needs to be updated once in case gui scale is 0 */
		    gui::setup_gui_scale,
		    debug::setup_debug_hud,
                    menu::setup_pause_menu,
                    hud::setup_hotbar,
                    hud::setup_crosshair,
		    player::setup_player_data,
                    music::setup_soundtrack,
                ).after(settings::setup_settings),
            )
            .add_systems(
                FixedUpdate,
                (
                    debug::update_fps_text,
                    debug::update_display_text,
                    debug::update_focus_text,
                )
                    .run_if(in_state(self.state.clone())),
            )
            .add_systems(
                Update,
                (
                    gui::update_gui_scale,
                    gui::change_gui_scale,
                    gui::handle_mouse,
                    debug::toggle_debug_hud,
                    debug::limit_fps,
                    settings::change_fullscreen,
                    screenshot,
                    save_screenshot,
                )
                    .run_if(in_state(self.state.clone())),
            )
            .add_systems(
                Update,
                (
                    hud::update_hotbar,
                    hud::update_hotbar_selection,
                    hud::update_hotbar_selector,
                    hud::update_crosshair,
                    menu::render_pause_menu,
                )
                    .run_if(in_state(self.state.clone())),
            )
            .add_systems(
                Update,
                (music::fade_in, music::fade_out, music::change_track)
                    .run_if(in_state(self.state.clone())),
            )
            .add_systems(
                Update,
                music::mute_music_on_focus
                    .run_if(settings::is_mute_on_lost_focus)
                    .run_if(in_state(self.state.clone())),
            );
    }
}

/** System that screenshots whole screen by pressing F2 */
fn screenshot(mut commands: Commands, input: Res<ButtonInput<KeyCode>>, mut counter: Local<u32>) {
    if input.just_pressed(KeyCode::F2) {
        let path = format!("./screenshot-{}.png", *counter);
        *counter += 1;
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }
}

/** Save screenshot */
fn save_screenshot(
    mut commands: Commands,
    screenshot_saving: Query<Entity, With<Capturing>>,
    query_window: Query<Entity, With<Window>>,
) {
    let Ok(window) = query_window.get_single() else {
        warn!("Couldn't save screenshot.");
        return;
    };

    match screenshot_saving.iter().count() {
        0 => {
            commands.entity(window).remove::<CursorIcon>();
        }
        x if x > 0 => {
            commands
                .entity(window)
                .insert(CursorIcon::from(SystemCursorIcon::Progress));
        }
        _ => {}
    }
}
