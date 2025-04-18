use bevy::prelude::*;
use bevy::render::view::screenshot::{Capturing, Screenshot, save_to_disk};
use bevy::window::SystemCursorIcon;
use bevy::winit::cursor::CursorIcon;

use crate::crosshair::{setup_crosshair, update_crosshair};
use crate::debug::*;
use crate::gui::{
    GUIMode, GUIModeChanged, GUIScale, GUIScaleChanged, change_gui_scale,
    handle_mouse, hud::*, menu::*, update_gui_scale,
};
use crate::hotbar::{
    HotbarSelectionChanged, setup_hotbar, update_hotbar, update_hotbar_selection,
    update_hotbar_selector,
};
use crate::music::{change_track, fade_in, fade_out, mute_music_on_focus, setup_soundtrack};
use crate::player::*;
use crate::settings::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
	app.add_event::<HotbarSelectionChanged>()
            .add_systems(
		PreStartup,
		(setup_debug_hud, setup_settings, setup_player_data),
            )
            .add_systems(
		Startup,
		(
                    setup_camera,
                    setup_pause_menu,
                    setup_hotbar,
                    setup_crosshair,
                    setup_soundtrack,
		),
            )
            .add_systems(
		FixedUpdate,
		(update_fps_text, update_display_text, update_focus_text),
            )
            .add_systems(
		Update,
		(
                    toggle_debug_hud,
                    change_fullscreen,
                    screenshot,
                    save_screenshot,
                    update_gui_scale,
                    change_gui_scale,
                    limit_fps,
                    handle_mouse,
		),
            )
	    .add_systems(
		Update,
		render_pause_menu,
	    )
            .add_systems(
		Update,
		(
                    update_hotbar,
                    update_hotbar_selection,
                    update_hotbar_selector,
                    update_crosshair,
		),
            )
            .add_systems(Update, (fade_in, fade_out, change_track))
            .add_systems(Update, mute_music_on_focus.run_if(is_mute_on_lost_focus))
    }
}


/** Setups camera for game to use. */
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Msaa::Off,
    ));
}

fn screenshot(mut commands: Commands, input: Res<ButtonInput<KeyCode>>, mut counter: Local<u32>) {
    if input.just_pressed(KeyCode::F2) {
        let path = format!("./screenshot-{}.png", *counter);
        *counter += 1;
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }
}

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
