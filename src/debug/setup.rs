use crate::{DisplayText, FocusText, FpsText};
use bevy::diagnostic::SystemInfo;
use bevy::prelude::*;
use bevy::render::renderer::RenderAdapterInfo;
use bevy::window::{Monitor, PrimaryMonitor};
use wgpu_types::DeviceType;

/** Marker to find debug's hud box entity */
#[derive(Component)]
pub struct HudRoot;

/** Setups debug hud in the left-up corner of the screen */
pub fn setup_debug_hud(
    mut commands: Commands,
    system: Res<SystemInfo>,
    adapter: Res<RenderAdapterInfo>,
    query_monitor: Query<(Entity, &Monitor, Has<PrimaryMonitor>)>,
    query_window: Query<&Window>,
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

    for (_, monitor, is_primary) in query_monitor.iter() {
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

    let text_monitor_info = text_monitor.id();

    let mut text_display = commands.spawn((
        Text::new("Display: "),
        TextFont::from_font_size(16.0),
        TextColor(Color::WHITE),
    ));

    let window = query_window.single();

    let width = window.resolution.width();
    let height = window.resolution.height();

    let display_info = format!("{}x{}", width, height);

    text_display
        .with_child((
            DisplayText,
            TextSpan::new(display_info),
            TextFont::from_font_size(16.0),
            TextColor(Color::WHITE),
        ))
        .with_child((
            FocusText,
            TextSpan::new("".to_string()),
            TextFont::from_font_size(16.0),
            TextColor(Color::WHITE),
        ));
    let text_display_info = text_display.id();

    let mut text_system = commands.spawn((
        Text::new("System: "),
        TextFont::from_font_size(16.0),
        TextColor(Color::WHITE),
    ));

    let system_info = format!(
        "{} ({}), cpu: {} (x{}), memory: {}",
        system.os, system.kernel, system.cpu, system.core_count, system.memory
    );

    text_system.with_child((
        TextSpan::new(system_info),
        TextFont::from_font_size(16.0),
        TextColor(Color::WHITE),
    ));

    let text_system_info = text_system.id();

    let mut text_adapter = commands.spawn((
        Text::new("Adapter: "),
        TextFont::from_font_size(16.0),
        TextColor(Color::WHITE),
    ));

    let device_type: String = match adapter.device_type {
        DeviceType::Other | DeviceType::DiscreteGpu => "".to_string(),
        DeviceType::IntegratedGpu => " (integrated)".to_string(),
        DeviceType::VirtualGpu => " (virtual)".to_string(),
        DeviceType::Cpu => " (cpu)".to_string(),
    };

    let adapter_info = format!(
        "{}{}, {} ({})",
        adapter.name, device_type, adapter.driver_info, adapter.backend
    );

    text_adapter.with_child((
        TextSpan::new(adapter_info),
        TextFont::from_font_size(16.0),
        TextColor(Color::WHITE),
    ));

    let text_adapter_info = text_adapter.id();

    commands.entity(hud_root).add_children(&[
        text_fps,
        text_monitor_info,
        text_display_info,
        text_system_info,
        text_adapter_info,
    ]);
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
