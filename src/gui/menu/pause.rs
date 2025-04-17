use bevy::prelude::*;

/** Marker to find pause menu hud box entity. */
#[derive(Component)]
pub struct PauseHudRoot;

/** Draws and setups pause menu. */
pub fn setup_pause_menu(mut commands: Commands) {
    let hud_root = commands
        .spawn((
            PauseHudRoot,
            (
                BackgroundColor(Color::BLACK.with_alpha(0.5)),
                GlobalZIndex(i32::MAX),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(100.),
                    top: Val::Percent(100.),
                    ..Default::default()
                },
            ),
        ))
        .id();

    let text_pause = commands
        .spawn((
            Text::new("Pause menu"),
            TextFont::from_font_size(16.0),
            TextColor(Color::WHITE),
        ))
        .id();

    commands.entity(hud_root).add_children(&[text_pause]);
}
