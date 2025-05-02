use bevy::diagnostic::SystemInfo;
use bevy::prelude::*;
use bevy::render::renderer::RenderAdapterInfo;
use bevy::window::{Monitor, PrimaryMonitor};
use wgpu_types::DeviceType;

use super::{DisplayText, FocusText, FpsText};
use crate::assets::RuntimeAsset;
use crate::gui::debug::DebugGUIState;
/// Marker to find debug's hud box entity.
#[derive(Component)]
pub struct DebugHudRoot;

/// Setups debug hud in the left-up corner of the screen.
pub fn setup_debug_hud(
    mut commands: Commands,
    debug_state: Res<State<DebugGUIState>>,
    asset_server: Res<AssetServer>,
    runtime_asset: Res<RuntimeAsset>,
    system: Res<SystemInfo>,
    adapter: Res<RenderAdapterInfo>,
    monitor_q: Query<(Entity, &Monitor, Has<PrimaryMonitor>)>,
    window_q: Query<&Window>,
) {
    let text_font = TextFont {
        font: asset_server.load(runtime_asset.font_path.clone()),
        font_size: 16.,
        ..default()
    };

    let vis_state = if *debug_state.get() == DebugGUIState::Opened {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    let hud_root = commands
        .spawn((
            DebugHudRoot,
            (
                BackgroundColor(Color::BLACK.with_alpha(0.5)),
                GlobalZIndex(10),
                vis_state,
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
            text_font.clone(),
            TextColor(Color::WHITE),
        ))
        .with_child((
            FpsText,
            TextSpan::new("N/A"),
            text_font.clone(),
            TextColor(Color::WHITE),
        ))
        .id();

    let mut monitor_text = commands.spawn((
        Text::new("Monitor: "),
        text_font.clone(),
        TextColor(Color::WHITE),
    ));

    for (_, monitor, is_primary) in monitor_q.iter() {
        let mut monitor_info = monitor
            .name
            .clone()
            .unwrap_or_else(|| "undefined".to_owned());
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
            text_font.clone(),
            TextColor(Color::WHITE),
        ));
    }

    let monitor_info_text = monitor_text.id();

    let mut display_text = commands.spawn((
        Text::new("Display: "),
        text_font.clone(),
        TextColor(Color::WHITE),
    ));

    let (width, height): (f32, f32);
    match window_q.single() {
        Ok(window) => {
            width = window.resolution.width();
            height = window.resolution.height();
        },
        Err(_) => {
            warn!("Cannot get access to `PrimaryWindow`.");
            width = 0.;
            height = 0.;
        },
    }

    let display_info = format!("{}x{}", width, height);

    display_text
        .with_child((
            DisplayText,
            TextSpan::new(display_info),
            text_font.clone(),
            TextColor(Color::WHITE),
        ))
        .with_child((
            FocusText,
            TextSpan::new("".to_owned()),
            text_font.clone(),
            TextColor(Color::WHITE),
        ));
    let display_info_text = display_text.id();

    let mut system_text = commands.spawn((
        Text::new("System: "),
        text_font.clone(),
        TextColor(Color::WHITE),
    ));

    let system_info: String;
    if system.os != "Unknown" {
        system_info = format!(
            "{} ({}), cpu: {} (x{}), memory: {}",
            system.os, system.kernel, system.cpu, system.core_count, system.memory
        );
    } else {
        if !crate::consts::DEBUG_MODE {
            warn!("Cannot get access to system information.");
            system_info = system.os.clone();
        } else {
            system_info = "Debug mode".to_owned();
        }
    }

    system_text.with_child((
        TextSpan::new(system_info),
        text_font.clone(),
        TextColor(Color::WHITE),
    ));

    let system_info_text = system_text.id();

    let mut adapter_text = commands.spawn((
        Text::new("Adapter: "),
        text_font.clone(),
        TextColor(Color::WHITE),
    ));

    let device_type: String = match adapter.device_type {
        DeviceType::Other | DeviceType::DiscreteGpu => "".to_owned(),
        DeviceType::IntegratedGpu => " (integrated)".to_owned(),
        DeviceType::VirtualGpu => " (virtual)".to_owned(),
        DeviceType::Cpu => " (cpu)".to_owned(),
    };

    let adapter_info = format!(
        "{}{}, {} ({})",
        adapter.name, device_type, adapter.driver_info, adapter.backend
    );

    adapter_text.with_child((
        TextSpan::new(adapter_info),
        text_font.clone(),
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
    mut hud_root_q: Query<&mut Visibility, With<DebugHudRoot>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::F3) {
        match hud_root_q.single_mut() {
            Ok(mut vis) => {
                *vis = match *vis {
                    Visibility::Hidden => Visibility::Visible,
                    _ => Visibility::Hidden,
                }
            },
            Err(_) => error!("Cannot toggle debug hud: no access to visibility."),
        }
    }
}
