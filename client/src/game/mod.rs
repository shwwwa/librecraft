use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Capturing, Screenshot};
use bevy::window::SystemCursorIcon;
use bevy::winit::cursor::CursorIcon;

use crate::assets;
use crate::gui::debug;
use crate::gui::hud;
use crate::gui::menu;
use crate::gui::{self, GUIState};
use crate::music;
use crate::settings;
use world::skybox;

/** Responsible for player logic. */
pub mod player;
/** Responsible for world logic. */
pub mod world;

/** Plugin responsible for game logic. */
pub struct GamePlugin<S: States> {
    pub state: S,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct GameplaySet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct DataSet;

impl<S: States> Plugin for GamePlugin<S> {
    fn build(&self, app: &mut App) {
        app.init_state::<gui::GUIState>()
            .configure_sets(
                Update,
                (
                    GameplaySet.run_if(in_state(self.state.clone())),
                    GameplaySet.run_if(in_state(GUIState::Closed)),
                ),
            )
            .add_plugins(skybox::SkyboxPlugin::from_image_file(
                assets::SKYBOX_TEST_PATH,
            ))
            .init_resource::<settings::Settings>()
            .init_resource::<player::Player>()
            .add_event::<gui::GUIScaleChanged>()
            .add_event::<hud::HotbarSelectionChanged>()
            .add_event::<settings::SettingsUpdated>()
            .add_systems(
                OnEnter(self.state.clone()),
                (settings::setup_settings, player::setup_player_data).in_set(DataSet),
            )
            .add_systems(
                OnEnter(self.state.clone()),
                (
                    debug::setup_debug_hud,
                    menu::setup_pause_menu,
                    hud::setup_hotbar,
                    hud::setup_crosshair,
                    music::setup_soundtrack,
                )
                    .after(DataSet),
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
                    settings::change_fullscreen,
                    settings::update_settings,
                    settings::save_window_position,
                    settings::save_window_size,
                    menu::render_pause_menu,
                )
                    .run_if(in_state(self.state.clone())),
            )
            .add_systems(
                Update,
                (
                    debug::toggle_debug_hud,
                    gui::update_auto_gui_scale,
                    self::screenshot,
                    self::save_screenshot,
                )
                    .in_set(GameplaySet),
            )
            .add_systems(
                Update,
                (
                    hud::update_hotbar,
                    hud::update_hotbar_selection,
                    hud::update_hotbar_selector,
                    hud::update_crosshair,
                )
                    .in_set(GameplaySet),
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

/** System that screenshots whole screen by pressing F2. */
fn screenshot(mut commands: Commands, input: Res<ButtonInput<KeyCode>>, mut counter: Local<u32>) {
    if input.just_pressed(KeyCode::F2) {
        let path = format!("./screenshot-{}.png", *counter);
        *counter += 1;
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }
}

/** System that adds an ability to save screenshot to file. */
fn save_screenshot(
    mut commands: Commands,
    capturing_q: Query<Entity, With<Capturing>>,
    window_q: Query<Entity, With<Window>>,
) {
    let Ok(window) = window_q.single() else {
        warn!("Screenshot can't be saved.");
        return;
    };

    match capturing_q.iter().count() {
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
