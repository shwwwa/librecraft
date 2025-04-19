use bevy::diagnostic::SystemInfo;
use bevy::prelude::*;
use bevy::render::renderer::RenderAdapterInfo;
use bevy::window::{Monitor, PrimaryMonitor};

use wgpu_types::DeviceType;

use super::{DisplayText, FocusText, FpsText};

/** Marker to find debug's hud box entity. */
#[derive(Component)]
pub struct DebugHudRoot;

/** Setups debug hud in the left-up corner of the screen. */
pub fn setup_debug_hud(
    mut commands: Commands,
    system: Res<SystemInfo>,
    adapter: Res<RenderAdapterInfo>,
    query_monitor: Query<(Entity, &Monitor, Has<PrimaryMonitor>)>,
    query_window: Query<&Window>,
) {
    let hud_root = commands
        .spawn((
            DebugHudRoot,
            (
                BackgroundColor(Color::BLACK.with_alpha(0.5)),
                GlobalZIndex(10),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(1.),
                    top: Val::Percent(1.),
                    // Resized depending on text inside.
                    bottom: Val::Auto,
                    right: Val::Auto,
                    padding: UiRect::all(Val::Px(4.0)),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
            ),
        ))
        .id();

    let fps_text = commands
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

    let mut monitor_text = commands.spawn((
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
        monitor_text.with_child((
            TextSpan::new(monitor_info),
            TextFont::from_font_size(16.0),
            TextColor(Color::WHITE),
        ));
    }

    let monitor_info_text = monitor_text.id();

    let mut display_text = commands.spawn((
        Text::new("Display: "),
        TextFont::from_font_size(16.0),
        TextColor(Color::WHITE),
    ));

    let window = query_window.single();

    let width = window.resolution.width();
    let height = window.resolution.height();

    let display_info = format!("{}x{}", width, height);

    display_text
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
    let display_info_text = display_text.id();

    let mut system_text = commands.spawn((
        Text::new("System: "),
        TextFont::from_font_size(16.0),
        TextColor(Color::WHITE),
    ));

    let system_info = format!(
        "{} ({}), cpu: {} (x{}), memory: {}",
        system.os, system.kernel, system.cpu, system.core_count, system.memory
    );

    system_text.with_child((
        TextSpan::new(system_info),
        TextFont::from_font_size(16.0),
        TextColor(Color::WHITE),
    ));

    let system_info_text = system_text.id();

    let mut adapter_text = commands.spawn((
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

    adapter_text.with_child((
        TextSpan::new(adapter_info),
        TextFont::from_font_size(16.0),
        TextColor(Color::WHITE),
    ));

    let adapter_info_text = adapter_text.id();

    commands.entity(hud_root).add_children(&[
        fps_text,
        monitor_info_text,
        display_info_text,
        system_info_text,
        adapter_info_text,
    ]);
}

pub fn toggle_debug_hud(
    mut q_hud_root: Query<&mut Visibility, With<DebugHudRoot>>,
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
