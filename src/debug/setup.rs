use crate::FpsText;
use bevy::prelude::*;
use bevy::window::{Monitor, PrimaryMonitor};

// Marker to find container's entity
#[derive(Component)]
pub struct HudRoot;

pub fn setup_debug_hud(
    mut commands: Commands,
    query_monitor: Query<(Entity, &Monitor, Has<PrimaryMonitor>)>,
) {
    let hud_root = commands
        .spawn((
            HudRoot,
            (
                BackgroundColor(Color::BLACK.with_alpha(0.5)),
                GlobalZIndex(i32::MAX),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(1.),
                    top: Val::Percent(1.),
                    // resized depending on text inside
                    bottom: Val::Auto,
                    right: Val::Auto,
                    padding: UiRect::all(Val::Px(4.0)),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
            ),
        ))
        .id();

    let text_fps = commands
        .spawn((
            Text::new("FPS: "),
            TextFont::from_font_size(16.0),
            TextColor(Color::WHITE),
        ))
        .with_child((
            FpsText,
            TextSpan::new("N/A"),
            TextFont::from_font_size(16.0),
            TextColor(Color::WHITE),
        ))
        .id();

    let mut text_monitor = commands.spawn((
        Text::new("Monitor: "),
        TextFont::from_font_size(16.0),
        TextColor(Color::WHITE),
    ));

    for (_, monitor, is_primary) in query.iter() {
        let mut monitor_info = monitor
            .name
            .clone()
            .unwrap_or_else(|| "undefined".to_string());
        if !is_primary {
            monitor_info.push_str(" (primary) ");
        }
        monitor_info.push_str(&format!(
            "({}x{}, {} hz) ",
            monitor.physical_width.clone(),
            monitor.physical_height.clone(),
            monitor.refresh_rate_millihertz.unwrap_or(0).div_ceil(10000) * 10
        ));
        text_monitor.with_child((
            TextSpan::new(monitor_info),
            TextFont::from_font_size(16.0),
            TextColor(Color::WHITE),
        ));
    }

    let mut text_adapter = commands.spawn((
	Text::new("GPU: "),
	TextFont::from_font_size(16.0),
	TextColor(Color::WHITE),
    ));
    
    let text_monitor_info = text_monitor.id();
    let text_adapter_info = text_adapter.id();
    
    commands
        .entity(hud_root)
        .add_children(&[text_fps, text_monitor_info, text_adapter_info]);
}

pub fn toggle_debug_hud(
    mut q_hud_root: Query<&mut Visibility, With<HudRoot>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::F3) {
        let mut visibility = q_hud_root.single_mut();
        *visibility = match *visibility {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        }
    }
}
