use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::Monitor;
use bevy::window::PrimaryMonitor;

// Marker to find fps text entity
#[derive(Component)]
pub struct FpsText;

pub fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    query_monitor: Query<(Entity, &Monitor, Has<PrimaryMonitor>)>,
    mut query: Query<(&mut TextSpan, &mut TextColor), With<FpsText>>,
) {
    for (mut span, mut color) in query.iter_mut() {
        // Try to get a "smoothed" FPS value from Bevy
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            **span = format!("{value:>3.0}");
	    
	    let monitor = query_monitor.single().1;
	    let hz : f64 = (monitor.refresh_rate_millihertz.unwrap_or(0).div_ceil(10000) * 10) as f64;
            // Adjust text color based on FPS value.
            color.0 = if value >= hz {
                Color::srgb(0.0, 1.0, 0.0)
            } else if value >= hz / 2. {
                Color::srgb((1.0 - (value - hz / 2.) / (hz / 2.)) as f32, 1.0, 0.0)
            } else if value >= hz / 4. {
                Color::srgb(1.0, ((value - hz / 4.) / (hz / 4.)) as f32, 0.0)
            } else {
                Color::srgb(1.0, 0.0, 0.0)
            };
        } else {
            **span = " N/A".into();
            color.0 = Color::WHITE;
        }
    }
}
